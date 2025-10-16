/**
 * TypeScript wrapper for witness_calculator.js
 * Handles witness calculation from Circom WASM circuits
 */

// @ts-ignore - JavaScript module without types
import witnessCalculatorBuilder from './witness_calculator.js';

/**
 * Calculate witness from Circom WASM circuit
 *
 * @param wasmBuffer - Compiled Circom WASM file
 * @param inputs - Circuit inputs as JavaScript object
 * @returns Witness as Uint8Array (serialized in snarkjs wtns format)
 *
 * @example
 * ```typescript
 * const witness = await calculateWitness(wasmBuffer, {
 *   puzzle: [5, 3, 0, ...],
 *   solution: [5, 3, 4, ...]
 * });
 * ```
 */
export async function calculateWitness(
  wasmBuffer: Uint8Array,
  inputs: Record<string, any>
): Promise<Uint8Array> {
  // Build witness calculator from WASM
  const witnessCalculator = await witnessCalculatorBuilder(wasmBuffer);

  // Calculate witness in binary format (snarkjs wtns format)
  // This includes header: magic "wtns", version, sections with prime and witness values
  const witnessBuffer = await witnessCalculator.calculateWTNSBin(inputs, false);

  return new Uint8Array(witnessBuffer);
}

/**
 * Calculate witness and return as BigInt array (for debugging/inspection)
 *
 * @param wasmBuffer - Compiled Circom WASM file
 * @param inputs - Circuit inputs as JavaScript object
 * @returns Array of witness values as BigInt
 */
export async function calculateWitnessArray(
  wasmBuffer: Uint8Array,
  inputs: Record<string, any>
): Promise<bigint[]> {
  const witnessCalculator = await witnessCalculatorBuilder(wasmBuffer);
  return await witnessCalculator.calculateWitness(inputs, false);
}

/**
 * Get circuit information from WASM
 *
 * @param wasmBuffer - Compiled Circom WASM file
 * @returns Circuit metadata
 */
export async function getCircuitInfo(wasmBuffer: Uint8Array): Promise<{
  version: number;
  n32: number;
  prime: bigint;
  witnessSize: number;
}> {
  const witnessCalculator = await witnessCalculatorBuilder(wasmBuffer);

  return {
    version: witnessCalculator.version,
    n32: witnessCalculator.n32,
    prime: witnessCalculator.prime,
    witnessSize: witnessCalculator.witnessSize,
  };
}
