pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/comparators.circom";
include "../node_modules/circomlib/circuits/poseidon.circom";
include "../node_modules/circomlib/circuits/eddsaposeidon.circom";

// Helper template to select from 3 public keys using 2-bit index
template SelectPublicKey() {
    signal input publicKeys[3][2];
    signal input index;  // 0, 1, or 2
    signal output Ax;
    signal output Ay;
    
    component isIndex0 = IsEqual();
    isIndex0.in[0] <== index;
    isIndex0.in[1] <== 0;
    
    component isIndex1 = IsEqual();
    isIndex1.in[0] <== index;
    isIndex1.in[1] <== 1;
    
    component isIndex2 = IsEqual();
    isIndex2.in[0] <== index;
    isIndex2.in[1] <== 2;
    
    // Use intermediate signals for quadratic constraints
    signal term0x;
    signal term1x;
    signal term2x;
    term0x <== isIndex0.out * publicKeys[0][0];
    term1x <== isIndex1.out * publicKeys[1][0];
    term2x <== isIndex2.out * publicKeys[2][0];
    Ax <== term0x + term1x + term2x;
    
    signal term0y;
    signal term1y;
    signal term2y;
    term0y <== isIndex0.out * publicKeys[0][1];
    term1y <== isIndex1.out * publicKeys[1][1];
    term2y <== isIndex2.out * publicKeys[2][1];
    Ay <== term0y + term1y + term2y;
}

// 2 of 3 Multisig Signature Verification
// Public inputs: message, publicKeys[3] (array of 3 public keys)
// Private inputs: signatures (2 signatures), signerIndices (indices of signers 0-2)
// Threshold: 2 signatures required
template MultiSig2of3() {
    // Public inputs
    signal input message;              // The message being signed
    signal input publicKeys[3][2];     // 3 public keys (each has 2 coordinates)
    
    // Private inputs
    signal input R8[2][2];             // 2 signature R8 points
    signal input S[2];                 // 2 signature S values
    signal input signerIndices[2];     // Indices (0, 1, or 2) indicating which public keys signed
    
    // Verify that signerIndices are in range [0, 2]
    component rangeCheck0 = LessEqThan(2);
    rangeCheck0.in[0] <== signerIndices[0];
    rangeCheck0.in[1] <== 2;
    rangeCheck0.out === 1;
    
    component rangeCheck1 = LessEqThan(2);
    rangeCheck1.in[0] <== signerIndices[1];
    rangeCheck1.in[1] <== 2;
    rangeCheck1.out === 1;
    
    // Verify that signerIndices are different (no duplicate signers)
    component isDifferent = IsZero();
    isDifferent.in <== signerIndices[0] - signerIndices[1];
    isDifferent.out === 0;  // Must be different (not zero)
    
    // Select public keys using indices
    component selectKey0 = SelectPublicKey();
    selectKey0.publicKeys <== publicKeys;
    selectKey0.index <== signerIndices[0];
    
    component selectKey1 = SelectPublicKey();
    selectKey1.publicKeys <== publicKeys;
    selectKey1.index <== signerIndices[1];
    
    // Verify first signature
    component verifyEdDSA0 = EdDSAPoseidonVerifier();
    verifyEdDSA0.enabled <== 1;
    verifyEdDSA0.Ax <== selectKey0.Ax;
    verifyEdDSA0.Ay <== selectKey0.Ay;
    verifyEdDSA0.R8x <== R8[0][0];
    verifyEdDSA0.R8y <== R8[0][1];
    verifyEdDSA0.S <== S[0];
    verifyEdDSA0.M <== message;
    
    // Verify second signature
    component verifyEdDSA1 = EdDSAPoseidonVerifier();
    verifyEdDSA1.enabled <== 1;
    verifyEdDSA1.Ax <== selectKey1.Ax;
    verifyEdDSA1.Ay <== selectKey1.Ay;
    verifyEdDSA1.R8x <== R8[1][0];
    verifyEdDSA1.R8y <== R8[1][1];
    verifyEdDSA1.S <== S[1];
    verifyEdDSA1.M <== message;
}

component main {public [message, publicKeys]} = MultiSig2of3();
