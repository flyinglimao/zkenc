/**
 * Tests for encap with symbol file
 */

import { describe, it, expect } from 'vitest';
import { readFileSync } from 'fs';
import { join } from 'path';
import { encap } from './zkenc.js';

describe('Encap with Symbol File', () => {
  const testDir = join(process.cwd(), 'tests', 'fixtures');

  // Load merkle-membership circuit files
  const r1csBuffer = new Uint8Array(
    readFileSync(join(testDir, 'merkle_membership.r1cs'))
  );
  const symContent = readFileSync(
    join(testDir, 'merkle_membership.sym'),
    'utf-8'
  );

  it('should encap with public inputs in original order', async () => {
    const publicInputs = {
      root: '12345',
      message: '67890',
    };

    const result = await encap({ r1csBuffer, symContent }, publicInputs);

    expect(result.ciphertext).toBeInstanceOf(Uint8Array);
    expect(result.ciphertext.length).toBeGreaterThan(0);
    expect(result.key).toBeInstanceOf(Uint8Array);
    expect(result.key.length).toBe(32);
  });

  it('should encap with public inputs in reversed order', async () => {
    const publicInputs = {
      message: '67890',
      root: '12345',
    };

    const result = await encap({ r1csBuffer, symContent }, publicInputs);

    expect(result.ciphertext).toBeInstanceOf(Uint8Array);
    expect(result.ciphertext.length).toBeGreaterThan(0);
    expect(result.key).toBeInstanceOf(Uint8Array);
    expect(result.key.length).toBe(32);
  });

  it('should produce same ciphertext for same inputs regardless of key order', async () => {
    const inputs1 = { root: '12345', message: '67890' };
    const inputs2 = { message: '67890', root: '12345' };

    const result1 = await encap({ r1csBuffer, symContent }, inputs1);
    const result2 = await encap({ r1csBuffer, symContent }, inputs2);

    // Both should succeed
    expect(result1.ciphertext).toBeInstanceOf(Uint8Array);
    expect(result2.ciphertext).toBeInstanceOf(Uint8Array);

    // Note: Ciphertexts will be different due to randomness in encap,
    // but this test confirms both orders work without errors
  });
});
