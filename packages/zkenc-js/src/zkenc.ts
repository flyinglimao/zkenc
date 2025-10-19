/**
 * zkenc-js - TypeScript bindings for zkenc witness encryption
 *
 * This module provides functions for witness encryption using Circom circuits.
 */

import { wasm_encap, wasm_decap, init as wasmInit } from "../pkg/zkenc_js.js";
import { calculateWitness } from "./witness.js";

// Initialize WASM module
let wasmInitialized = false;
async function ensureWasmInit() {
  if (!wasmInitialized) {
    wasmInit();
    wasmInitialized = true;
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
 * Encrypt message using witness encryption
 *
 * This is a high-level API that combines encap and AES encryption.
 * It generates a witness-encrypted key and uses it to encrypt the message.
 *
 * @param circuitFiles - R1CS and WASM files
 * @param publicInputs - Public inputs as JSON object
 * @param message - Message to encrypt
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
  message: Uint8Array
): Promise<EncryptResult> {
  // Step 1: Generate witness-encrypted key
  const { ciphertext: witnessCiphertext, key } = await encap(
    circuitFiles,
    publicInputs
  );

  // Step 2: Encrypt message with the key using AES-256-GCM
  const { aesGcmEncrypt } = await import("./crypto.js");
  const encryptedMessage = await aesGcmEncrypt(key, message);

  // Step 3: Combine both ciphertexts
  // Format: [4 bytes length][witness ciphertext][encrypted message]
  const lengthBuffer = new Uint8Array(4);
  new DataView(lengthBuffer.buffer).setUint32(
    0,
    witnessCiphertext.length,
    false
  );

  const combinedCiphertext = new Uint8Array(
    4 + witnessCiphertext.length + encryptedMessage.length
  );
  combinedCiphertext.set(lengthBuffer, 0);
  combinedCiphertext.set(witnessCiphertext, 4);
  combinedCiphertext.set(encryptedMessage, 4 + witnessCiphertext.length);

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
  if (ciphertext.length < 4) {
    throw new Error("Invalid ciphertext: too short");
  }

  const witnessCtLength = new DataView(
    ciphertext.buffer,
    ciphertext.byteOffset
  ).getUint32(0, false);

  if (ciphertext.length < 4 + witnessCtLength) {
    throw new Error("Invalid ciphertext: length mismatch");
  }

  const witnessCiphertext = ciphertext.slice(4, 4 + witnessCtLength);
  const encryptedMessage = ciphertext.slice(4 + witnessCtLength);

  // Step 2: Recover key using witness decap
  const key = await decap(circuitFiles, witnessCiphertext, inputs);

  // Step 3: Decrypt message with recovered key
  const { aesGcmDecrypt } = await import("./crypto.js");
  const message = await aesGcmDecrypt(key, encryptedMessage);

  return message;
}
