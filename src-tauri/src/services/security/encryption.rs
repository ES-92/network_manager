use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use argon2::{Argon2, password_hash::SaltString};
use std::path::PathBuf;

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

pub struct ConfigEncryption {
    config_path: PathBuf,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EncryptedConfig {
    salt: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl ConfigEncryption {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("network_manager");

        std::fs::create_dir_all(&config_dir).ok();

        Self {
            config_path: config_dir.join("config.enc"),
        }
    }

    /// Derive encryption key from password using Argon2id
    fn derive_key(&self, password: &str, salt: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error + Send + Sync>> {
        let argon2 = Argon2::default();
        let mut key = [0u8; 32];

        argon2.hash_password_into(
            password.as_bytes(),
            salt,
            &mut key,
        ).map_err(|e| format!("Key derivation failed: {}", e))?;

        Ok(key)
    }

    /// Encrypt configuration data
    pub fn encrypt(&self, data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Generate random salt
        let salt_string = SaltString::generate(&mut OsRng);
        let salt = salt_string.as_str().as_bytes().to_vec();

        // Derive key from password
        let key = self.derive_key(password, &salt)?;

        // Generate random nonce
        let nonce_bytes: [u8; NONCE_SIZE] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt data
        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Serialize encrypted config
        let encrypted = EncryptedConfig {
            salt,
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        };

        Ok(serde_json::to_vec(&encrypted)?)
    }

    /// Decrypt configuration data
    pub fn decrypt(&self, encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Deserialize encrypted config
        let encrypted: EncryptedConfig = serde_json::from_slice(encrypted_data)?;

        // Derive key from password
        let key = self.derive_key(password, &encrypted.salt)?;

        // Decrypt data
        let nonce = Nonce::from_slice(&encrypted.nonce);
        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|_| "Decryption failed - wrong password?")?;

        Ok(plaintext)
    }

    /// Save encrypted configuration to file
    pub fn save_config<T: serde::Serialize>(&self, config: &T, password: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_vec(config)?;
        let encrypted = self.encrypt(&json, password)?;
        std::fs::write(&self.config_path, encrypted)?;
        Ok(())
    }

    /// Load and decrypt configuration from file
    pub fn load_config<T: serde::de::DeserializeOwned>(&self, password: &str) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let encrypted = std::fs::read(&self.config_path)?;
        let decrypted = self.decrypt(&encrypted, password)?;
        let config: T = serde_json::from_slice(&decrypted)?;
        Ok(config)
    }

    /// Check if encrypted config file exists
    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }
}
