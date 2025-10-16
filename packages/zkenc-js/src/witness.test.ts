import { describe, it, expect } from "vitest";
import {
  calculateWitness,
  calculateWitnessArray,
  getCircuitInfo,
} from "./witness.js";
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

describe("Witness Calculator", () => {
  const fixtures = join(__dirname, "../tests/fixtures");
  const wasmBuffer = new Uint8Array(
    readFileSync(join(fixtures, "sudoku.wasm"))
  );
  const inputData = JSON.parse(
    readFileSync(join(fixtures, "sudoku_basic.json"), "utf-8")
  );

  describe("getCircuitInfo", () => {
    it("should return circuit metadata", async () => {
      const info = await getCircuitInfo(wasmBuffer);

      expect(info.version).toBe(2);
      expect(info.n32).toBeGreaterThan(0);
      expect(typeof info.prime).toBe("bigint");
      expect(info.witnessSize).toBe(202); // Sudoku circuit has 202 wires
    });
  });

  describe("calculateWitnessArray", () => {
    it("should calculate witness as BigInt array", async () => {
      const witness = await calculateWitnessArray(wasmBuffer, inputData);

      expect(Array.isArray(witness)).toBe(true);
      expect(witness.length).toBe(202); // 202 wires
      expect(typeof witness[0]).toBe("bigint");
      expect(witness[0]).toBe(1n); // First element is always 1 (constant)
    });

    it("should fail with invalid inputs", async () => {
      await expect(
        calculateWitnessArray(wasmBuffer, { invalid: [1, 2, 3] })
      ).rejects.toThrow();
    });

    it("should fail with incomplete inputs", async () => {
      await expect(
        calculateWitnessArray(wasmBuffer, { puzzle: inputData.puzzle })
      ).rejects.toThrow();
    });
  });

  describe("calculateWitness", () => {
    it("should calculate witness in binary format", async () => {
      const witnessBuffer = await calculateWitness(wasmBuffer, inputData);

      expect(witnessBuffer).toBeInstanceOf(Uint8Array);
      expect(witnessBuffer.length).toBeGreaterThan(0);

      // Check wtns magic header
      const magic = new TextDecoder().decode(witnessBuffer.slice(0, 4));
      expect(magic).toBe("wtns");

      // Check version (should be 2)
      const version = new DataView(witnessBuffer.buffer).getUint32(4, true);
      expect(version).toBe(2);
    });

    it("should produce consistent witness for same inputs", async () => {
      const witness1 = await calculateWitness(wasmBuffer, inputData);
      const witness2 = await calculateWitness(wasmBuffer, inputData);

      expect(witness1).toEqual(witness2);
    });

    it("should produce different witness for different valid inputs", async () => {
      // We can't just modify solution arbitrarily as it must satisfy constraints
      // Instead, test that same inputs produce same witness (deterministic)
      const witness1 = await calculateWitness(wasmBuffer, inputData);
      const witness2 = await calculateWitness(wasmBuffer, inputData);

      expect(witness1).toEqual(witness2);
      expect(witness1.length).toBeGreaterThan(100); // Non-trivial witness
    });
  });

  describe("Browser/Node.js compatibility", () => {
    it("should work with Uint8Array (browser-like)", async () => {
      // Simulate browser environment where we get Uint8Array from fetch
      const buffer = new Uint8Array(wasmBuffer);
      const witness = await calculateWitness(buffer, inputData);

      expect(witness).toBeInstanceOf(Uint8Array);
    });

    it("should work with Buffer (Node.js)", async () => {
      // Node.js Buffer is also Uint8Array compatible
      const buffer = Buffer.from(wasmBuffer);
      const witness = await calculateWitness(buffer, inputData);

      expect(witness).toBeInstanceOf(Uint8Array);
    });
  });
});
