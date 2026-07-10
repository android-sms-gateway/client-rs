use aes::Aes256;
use base64::Engine;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::RngCore;

use crate::Error;

const DEFAULT_ITERATIONS: u32 = 75_000;
const SALT_SIZE: usize = 16;

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// Encrypts and decrypts message content using AES-256-CBC with PBKDF2-SHA1 key derivation.
///
/// The encrypted format is:
/// `$aes-256-cbc/pbkdf2-sha1$i=<iterations>$<base64 salt>$<base64 ciphertext>`
///
/// ## Example
///
/// ```no_run
/// use android_sms_gateway::encryption::Encryptor;
///
/// let encryptor = Encryptor::new("my-passphrase");
/// let encrypted = encryptor.encrypt("Hello, world!");
/// let decrypted = encryptor.decrypt(&encrypted).unwrap();
/// assert_eq!(decrypted, "Hello, world!");
/// ```
pub struct Encryptor {
    passphrase: String,
    iterations: u32,
}

impl Encryptor {
    pub fn new(passphrase: impl Into<String>) -> Self {
        Self {
            passphrase: passphrase.into(),
            iterations: DEFAULT_ITERATIONS,
        }
    }

    pub fn with_iterations(passphrase: impl Into<String>, iterations: u32) -> Self {
        assert!(iterations >= 1_000, "iterations must be at least 1000");
        Self {
            passphrase: passphrase.into(),
            iterations,
        }
    }

    pub fn encrypt(&self, cleartext: &str) -> String {
        let mut salt = [0u8; SALT_SIZE];
        rand::rngs::OsRng.fill_bytes(&mut salt);

        let mut key = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<sha1::Sha1>(
            self.passphrase.as_bytes(),
            &salt,
            self.iterations,
            &mut key,
        );

        let cipher = Aes256CbcEnc::new_from_slices(&key, &salt).expect("valid key and IV length");
        let buf = cipher.encrypt_padded_vec_mut::<Pkcs7>(cleartext.as_bytes());

        format!(
            "$aes-256-cbc/pbkdf2-sha1$i={}${}${}",
            self.iterations,
            base64::engine::general_purpose::STANDARD.encode(salt),
            base64::engine::general_purpose::STANDARD.encode(&buf),
        )
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, Error> {
        let parts: Vec<&str> = encrypted.split('$').collect();
        if parts.len() < 5 {
            return Err(Error::Validation("Invalid encryption format".into()));
        }

        if parts[1] != "aes-256-cbc/pbkdf2-sha1" {
            return Err(Error::Validation("Unsupported algorithm".into()));
        }

        let iterations = parse_params(parts[2])
            .get("i")
            .and_then(|v| v.parse::<u32>().ok())
            .ok_or_else(|| Error::Validation("Missing iteration count".into()))?;

        let salt = base64::engine::general_purpose::STANDARD
            .decode(parts[3])
            .map_err(|_| Error::Validation("Invalid salt encoding".into()))?;

        if salt.len() != SALT_SIZE {
            return Err(Error::Validation("Invalid salt length".into()));
        }

        let encrypted_bytes = base64::engine::general_purpose::STANDARD
            .decode(parts[4])
            .map_err(|_| Error::Validation("Invalid data encoding".into()))?;

        if encrypted_bytes.len() % 16 != 0 {
            return Err(Error::Validation("Invalid encrypted data length".into()));
        }

        let mut key = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<sha1::Sha1>(self.passphrase.as_bytes(), &salt, iterations, &mut key);

        let cipher = Aes256CbcDec::new_from_slices(&key, &salt).expect("valid key and IV length");

        let unpadded = cipher
            .decrypt_padded_vec_mut::<Pkcs7>(&encrypted_bytes)
            .map_err(|_| Error::Validation("Decryption failed, invalid passphrase?".into()))?;

        String::from_utf8(unpadded)
            .map_err(|_| Error::Validation("Decrypted data is not valid UTF-8".into()))
    }
}

fn parse_params(params: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for pair in params.split(',') {
        if let Some((key, value)) = pair.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_known_value() {
        let passphrase = "passphrase";
        let cleartext = "hello";
        let encrypted =
            "$aes-256-cbc/pbkdf2-sha1$i=75000$obSTW6ittQvTtdAxonQKIw==$g3QFAC9CtBcPxoKlouqsyQ==";
        let encryptor = Encryptor::with_iterations(passphrase, 75000);
        let decrypted = encryptor.decrypt(encrypted).unwrap();
        assert_eq!(cleartext, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let passphrase = "correcthorsebatterystaple";
        let encryptor = Encryptor::with_iterations(passphrase, 1000);
        let cleartext = "The quick brown fox jumps over the lazy dog";
        let encrypted = encryptor.encrypt(cleartext);
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!(cleartext, decrypted);
    }

    #[test]
    fn test_decrypt_with_different_iterations() {
        let passphrase = "correcthorsebatterystaple";
        let encryptor = Encryptor::with_iterations(passphrase, 50000);
        let cleartext = "hello world";
        let encrypted = encryptor.encrypt(cleartext);
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!(cleartext, decrypted);
    }

    #[test]
    fn test_decrypt_empty_string() {
        let passphrase = "test";
        let encryptor = Encryptor::with_iterations(passphrase, 1000);
        let encrypted = encryptor.encrypt("");
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!("", decrypted);
    }

    #[test]
    fn test_decrypt_wrong_passphrase() {
        let encrypted =
            "$aes-256-cbc/pbkdf2-sha1$i=1000$obSTW6ittQvTtdAxonQKIw==$g3QFAC9CtBcPxoKlouqsyQ==";
        let encryptor = Encryptor::with_iterations("wrong", 1000);
        assert!(encryptor.decrypt(encrypted).is_err());
    }

    #[test]
    fn test_invalid_format() {
        let encryptor = Encryptor::with_iterations("test", 1000);
        assert!(encryptor.decrypt("invalid$format$string").is_err());
    }

    #[test]
    fn test_unsupported_algorithm() {
        let encryptor = Encryptor::with_iterations("test", 1000);
        assert!(encryptor.decrypt("$unsupported$i=0$salt$data").is_err());
    }

    #[test]
    fn test_missing_iteration_count() {
        let encryptor = Encryptor::with_iterations("test", 1000);
        assert!(encryptor
            .decrypt("$aes-256-cbc/pbkdf2-sha1$x=0$salt$data")
            .is_err());
    }

    #[test]
    fn test_default_iterations() {
        let encryptor = Encryptor::new("test");
        let cleartext = "hello";
        let encrypted = encryptor.encrypt(cleartext);
        assert!(encrypted.contains("i=75000"));
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        assert_eq!(cleartext, decrypted);
    }
}
