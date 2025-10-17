import { describe, it, expect } from "vitest";
import { encrypt, decrypt } from "./zkenc";

describe("zkenc-js", () => {
  // encap/decap tests are in zkenc-wasm.test.ts

  describe("encrypt/decrypt", () => {
    it("should encrypt and decrypt message successfully", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode(
        "Secret message for Sudoku solver"
      );

      const encrypted = await encrypt(key, message);
      const decrypted = await decrypt(key, encrypted);

      expect(decrypted).toEqual(message);
      expect(new TextDecoder().decode(decrypted)).toBe(
        "Secret message for Sudoku solver"
      );
    });

    it("should fail to decrypt with wrong key", async () => {
      const key1 = crypto.getRandomValues(new Uint8Array(32));
      const key2 = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode("secret");

      const encrypted = await encrypt(key1, message);

      await expect(decrypt(key2, encrypted)).rejects.toThrow();
    });

    it("should produce different ciphertexts for same message", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode("test");

      const encrypted1 = await encrypt(key, message);
      const encrypted2 = await encrypt(key, message);

      // Should be different due to random nonce
      expect(encrypted1).not.toEqual(encrypted2);

      // But both should decrypt to same message
      expect(await decrypt(key, encrypted1)).toEqual(message);
      expect(await decrypt(key, encrypted2)).toEqual(message);
    });
  });
});
