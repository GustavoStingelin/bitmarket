use wasm_bindgen::prelude::*;
use bitcoin::Network;
use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::key::UntweakedKeyPair;
use bitcoin::psbt::PartiallySignedTransaction;
use bitcoin::Transaction;
use base64::{Engine as _, engine::general_purpose::STANDARD};

/// A Bitcoin wallet that supports Taproot and PSBT operations
#[wasm_bindgen]
pub struct Wallet {
    network: Network,
    secp: Secp256k1<bitcoin::secp256k1::All>,
    keys: Vec<SecretKey>,
}

#[wasm_bindgen]
impl Wallet {
    /// Creates a new wallet instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Wallet {
            network: Network::Testnet,
            secp: Secp256k1::new(),
            keys: Vec::new(),
        }
    }

    /// Synchronizes the wallet with the blockchain
    #[wasm_bindgen]
    pub fn sync(&mut self) -> Result<(), JsValue> {
        // Placeholder for actual sync implementation
        Ok(())
    }

    /// Creates a new PSBT for a transaction
    #[wasm_bindgen]
    pub fn create_psbt(&self) -> Result<String, JsValue> {
        // Create an empty transaction
        let tx = Transaction {
            version: 2,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: Vec::new(),
            output: Vec::new(),
        };
        
        // Create PSBT from the transaction
        let psbt = PartiallySignedTransaction::from_unsigned_tx(tx)
            .map_err(|e| JsValue::from_str(&format!("Failed to create PSBT: {}", e)))?;
        
        // Serialize PSBT to base64
        let psbt_base64 = STANDARD.encode(psbt.serialize());
        Ok(psbt_base64)
    }

    #[wasm_bindgen]
    pub fn get_network(&self) -> String {
        format!("{:?}", self.network)
    }

    #[wasm_bindgen]
    pub fn generate_taproot_key(&self) -> String {
        // Generate a random key pair for taproot
        let keypair = UntweakedKeyPair::new(&self.secp, &mut rand::thread_rng());
        
        // Get the x-only public key
        let (pubkey, _parity) = keypair.x_only_public_key();
        
        // Return the public key as hex
        pubkey.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert_eq!(wallet.get_network(), "Testnet");
    }

    #[test]
    fn test_psbt_creation() {
        let wallet = Wallet::new();
        let psbt_base64 = wallet.create_psbt().unwrap();
        assert!(!psbt_base64.is_empty());
        // Verify it's valid base64
        assert!(STANDARD.decode(&psbt_base64).is_ok());
    }

    #[test]
    fn test_taproot_key_generation() {
        let wallet = Wallet::new();
        let pubkey = wallet.generate_taproot_key();
        assert!(!pubkey.is_empty());
        // Verify it's a valid hex string
        assert!(hex::decode(&pubkey).is_ok());
    }
}

// More wallet functionality to be added 