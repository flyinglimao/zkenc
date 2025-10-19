/**
 * Witness Calculator for Circom Circuits
 *
 * This file is a TypeScript rewrite of the witness_calculator.js from circom's snarkjs.
 * Original source: https://github.com/iden3/snarkjs/blob/master/src/witness_calculator.js
 *
 * Rewritten to TypeScript for:
 * - Type safety
 * - Better IDE support
 * - CommonJS/ESM dual module support
 * - Integration with zkenc-js
 *
 * @license GPL-3.0
 */

interface WitnessCalculatorOptions {
  sanityCheck?: boolean;
}

interface CircuitInput {
  [key: string]: string | number | bigint | (string | number | bigint)[];
}

/**
 * Witness Calculator class for computing witness values from circuit inputs
 */
export class WitnessCalculator {
  private instance: WebAssembly.Instance;
  public readonly n32: number;
  public readonly prime: bigint;
  public readonly witnessSize: number;
  private sanityCheck: boolean;
  public readonly version: number;

  constructor(instance: WebAssembly.Instance, sanityCheck?: boolean) {
    this.instance = instance;
    this.sanityCheck = sanityCheck || false;

    const exports = this.instance.exports as any;

    this.version = exports.getVersion();
    this.n32 = exports.getFieldNumLen32();

    // Get prime number
    exports.getRawPrime();
    const arr = new Uint32Array(this.n32);
    for (let i = 0; i < this.n32; i++) {
      arr[this.n32 - 1 - i] = exports.readSharedRWMemory(i);
    }
    this.prime = fromArray32(arr);

    this.witnessSize = exports.getWitnessSize();
  }

  circom_version(): number {
    return this.version;
  }

  private async _doCalculateWitness(
    input: CircuitInput,
    sanityCheck?: boolean
  ): Promise<void> {
    const exports = this.instance.exports as any;
    exports.init(this.sanityCheck || sanityCheck ? 1 : 0);

    const keys = Object.keys(input);
    let input_counter = 0;

    for (const k of keys) {
      const h = fnvHash(k);
      const hMSB = parseInt(h.slice(0, 8), 16);
      const hLSB = parseInt(h.slice(8, 16), 16);
      const fArr = flatArray(input[k]);

      const signalSize = exports.getInputSignalSize(hMSB, hLSB);
      if (signalSize < 0) {
        throw new Error(`Signal ${k} not found\n`);
      }
      if (fArr.length < signalSize) {
        throw new Error(`Not enough values for input signal ${k}\n`);
      }
      if (fArr.length > signalSize) {
        throw new Error(`Too many values for input signal ${k}\n`);
      }

      for (let i = 0; i < fArr.length; i++) {
        const arrFr = toArray32(normalize(fArr[i], this.prime), this.n32);
        for (let j = 0; j < this.n32; j++) {
          exports.writeSharedRWMemory(j, arrFr[this.n32 - 1 - j]);
        }
        try {
          exports.setInputSignal(hMSB, hLSB, i);
          input_counter++;
        } catch (err) {
          throw new Error(`Error setting signal ${k}[${i}]: ${err}`);
        }
      }
    }

    const expectedInputs = exports.getInputSize();
    if (input_counter < expectedInputs) {
      throw new Error(
        `Not all inputs have been set. Only ${input_counter} out of ${expectedInputs}`
      );
    }
  }

  async calculateWitness(
    input: CircuitInput,
    sanityCheck?: boolean
  ): Promise<bigint[]> {
    const w: bigint[] = [];
    const exports = this.instance.exports as any;

    await this._doCalculateWitness(input, sanityCheck);

    for (let i = 0; i < this.witnessSize; i++) {
      exports.getWitness(i);
      const arr = new Uint32Array(this.n32);
      for (let j = 0; j < this.n32; j++) {
        arr[this.n32 - 1 - j] = exports.readSharedRWMemory(j);
      }
      w.push(fromArray32(arr));
    }

    return w;
  }

  async calculateBinWitness(
    input: CircuitInput,
    sanityCheck?: boolean
  ): Promise<Uint8Array> {
    const exports = this.instance.exports as any;
    const buff32 = new Uint32Array(this.witnessSize * this.n32);
    const buff = new Uint8Array(buff32.buffer);

    await this._doCalculateWitness(input, sanityCheck);

    for (let i = 0; i < this.witnessSize; i++) {
      exports.getWitness(i);
      const pos = i * this.n32;
      for (let j = 0; j < this.n32; j++) {
        buff32[pos + j] = exports.readSharedRWMemory(j);
      }
    }

    return buff;
  }

  async calculateWTNSBin(
    input: CircuitInput,
    sanityCheck?: boolean
  ): Promise<Uint8Array> {
    const exports = this.instance.exports as any;
    const buff32 = new Uint32Array(this.witnessSize * this.n32 + this.n32 + 11);
    const buff = new Uint8Array(buff32.buffer);

    await this._doCalculateWitness(input, sanityCheck);

    // "wtns" magic header
    buff[0] = "w".charCodeAt(0);
    buff[1] = "t".charCodeAt(0);
    buff[2] = "n".charCodeAt(0);
    buff[3] = "s".charCodeAt(0);

    // version 2
    buff32[1] = 2;

    // number of sections: 2
    buff32[2] = 2;

    // Section 1: Header
    buff32[3] = 1; // section id

    const n8 = this.n32 * 4;
    const idSection1length = 8 + n8;
    const idSection1lengthHex = idSection1length.toString(16).padStart(16, "0");
    buff32[4] = parseInt(idSection1lengthHex.slice(0, 8), 16);
    buff32[5] = parseInt(idSection1lengthHex.slice(8, 16), 16);

    // field size
    buff32[6] = n8;

    // prime number
    exports.getRawPrime();
    let pos = 7;
    for (let j = 0; j < this.n32; j++) {
      buff32[pos + j] = exports.readSharedRWMemory(j);
    }
    pos += this.n32;

    // witness size
    buff32[pos] = this.witnessSize;
    pos++;

    // Section 2: Witness data
    buff32[pos] = 2; // section id
    pos++;

    const idSection2length = n8 * this.witnessSize;
    const idSection2lengthHex = idSection2length.toString(16).padStart(16, "0");
    buff32[pos] = parseInt(idSection2lengthHex.slice(0, 8), 16);
    buff32[pos + 1] = parseInt(idSection2lengthHex.slice(8, 16), 16);
    pos += 2;

    // Write witness values
    for (let i = 0; i < this.witnessSize; i++) {
      exports.getWitness(i);
      for (let j = 0; j < this.n32; j++) {
        buff32[pos + j] = exports.readSharedRWMemory(j);
      }
      pos += this.n32;
    }

    return buff;
  }
}

/**
 * Build a WitnessCalculator from compiled WASM code
 * @param wasmCode - Compiled Circom WASM code
 * @param options - Optional configuration
 * @returns WitnessCalculator instance
 */
export default async function witnessCalculatorBuilder(
  wasmCode: BufferSource,
  options?: WitnessCalculatorOptions
): Promise<WitnessCalculator> {
  options = options || {};

  let wasmModule: WebAssembly.Module;
  try {
    wasmModule = await WebAssembly.compile(wasmCode);
  } catch (err) {
    console.error(err);
    console.error("\nTry to run circom --c to generate c++ code instead\n");
    throw new Error(`WASM compilation failed: ${err}`);
  }

  let errStr = "";
  let msgStr = "";

  function getMessage(exports: any): string {
    let message = "";
    let c = exports.getMessageChar();
    while (c !== 0) {
      message += String.fromCharCode(c);
      c = exports.getMessageChar();
    }
    return message;
  }

  function printSharedRWMemory(exports: any, n32: number): void {
    const shared_rw_memory_size = exports.getFieldNumLen32();
    const arr = new Uint32Array(shared_rw_memory_size);
    for (let j = 0; j < shared_rw_memory_size; j++) {
      arr[shared_rw_memory_size - 1 - j] = exports.readSharedRWMemory(j);
    }

    if (msgStr !== "") {
      msgStr += " ";
    }
    msgStr += fromArray32(arr).toString();
  }

  const instance = await WebAssembly.instantiate(wasmModule, {
    runtime: {
      exceptionHandler(code: number): void {
        let err: string;
        if (code === 1) {
          err = "Signal not found.\n";
        } else if (code === 2) {
          err = "Too many signals set.\n";
        } else if (code === 3) {
          err = "Signal already set.\n";
        } else if (code === 4) {
          err = "Assert Failed.\n";
        } else if (code === 5) {
          err = "Not enough memory.\n";
        } else if (code === 6) {
          err = "Input signal array access exceeds the size.\n";
        } else {
          err = "Unknown error.\n";
        }
        throw new Error(err + errStr);
      },
      printErrorMessage(): void {
        errStr += getMessage(instance.exports) + "\n";
      },
      writeBufferMessage(): void {
        const msg = getMessage(instance.exports);
        if (msg === "\n") {
          console.log(msgStr);
          msgStr = "";
        } else {
          if (msgStr !== "") {
            msgStr += " ";
          }
          msgStr += msg;
        }
      },
      showSharedRWMemory(): void {
        const exports = instance.exports as any;
        printSharedRWMemory(exports, exports.getFieldNumLen32());
      },
    },
  });

  return new WitnessCalculator(instance, options.sanityCheck);
}

// Helper functions

function toArray32(rem: bigint, size?: number): number[] {
  const res: number[] = [];
  const radix = BigInt(0x100000000);
  while (rem) {
    res.unshift(Number(rem % radix));
    rem = rem / radix;
  }
  if (size) {
    let i = size - res.length;
    while (i > 0) {
      res.unshift(0);
      i--;
    }
  }
  return res;
}

function fromArray32(arr: Uint32Array | number[]): bigint {
  let res = BigInt(0);
  const radix = BigInt(0x100000000);
  for (let i = 0; i < arr.length; i++) {
    res = res * radix + BigInt(arr[i]);
  }
  return res;
}

function flatArray(a: any): bigint[] {
  const res: bigint[] = [];
  fillArray(res, a);
  return res;

  function fillArray(res: bigint[], a: any): void {
    if (Array.isArray(a)) {
      for (let i = 0; i < a.length; i++) {
        fillArray(res, a[i]);
      }
    } else {
      res.push(BigInt(a));
    }
  }
}

function normalize(n: string | number | bigint, prime: bigint): bigint {
  let res = BigInt(n) % prime;
  if (res < 0) res += prime;
  return res;
}

function fnvHash(str: string): string {
  const uint64_max = BigInt(2) ** BigInt(64);
  let hash = BigInt("0xCBF29CE484222325");
  for (let i = 0; i < str.length; i++) {
    hash ^= BigInt(str.charCodeAt(i));
    hash *= BigInt(0x100000001b3);
    hash %= uint64_max;
  }
  let shash = hash.toString(16);
  const n = 16 - shash.length;
  shash = "0".repeat(n) + shash;
  return shash;
}
