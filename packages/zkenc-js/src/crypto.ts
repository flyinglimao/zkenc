/**
 * Cryptographic utilities using Web Crypto API
 */

/**
 * Encrypt data using AES-256-GCM
 * 
 * @param key - 32-byte encryption key
 * @param data - Data to encrypt
 * @returns Encrypted data with nonce prepended (12 bytes nonce + encrypted data + 16 bytes tag)
 */
export async function aesGcmEncrypt(
  key: Uint8Array,
  data: Uint8Array
): Promise<Uint8Array> {
  if (key.length !== 32) {
    throw new Error('Key must be 32 bytes');
  }

  // Generate random 12-byte nonce
  const nonce = crypto.getRandomValues(new Uint8Array(12));

  // Import key
  const cryptoKey = await crypto.subtle.importKey(
    'raw',
    key as BufferSource,
    { name: 'AES-GCM' },
    false,
    ['encrypt']
  );

  // Encrypt
  const encrypted = await crypto.subtle.encrypt(
    {
      name: 'AES-GCM',
      iv: nonce,
      tagLength: 128, // 16 bytes
    },
    cryptoKey,
    data as BufferSource
  );

  // Prepend nonce to encrypted data
  const result = new Uint8Array(12 + encrypted.byteLength);
  result.set(nonce, 0);
  result.set(new Uint8Array(encrypted), 12);

  return result;
}

/**
 * Decrypt data using AES-256-GCM
 * 
 * @param key - 32-byte decryption key
 * @param encrypted - Encrypted data with nonce prepended
 * @returns Decrypted data
 */
export async function aesGcmDecrypt(
  key: Uint8Array,
  encrypted: Uint8Array
): Promise<Uint8Array> {
  if (key.length !== 32) {
    throw new Error('Key must be 32 bytes');
  }

  if (encrypted.length < 12 + 16) {
    throw new Error('Invalid encrypted data: too short');
  }

  // Extract nonce and ciphertext
  const nonce = encrypted.slice(0, 12);
  const ciphertext = encrypted.slice(12);

  // Import key
  const cryptoKey = await crypto.subtle.importKey(
    'raw',
    key as BufferSource,
    { name: 'AES-GCM' },
    false,
    ['decrypt']
  );

  // Decrypt
  try {
    const decrypted = await crypto.subtle.decrypt(
      {
        name: 'AES-GCM',
        iv: nonce,
        tagLength: 128,
      },
      cryptoKey,
      ciphertext as BufferSource
    );

    return new Uint8Array(decrypted);
  } catch (error) {
    throw new Error('Decryption failed: invalid key or corrupted data');
  }
}
