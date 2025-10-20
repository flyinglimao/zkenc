/**
 * zkenc-js - TypeScript bindings for zkenc witness encryption
 *
 * This module provides functions for witness encryption using Circom circuits.
 */

import { wasm_encap, wasm_decap, init as wasmInit } from "../pkg/zkenc_js.js";
import { calculateWitness } from "./witness.js";

// Initialize WASM module
let wasmInitialized = false;
let wasmInitPromise: Promise<void> | null = null;

async function ensureWasmInit() {
  if (!wasmInitialized) {
    if (!wasmInitPromise) {
      wasmInitPromise = (async () => {
        await wasmInit();
        wasmInitialized = true;
      })();
    }
    await wasmInitPromise;
  }
}

/**
 * Encapsulation result containing ciphertext and encryption key
 */
export interface EncapResult {
  /** Ciphertext that can be decrypted with valid witness */
  ciphertext: Uint8Array;
  /** Symmetric encryption key (32 bytes) */
  key: Uint8Array;
}

/**
 * Circuit files required for witness encryption
 */
export interface CircuitFiles {
  /** R1CS circuit file (.r1cs) */
  r1csBuffer: Uint8Array;
  /** Circom WASM file (.wasm) for witness calculation */
  wasmBuffer: Uint8Array;
}

/**
 * Generate ciphertext and encryption key from circuit and public inputs
 *
 * @param circuitFiles - R1CS and WASM files
 * @param publicInputs - Public inputs as JSON object
 * @returns Ciphertext and encryption key
 *
 * @example
 * ```typescript
 * const { ciphertext, key } = await encap({
 *   r1csBuffer: r1csData,
 *   wasmBuffer: wasmData
 * }, { puzzle: [5,3,0,...] });
 * ```
 */
export async function encap(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>
): Promise<EncapResult> {
  await ensureWasmInit();

  // Convert public inputs to JSON string
  const publicInputsJson = JSON.stringify(publicInputs);

  // Call WASM encap function
  const result = wasm_encap(circuitFiles.r1csBuffer, publicInputsJson);

  return {
    ciphertext: result.ciphertext,
    key: result.key,
  };
}

/**
 * Recover encryption key from ciphertext using valid witness
 *
 * @param circuitFiles - R1CS and WASM files
 * @param ciphertext - Ciphertext from encap
 * @param inputs - Full inputs (public + witness) as JSON object
 * @returns Recovered encryption key
 *
 * @example
 * ```typescript
 * const key = await decap({
 *   r1csBuffer: r1csData,
 *   wasmBuffer: wasmData
 * }, ciphertext, {
 *   puzzle: [5,3,0,...],
 *   solution: [5,3,4,...]
 * });
 * ```
 */
export async function decap(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array> {
  await ensureWasmInit();

  // Calculate witness from inputs using Circom WASM
  const witnessBytes = await calculateWitness(
    circuitFiles.wasmBuffer as BufferSource,
    inputs
  );

  // Call WASM decap function
  const key = wasm_decap(circuitFiles.r1csBuffer, witnessBytes, ciphertext);

  return key;
}

/**
 * Encryption result containing witness-encrypted ciphertext
 */
export interface EncryptResult {
  /** Combined ciphertext containing both witness encryption and AES-encrypted message */
  ciphertext: Uint8Array;
  /** The encryption key (for advanced users who want direct key access) */
  key: Uint8Array;
}

/**
 * Options for encryption
 */
export interface EncryptOptions {
  /**
   * Whether to include public inputs in the ciphertext
   * @default true
   */
  includePublicInput?: boolean;
}

/**
 * Encrypt message using witness encryption
 *
 * This is a high-level API that combines encap and AES encryption.
 * It generates a witness-encrypted key and uses it to encrypt the message.
 *
 * @param circuitFiles - R1CS and WASM files
 * @param publicInputs - Public inputs as JSON object
 * @param message - Message to encrypt
 * @param options - Encryption options
 * @returns Combined ciphertext and the encryption key
 *
 * @example
 * ```typescript
 * const { ciphertext, key } = await encrypt({
 *   r1csBuffer: r1csData,
 *   wasmBuffer: wasmData
 * }, { puzzle: [5,3,0,...] }, new TextEncoder().encode('secret'));
 * ```
 */
export async function encrypt(
  circuitFiles: CircuitFiles,
  publicInputs: Record<string, any>,
  message: Uint8Array,
  options: EncryptOptions = {}
): Promise<EncryptResult> {
  const { includePublicInput = true } = options;

  // Step 1: Generate witness-encrypted key
  const { ciphertext: witnessCiphertext, key } = await encap(
    circuitFiles,
    publicInputs
  );

  // Step 2: Encrypt message with the key using AES-256-GCM
  const { aesGcmEncrypt } = await import("./crypto.js");
  const encryptedMessage = await aesGcmEncrypt(key, message);

  // Step 3: Prepare public input data (if includePublicInput is true)
  let publicInputBytes = new Uint8Array(0);
  if (includePublicInput) {
    const publicInputJson = JSON.stringify(publicInputs);
    const encoder = new TextEncoder();
    publicInputBytes = encoder.encode(publicInputJson);
  }

  // Step 4: Combine all parts
  // Format: [1 byte flag][4 bytes witness ct length][witness ciphertext]
  //         [4 bytes public input length (if flag=1)][public input (if flag=1)]
  //         [encrypted message]
  const flag = includePublicInput ? 1 : 0;
  const headerSize = includePublicInput ? 9 : 5; // flag(1) + witnessLen(4) + publicLen(4 if included)

  const totalSize =
    headerSize +
    witnessCiphertext.length +
    publicInputBytes.length +
    encryptedMessage.length;

  const combinedCiphertext = new Uint8Array(totalSize);
  const view = new DataView(combinedCiphertext.buffer);

  let offset = 0;

  // Write flag
  view.setUint8(offset, flag);
  offset += 1;

  // Write witness ciphertext length
  view.setUint32(offset, witnessCiphertext.length, false);
  offset += 4;

  // Write witness ciphertext
  combinedCiphertext.set(witnessCiphertext, offset);
  offset += witnessCiphertext.length;

  // Write public input (if included)
  if (includePublicInput) {
    view.setUint32(offset, publicInputBytes.length, false);
    offset += 4;
    combinedCiphertext.set(publicInputBytes, offset);
    offset += publicInputBytes.length;
  }

  // Write encrypted message
  combinedCiphertext.set(encryptedMessage, offset);

  return {
    ciphertext: combinedCiphertext,
    key,
  };
}

/**
 * Decrypt message using witness decryption
 *
 * This is a high-level API that combines decap and AES decryption.
 * It recovers the key from witness and uses it to decrypt the message.
 *
 * @param circuitFiles - R1CS and WASM files
 * @param ciphertext - Combined ciphertext from encrypt
 * @param inputs - Full inputs (public + witness) as JSON object
 * @returns Decrypted message
 *
 * @example
 * ```typescript
 * const decrypted = await decrypt({
 *   r1csBuffer: r1csData,
 *   wasmBuffer: wasmData
 * }, ciphertext, {
 *   puzzle: [5,3,0,...],
 *   solution: [5,3,4,...]
 * });
 * const message = new TextDecoder().decode(decrypted);
 * ```
 */
export async function decrypt(
  circuitFiles: CircuitFiles,
  ciphertext: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array> {
  // Step 1: Parse combined ciphertext
  if (ciphertext.length < 5) {
    throw new Error("Invalid ciphertext: too short");
  }

  const view = new DataView(ciphertext.buffer, ciphertext.byteOffset);
  let offset = 0;

  // Read flag
  const flag = view.getUint8(offset);
  offset += 1;

  // Read witness ciphertext length
  const witnessCtLength = view.getUint32(offset, false);
  offset += 4;

  if (ciphertext.length < offset + witnessCtLength) {
    throw new Error("Invalid ciphertext: length mismatch");
  }

  // Extract witness ciphertext
  const witnessCiphertext = ciphertext.slice(offset, offset + witnessCtLength);
  offset += witnessCtLength;

  // Skip public input if present (flag === 1)
  if (flag === 1) {
    if (ciphertext.length < offset + 4) {
      throw new Error("Invalid ciphertext: missing public input length");
    }
    const publicInputLength = view.getUint32(offset, false);
    offset += 4;
    // Skip the public input data
    offset += publicInputLength;
  }

  // Extract encrypted message
  const encryptedMessage = ciphertext.slice(offset);

  // Step 2: Recover key using witness decap
  const key = await decap(circuitFiles, witnessCiphertext, inputs);

  // Step 3: Decrypt message with recovered key
  const { aesGcmDecrypt } = await import("./crypto.js");
  const message = await aesGcmDecrypt(key, encryptedMessage);

  return message;
}

/**
 * Extract public inputs from ciphertext
 *
 * Retrieves the public inputs that were embedded in the ciphertext during encryption.
 * This only works if the ciphertext was created with `includePublicInput: true`.
 *
 * @param ciphertext - Combined ciphertext from encrypt
 * @returns Public inputs as JSON object
 * @throws Error if public inputs were not included in the ciphertext
 *
 * @example
 * ```typescript
 * const publicInputs = getPublicInput(ciphertext);
 * console.log(publicInputs.puzzle); // [5,3,0,...]
 * ```
 */
export function getPublicInput(ciphertext: Uint8Array): Record<string, any> {
  // Step 1: Validate ciphertext
  if (ciphertext.length < 5) {
    throw new Error("Invalid ciphertext: too short");
  }

  const view = new DataView(ciphertext.buffer, ciphertext.byteOffset);
  let offset = 0;

  // Read flag
  const flag = view.getUint8(offset);
  offset += 1;

  if (flag !== 1) {
    throw new Error(
      "Public inputs are not included in this ciphertext. " +
        "Use encrypt() with includePublicInput: true to embed public inputs."
    );
  }

  // Read witness ciphertext length
  const witnessCtLength = view.getUint32(offset, false);
  offset += 4;

  if (ciphertext.length < offset + witnessCtLength) {
    throw new Error("Invalid ciphertext: length mismatch");
  }

  // Skip witness ciphertext
  offset += witnessCtLength;

  // Read public input length
  if (ciphertext.length < offset + 4) {
    throw new Error("Invalid ciphertext: missing public input length");
  }

  const publicInputLength = view.getUint32(offset, false);
  offset += 4;

  if (ciphertext.length < offset + publicInputLength) {
    throw new Error("Invalid ciphertext: public input length mismatch");
  }

  // Extract and decode public input
  const publicInputBytes = ciphertext.slice(offset, offset + publicInputLength);
  const decoder = new TextDecoder();
  const publicInputJson = decoder.decode(publicInputBytes);

  try {
    return JSON.parse(publicInputJson);
  } catch (error) {
    throw new Error(
      "Failed to parse public inputs: " + (error as Error).message
    );
  }
}
