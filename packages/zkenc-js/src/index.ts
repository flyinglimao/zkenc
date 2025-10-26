/**
 * zkenc-js - Witness encryption for JavaScript/TypeScript
 *
 * @packageDocumentation
 */

export { encap, decap, encrypt, decrypt, getPublicInput } from "./zkenc.js";
export type {
  EncapResult,
  EncryptResult,
  CircuitFiles,
  CircuitFilesForEncap,
  EncryptOptions,
} from "./zkenc.js";
export {
  parseSymFile,
  getInputSignals,
  mapInputsToWires,
} from "./sym_parser.js";
export type { SymbolEntry } from "./sym_parser.js";
