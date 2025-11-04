use serde::{Serialize, Deserialize};
use ed25519_dalek::{VerifyingKey, Signature};

/// Represents a prescription transaction
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub doctor_id: String,
    pub patient_id: String,
    pub drug: String,
    pub signature: Vec<u8>,
    pub pubkey: Vec<u8>,
}

impl Transaction {
    /// Verify the transaction signature
    pub fn verify_signature(&self) -> bool {
        // Convert Vec<u8> to fixed-size arrays
        if self.pubkey.len() != 32 || self.signature.len() != 64 {
            return false;
        }
        
        let pubkey_bytes: [u8; 32] = match self.pubkey.as_slice().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        
        let sig_bytes: [u8; 64] = match self.signature.as_slice().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        
        let pubkey = match VerifyingKey::from_bytes(&pubkey_bytes) {
            Ok(pk) => pk,
            Err(_) => return false,
        };
        
        let sig = Signature::from_bytes(&sig_bytes);
        pubkey.verify_strict(self.drug.as_bytes(), &sig).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{generate_keypair, sign_message};

    #[test]
    fn test_valid_transaction_signature() {
        let keypair = generate_keypair();
        let drug = "Aspirin";
        let sig = sign_message(&keypair, drug.as_bytes());

        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: drug.to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        assert!(tx.verify_signature(), "Valid signature should verify");
    }

    #[test]
    fn test_invalid_transaction_signature_wrong_drug() {
        let keypair = generate_keypair();
        let drug = "Aspirin";
        let sig = sign_message(&keypair, drug.as_bytes());

        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Ibuprofen".to_string(), // Different drug
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        assert!(!tx.verify_signature(), "Signature for wrong drug should fail");
    }

    #[test]
    fn test_invalid_transaction_signature_wrong_key() {
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();
        let drug = "Aspirin";
        let sig = sign_message(&keypair1, drug.as_bytes());

        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: drug.to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair2.verifying_key().to_bytes().to_vec(), // Wrong public key
        };

        assert!(!tx.verify_signature(), "Signature with wrong key should fail");
    }

    #[test]
    fn test_invalid_transaction_signature_corrupted() {
        let keypair = generate_keypair();
        let drug = "Aspirin";
        let mut sig_bytes = sign_message(&keypair, drug.as_bytes()).to_bytes().to_vec();
        sig_bytes[0] = sig_bytes[0].wrapping_add(1); // Corrupt signature

        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: drug.to_string(),
            signature: sig_bytes,
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        assert!(!tx.verify_signature(), "Corrupted signature should fail");
    }
}
