import { describe, it, expect } from "vitest";
import { encap, decap } from "./zkenc";
import { aesGcmEncrypt, aesGcmDecrypt } from "./crypto";
import { readFileSync } from "fs";
import { join } from "path";

const FIXTURES_DIR = join(__dirname, "../tests/fixtures");

describe("E2E Tests - Sudoku Witness Encryption", () => {
  // Load circuit files
  const r1csPath = join(FIXTURES_DIR, "sudoku.r1cs");
  const wasmPath = join(FIXTURES_DIR, "sudoku.wasm");
  const r1csBytes = new Uint8Array(readFileSync(r1csPath));
  const wasmBytes = new Uint8Array(readFileSync(wasmPath));

  // Load test data from fixture - use sudoku_general with incomplete puzzle
  const inputPath = join(FIXTURES_DIR, "sudoku_general.json");
  const testData = JSON.parse(readFileSync(inputPath, "utf-8"));

  const validPuzzle = testData.puzzle;
  const validSolution = testData.solution;

  const invalidSolution = new Array(81).fill(1); // All ones - invalid

  const plaintext =
    "This is a secret message encrypted with witness encryption!";
  const plaintextBytes = new TextEncoder().encode(plaintext);

  it("should complete full encrypt/decrypt workflow with valid witness", async () => {
    // Step 1: Encap - generate ciphertext and key using only public inputs (puzzle)
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const { ciphertext, key: encapKey } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    console.log("Encap completed:", {
      ciphertextLength: ciphertext.length,
      keyLength: encapKey.length,
    });

    // Step 2: Encrypt message with encap key
    const encrypted = await aesGcmEncrypt(encapKey, plaintextBytes);
    console.log("Encrypted data length:", encrypted.length);

    // Step 3: Decap - recover key using complete witness (puzzle + solution)
    const fullInputs = { puzzle: validPuzzle, solution: validSolution };
    const decapKey = await decap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      ciphertext,
      fullInputs
    );

    console.log("Decap completed, key length:", decapKey.length);

    // Step 4: Decrypt message with decap key
    const decrypted = await aesGcmDecrypt(decapKey, encrypted);
    const decryptedText = new TextDecoder().decode(decrypted);
    console.log("Decrypted:", decryptedText);

    // Verify the workflow
    expect(decryptedText).toBe(plaintext);
    expect(encapKey).toEqual(decapKey); // Keys should match
  });

  it("should reject decap with invalid witness", async () => {
    // Generate ciphertext with valid puzzle
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const { ciphertext } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    // Try to decap with invalid solution
    const invalidInputs = { puzzle: validPuzzle, solution: invalidSolution };

    await expect(async () => {
      await decap(
        { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
        ciphertext,
        invalidInputs
      );
    }).rejects.toThrow(); // Should throw because constraint system is not satisfied
  });

  it("should produce different ciphertexts for different puzzles", async () => {
    const puzzle1 = validPuzzle;
    const puzzle2 = [...validPuzzle]; // Copy
    puzzle2[0] = 9; // Change one cell

    const { ciphertext: ct1 } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      { puzzle: puzzle1, solution: validSolution }
    );

    const { ciphertext: ct2 } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      { puzzle: puzzle2, solution: validSolution }
    );

    // Ciphertexts should be different for different public inputs
    expect(ct1).not.toEqual(ct2);
  });

  it("should produce different ciphertexts with same inputs (randomness)", async () => {
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };

    const { ciphertext: ct1 } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    const { ciphertext: ct2 } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    // Different random values should produce different ciphertexts
    expect(ct1).not.toEqual(ct2);
  });

  it("should handle large plaintext data", async () => {
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const fullInputs = { puzzle: validPuzzle, solution: validSolution };

    // Generate large plaintext (10KB)
    const largePlaintext = new TextEncoder().encode("A".repeat(10 * 1024));

    // Encap and encrypt
    const { ciphertext, key: encapKey } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );
    const encrypted = await aesGcmEncrypt(encapKey, largePlaintext);

    // Decap and decrypt
    const decapKey = await decap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      ciphertext,
      fullInputs
    );
    const decrypted = await aesGcmDecrypt(decapKey, encrypted);

    expect(decrypted).toEqual(largePlaintext);
    expect(decrypted.length).toBe(10 * 1024);
  });

  it("should reject decap with wrong ciphertext", async () => {
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const fullInputs = { puzzle: validPuzzle, solution: validSolution };

    // Generate valid ciphertext
    const { ciphertext } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    // Create a completely invalid ciphertext (too short)
    const corruptedCiphertext = new Uint8Array(10);

    // Try to decap with corrupted ciphertext
    await expect(async () => {
      await decap(
        { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
        corruptedCiphertext,
        fullInputs
      );
    }).rejects.toThrow(); // Should fail deserialization
  });

  it("should reject decap with incomplete inputs", async () => {
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const { ciphertext } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );

    // Try to decap with only puzzle (missing solution)
    const incompleteInputs = { puzzle: validPuzzle };

    await expect(async () => {
      await decap(
        { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
        ciphertext,
        incompleteInputs
      );
    }).rejects.toThrow(); // Should fail witness calculation
  });

  it("should handle binary data encryption", async () => {
    const publicInputs = { puzzle: validPuzzle, solution: validSolution };
    const fullInputs = { puzzle: validPuzzle, solution: validSolution };

    // Create binary data
    const binaryData = new Uint8Array([0, 1, 2, 3, 255, 254, 253]);

    // Encap and encrypt
    const { ciphertext, key: encapKey } = await encap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      publicInputs
    );
    const encrypted = await aesGcmEncrypt(encapKey, binaryData);

    // Decap and decrypt
    const decapKey = await decap(
      { r1csBuffer: r1csBytes, wasmBuffer: wasmBytes },
      ciphertext,
      fullInputs
    );
    const decrypted = await aesGcmDecrypt(decapKey, encrypted);

    expect(decrypted).toEqual(binaryData);
  });
});
