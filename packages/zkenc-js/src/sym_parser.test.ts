import { describe, it, expect } from "vitest";
import { parseSymFile, getInputSignals, mapInputsToWires } from "./sym_parser";

describe("Symbol File Parser", () => {
  const sampleSym = `1,1,172,main.root
2,2,172,main.message
3,3,172,main.R8x
4,4,172,main.Ax
5,5,172,main.pathElements[0]
6,6,172,main.pathElements[1]
7,7,172,main.pathIndices[0]
8,-1,169,main.internal.signal`;

  it("should parse sym file correctly", () => {
    const wireMap = parseSymFile(sampleSym);

    expect(wireMap.get("main.root")).toBe(1);
    expect(wireMap.get("main.message")).toBe(2);
    expect(wireMap.get("main.R8x")).toBe(3);
    expect(wireMap.get("main.pathElements[0]")).toBe(5);
    expect(wireMap.get("main.pathElements[1]")).toBe(6);

    // Internal signals with wireId -1 should not be included
    expect(wireMap.has("main.internal.signal")).toBe(false);
  });

  it("should extract input signals", () => {
    const wireMap = parseSymFile(sampleSym);
    const inputSignals = getInputSignals(wireMap);

    expect(inputSignals.get("root")).toBe(1);
    expect(inputSignals.get("message")).toBe(2);
    expect(inputSignals.get("R8x")).toBe(3);
    expect(inputSignals.get("pathElements[0]")).toBe(5);
  });

  it("should map simple inputs to wires", () => {
    const wireMap = new Map([
      ["root", 1],
      ["message", 2],
    ]);

    const inputs = {
      root: "12345",
      message: "67890",
    };

    const wireValues = mapInputsToWires(inputs, wireMap);

    expect(wireValues.get(1)).toBe("12345");
    expect(wireValues.get(2)).toBe("67890");
  });

  it("should map array inputs to wires", () => {
    const wireMap = new Map([
      ["pathElements[0]", 5],
      ["pathElements[1]", 6],
      ["pathElements[2]", 7],
    ]);

    const inputs = {
      pathElements: ["100", "200", "300"],
    };

    const wireValues = mapInputsToWires(inputs, wireMap);

    expect(wireValues.get(5)).toBe("100");
    expect(wireValues.get(6)).toBe("200");
    expect(wireValues.get(7)).toBe("300");
  });

  it("should handle inputs regardless of key order", () => {
    const wireMap = new Map([
      ["root", 1],
      ["message", 2],
    ]);

    // Test with different key orders
    const inputs1 = { root: "123", message: "456" };
    const inputs2 = { message: "456", root: "123" };

    const wireValues1 = mapInputsToWires(inputs1, wireMap);
    const wireValues2 = mapInputsToWires(inputs2, wireMap);

    // Both should produce the same wire mapping
    expect(wireValues1.get(1)).toBe("123");
    expect(wireValues1.get(2)).toBe("456");
    expect(wireValues2.get(1)).toBe("123");
    expect(wireValues2.get(2)).toBe("456");
  });
});
