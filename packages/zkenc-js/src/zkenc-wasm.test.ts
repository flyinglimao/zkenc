/**
 * Tests for zkenc WASM integration (encap/decap)
 */

import { describe, it, expect } from "vitest";
import { readFileSync } from "fs";
import { join } from "path";
import { encap, decap } from "./zkenc.js";

describe("zkenc WASM integration", () => {
  const testDir = join(process.cwd(), "tests", "fixtures");

  // Load test fixtures
  const r1csBuffer = new Uint8Array(readFileSync(join(testDir, "sudoku.r1cs")));
  const wasmBuffer = new Uint8Array(readFileSync(join(testDir, "sudoku.wasm")));
  const symContent = readFileSync(join(testDir, "sudoku.sym"), "utf-8");
  const sudokuInput = JSON.parse(
    readFileSync(join(testDir, "sudoku_general.json"), "utf-8")
  );

  it("should perform encap with public inputs", async () => {
    // Only use public inputs (puzzle) for encap
    const result = await encap({ r1csBuffer, symContent }, { puzzle: sudokuInput.puzzle });

    expect(result.ciphertext).toBeInstanceOf(Uint8Array);
    expect(result.ciphertext.length).toBeGreaterThan(0);
    expect(result.key).toBeInstanceOf(Uint8Array);
    expect(result.key.length).toBe(32);
  });

  it("should perform decap with valid witness", async () => {
    // First encap with only public inputs
    const { ciphertext, key: originalKey } = await encap(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle }
    );

    // Then decap with full inputs (needs WASM for witness calculation)
    const recoveredKey = await decap({ r1csBuffer, wasmBuffer }, ciphertext, {
      puzzle: sudokuInput.puzzle,
      solution: sudokuInput.solution,
    });

    expect(recoveredKey).toBeInstanceOf(Uint8Array);
    expect(recoveredKey.length).toBe(32);

    // Keys should match
    expect(Array.from(recoveredKey)).toEqual(Array.from(originalKey));
  });

  it("should produce different ciphertexts for same inputs", async () => {
    // Encap twice with same inputs
    const result1 = await encap(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle }
    );
    const result2 = await encap(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle }
    );

    // Ciphertexts should be different (due to randomness)
    expect(Array.from(result1.ciphertext)).not.toEqual(
      Array.from(result2.ciphertext)
    );

    // But keys should also be different
    expect(Array.from(result1.key)).not.toEqual(Array.from(result2.key));
  });

  it("should work with low-level encap/decap flow", async () => {
    const { aesGcmEncrypt, aesGcmDecrypt } = await import("./crypto.js");

    const message = new TextEncoder().encode(
      "Secret message for Sudoku solver!"
    );

    // 1. Encap to get key
    const { ciphertext: zkCiphertext, key } = await encap(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle }
    );

    // 2. Encrypt message with key using AES
    const encryptedMessage = await aesGcmEncrypt(key, message);

    // 3. Decap to recover key
    const recoveredKey = await decap(
      { r1csBuffer, wasmBuffer },
      zkCiphertext,
      sudokuInput
    );

    // 4. Decrypt message with recovered key
    const decryptedMessage = await aesGcmDecrypt(
      recoveredKey,
      encryptedMessage
    );

    // 5. Verify
    expect(new TextDecoder().decode(decryptedMessage)).toBe(
      "Secret message for Sudoku solver!"
    );
  });

  it("should work with high-level encrypt/decrypt flow", async () => {
    const { encrypt, decrypt } = await import("./zkenc.js");

    const message = new TextEncoder().encode(
      "Secret message for Sudoku solver!"
    );

    // 1. Encrypt: combines encap + AES encryption
    const { ciphertext, key } = await encrypt(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle },
      message
    );

    // Verify the key is returned for advanced users
    expect(key).toBeInstanceOf(Uint8Array);
    expect(key.length).toBe(32);

    // Verify ciphertext contains both witness CT and encrypted message
    expect(ciphertext.length).toBeGreaterThan(1576 + 28); // witness CT + AES overhead

    // 2. Decrypt: combines decap + AES decryption
    const decryptedMessage = await decrypt(
      { r1csBuffer, wasmBuffer },
      ciphertext,
      sudokuInput
    );

    // 3. Verify
    expect(new TextDecoder().decode(decryptedMessage)).toBe(
      "Secret message for Sudoku solver!"
    );
  });

  it("should produce different ciphertexts with encrypt", async () => {
    const { encrypt } = await import("./zkenc.js");

    const message = new TextEncoder().encode("Same message");

    // Encrypt twice with same inputs
    const result1 = await encrypt(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle },
      message
    );
    const result2 = await encrypt(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle },
      message
    );

    // Ciphertexts should be different (due to randomness in encap and AES nonce)
    expect(Array.from(result1.ciphertext)).not.toEqual(
      Array.from(result2.ciphertext)
    );

    // Keys should also be different
    expect(Array.from(result1.key)).not.toEqual(Array.from(result2.key));
  });

  it("should fail to decrypt with invalid witness", async () => {
    const { encrypt, decrypt } = await import("./zkenc.js");

    const message = new TextEncoder().encode("Secret");

    // Encrypt with correct puzzle
    const { ciphertext } = await encrypt(
      { r1csBuffer, symContent },
      { puzzle: sudokuInput.puzzle },
      message
    );

    // Try to decrypt with invalid solution
    const invalidSolution = sudokuInput.solution.map((n: number) =>
      n === 0 ? 0 : (n % 9) + 1
    );

    await expect(
      decrypt({ r1csBuffer, symContent }, ciphertext, {
        puzzle: sudokuInput.puzzle,
        solution: invalidSolution,
      })
    ).rejects.toThrow();
  });
});
