use aes::Aes256;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr64BE;

type Aes256Ctr = Ctr64BE<Aes256>;

/// Encrypt data using AES-256-GCM (Galois/Counter Mode)
///
/// GCM provides both confidentiality and authenticity.
///
/// # Format
/// Output: [nonce(12 bytes)][ciphertext + tag(16 bytes)]
///
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `plaintext` - Data to encrypt
///
/// # Returns
/// Combined nonce + ciphertext + authentication tag
pub fn encrypt_gcm(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    // Validate key length
    if key.len() != 32 {
        anyhow::bail!("Key must be 32 bytes (256 bits), got {}", key.len());
    }

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key).context("Failed to create AES-GCM cipher")?;

    // Generate random nonce (12 bytes for GCM)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Combine nonce + ciphertext
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt data using AES-256-GCM
///
/// # Arguments
/// * `key` - 32-byte encryption key (same as used for encryption)
/// * `data` - Combined nonce + ciphertext + tag
///
/// # Returns
/// Original plaintext
pub fn decrypt_gcm(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // Validate key length
    if key.len() != 32 {
        anyhow::bail!("Key must be 32 bytes (256 bits), got {}", key.len());
    }

    // Validate minimum data length (nonce + tag)
    if data.len() < 28 {
        anyhow::bail!("Data too short, need at least 28 bytes (12 nonce + 16 tag)");
    }

    // Create cipher
    let cipher = Aes256Gcm::new_from_slice(key).context("Failed to create AES-GCM cipher")?;

    // Extract nonce (first 12 bytes)
    let nonce = Nonce::from_slice(&data[..12]);

    // Extract ciphertext (rest)
    let ciphertext = &data[12..];

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
        anyhow::anyhow!(
            "Decryption failed (authentication failed or corrupted data): {}",
            e
        )
    })?;

    Ok(plaintext)
}

/// Encrypt data using AES-256-CTR (Counter Mode)
///
/// CTR mode provides confidentiality but not authenticity.
/// Consider using GCM if you need authentication.
///
/// # Format
/// Output: [iv(16 bytes)][ciphertext]
///
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `plaintext` - Data to encrypt
///
/// # Returns
/// Combined IV + ciphertext
pub fn encrypt_ctr(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    // Validate key length
    if key.len() != 32 {
        anyhow::bail!("Key must be 32 bytes (256 bits), got {}", key.len());
    }

    // Generate random IV (16 bytes for AES)
    let mut iv = [0u8; 16];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut iv);

    // Create cipher
    let mut cipher = Aes256Ctr::new(key.into(), &iv.into());

    // Encrypt (CTR mode is symmetric - same operation for encrypt/decrypt)
    let mut ciphertext = plaintext.to_vec();
    cipher.apply_keystream(&mut ciphertext);

    // Combine IV + ciphertext
    let mut result = iv.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt data using AES-256-CTR
///
/// # Arguments
/// * `key` - 32-byte encryption key (same as used for encryption)
/// * `data` - Combined IV + ciphertext
///
/// # Returns
/// Original plaintext
pub fn decrypt_ctr(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // Validate key length
    if key.len() != 32 {
        anyhow::bail!("Key must be 32 bytes (256 bits), got {}", key.len());
    }

    // Validate minimum data length (IV)
    if data.len() < 16 {
        anyhow::bail!("Data too short, need at least 16 bytes for IV");
    }

    // Extract IV (first 16 bytes)
    let iv = &data[..16];

    // Extract ciphertext (rest)
    let ciphertext = &data[16..];

    // Create cipher
    let mut cipher = Aes256Ctr::new(key.into(), iv.into());

    // Decrypt (CTR mode is symmetric)
    let mut plaintext = ciphertext.to_vec();
    cipher.apply_keystream(&mut plaintext);

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcm_roundtrip() {
        let key = b"12345678901234567890123456789012"; // 32 bytes
        let plaintext = b"Hello, World!";

        let encrypted = encrypt_gcm(key, plaintext).unwrap();
        assert!(encrypted.len() > plaintext.len());

        let decrypted = decrypt_gcm(key, &encrypted).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_ctr_roundtrip() {
        let key = b"12345678901234567890123456789012"; // 32 bytes
        let plaintext = b"Hello, World!";

        let encrypted = encrypt_ctr(key, plaintext).unwrap();
        assert!(encrypted.len() > plaintext.len());

        let decrypted = decrypt_ctr(key, &encrypted).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_gcm_wrong_key() {
        let key1 = b"12345678901234567890123456789012";
        let key2 = b"99999999999999999999999999999999";
        let plaintext = b"Secret message";

        let encrypted = encrypt_gcm(key1, plaintext).unwrap();
        let result = decrypt_gcm(key2, &encrypted);

        assert!(result.is_err(), "Should fail with wrong key");
    }

    #[test]
    fn test_invalid_key_length() {
        let short_key = b"too_short";
        let plaintext = b"data";

        assert!(encrypt_gcm(short_key, plaintext).is_err());
        assert!(encrypt_ctr(short_key, plaintext).is_err());
    }
}
