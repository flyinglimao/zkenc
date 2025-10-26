/**
 * Parser for Circom .sym files
 *
 * The .sym file format is:
 * <label_id>,<wire_id>,<component_id>,<signal_name>
 *
 * Example:
 * 1,1,172,main.root
 * 2,2,172,main.message
 * 3,3,172,main.R8x
 */

export interface SymbolEntry {
  labelId: number;
  wireId: number;
  componentId: number;
  signalName: string;
}

/**
 * Parse Circom .sym file to get signal name to wire ID mapping
 *
 * @param symContent - Content of the .sym file as string
 * @returns Map from signal name to wire ID
 */
export function parseSymFile(symContent: string): Map<string, number> {
  const lines = symContent.trim().split("\n");
  const wireMap = new Map<string, number>();

  for (const line of lines) {
    const parts = line.split(",");
    if (parts.length !== 4) {
      continue; // Skip invalid lines
    }

    const labelId = parseInt(parts[0], 10);
    const wireId = parseInt(parts[1], 10);
    const componentId = parseInt(parts[2], 10);
    const signalName = parts[3].trim();

    // Only include signals with valid wire IDs (wireId >= 0)
    // Wire ID -1 means internal signal, not an input
    if (wireId >= 0) {
      wireMap.set(signalName, wireId);
    }
  }

  return wireMap;
}

/**
 * Extract input signals from wire mapping
 * Filters for signals that start with "main." and are inputs
 *
 * @param wireMap - Wire mapping from parseSymFile
 * @param maxWireId - Maximum wire ID to consider as input (typically n_pub_in + n_pub_out)
 * @returns Map from simplified signal name (without "main.") to wire ID
 */
export function getInputSignals(
  wireMap: Map<string, number>,
  maxWireId?: number
): Map<string, number> {
  const inputSignals = new Map<string, number>();

  for (const [signalName, wireId] of wireMap.entries()) {
    // Skip if wireId exceeds maximum (if specified)
    if (maxWireId !== undefined && wireId > maxWireId) {
      continue;
    }

    // Extract input signals (those starting with "main.")
    if (signalName.startsWith("main.")) {
      const simplifiedName = signalName.substring(5); // Remove "main." prefix
      inputSignals.set(simplifiedName, wireId);
    }
  }

  return inputSignals;
}

/**
 * Map JSON inputs to wire values using symbol file
 *
 * @param inputs - JSON inputs as key-value pairs
 * @param wireMap - Wire mapping from parseSymFile
 * @returns Map from wire ID to field element value (as string)
 */
export function mapInputsToWires(
  inputs: Record<string, any>,
  wireMap: Map<string, number>
): Map<number, string | string[]> {
  const wireValues = new Map<number, string | string[]>();

  function flattenInput(key: string, value: any, prefix: string = ""): void {
    const fullKey = prefix ? `${prefix}.${key}` : key;

    if (Array.isArray(value)) {
      // Handle array inputs
      for (let i = 0; i < value.length; i++) {
        const arrayKey = `${fullKey}[${i}]`;
        const wireId = wireMap.get(arrayKey);
        if (wireId !== undefined) {
          wireValues.set(wireId, value[i].toString());
        }
      }
    } else if (typeof value === "object" && value !== null) {
      // Recursively handle nested objects
      for (const [nestedKey, nestedValue] of Object.entries(value)) {
        flattenInput(nestedKey, nestedValue, fullKey);
      }
    } else {
      // Scalar value
      const wireId = wireMap.get(fullKey);
      if (wireId !== undefined) {
        wireValues.set(wireId, value.toString());
      }
    }
  }

  // Process all inputs
  for (const [key, value] of Object.entries(inputs)) {
    flattenInput(key, value);
  }

  return wireValues;
}
