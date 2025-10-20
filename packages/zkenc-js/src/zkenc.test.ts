import { describe, it, expect } from "vitest";
import { aesGcmEncrypt, aesGcmDecrypt } from "./crypto";

describe("zkenc-js crypto utilities", () => {
  // High-level encrypt/decrypt and encap/decap tests are in zkenc-wasm.test.ts

  describe("AES-256-GCM encryption (internal)", () => {
    it("should encrypt and decrypt message successfully", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode(
        "Secret message for Sudoku solver"
      );

      const encrypted = await aesGcmEncrypt(key, message);
      const decrypted = await aesGcmDecrypt(key, encrypted);

      expect(decrypted).toEqual(message);
      expect(new TextDecoder().decode(decrypted)).toBe(
        "Secret message for Sudoku solver"
      );
    });

    it("should fail to decrypt with wrong key", async () => {
      const key1 = crypto.getRandomValues(new Uint8Array(32));
      const key2 = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode("secret");

      const encrypted = await aesGcmEncrypt(key1, message);

      await expect(aesGcmDecrypt(key2, encrypted)).rejects.toThrow();
    });

    it("should produce different ciphertexts for same message", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new TextEncoder().encode("test");

      const encrypted1 = await aesGcmEncrypt(key, message);
      const encrypted2 = await aesGcmEncrypt(key, message);

      // Should be different due to random nonce
      expect(encrypted1).not.toEqual(encrypted2);

      // But both should decrypt to same message
      expect(await aesGcmDecrypt(key, encrypted1)).toEqual(message);
      expect(await aesGcmDecrypt(key, encrypted2)).toEqual(message);
    });

    it("should handle empty messages", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new Uint8Array(0);

      const encrypted = await aesGcmEncrypt(key, message);
      const decrypted = await aesGcmDecrypt(key, encrypted);

      expect(decrypted).toEqual(message);
      expect(decrypted.length).toBe(0);
    });

    it("should handle large messages", async () => {
      const key = crypto.getRandomValues(new Uint8Array(32));
      const message = new Uint8Array(1024 * 1024); // 1MB

      // Fill in chunks to avoid QuotaExceededError (crypto.getRandomValues has 65KB limit)
      const chunkSize = 65536;
      for (let i = 0; i < message.length; i += chunkSize) {
        const chunk = message.subarray(
          i,
          Math.min(i + chunkSize, message.length)
        );
        crypto.getRandomValues(chunk);
      }

      const encrypted = await aesGcmEncrypt(key, message);
      const decrypted = await aesGcmDecrypt(key, encrypted);

      expect(decrypted).toEqual(message);
    });
  });
});
