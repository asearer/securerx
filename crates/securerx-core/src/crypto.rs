use ed25519_dalek::{SigningKey, Signature, Signer};
use rand::rngs::OsRng;

/// Generate a new Ed25519 keypair
pub fn generate_keypair() -> SigningKey {
    let mut csprng = OsRng{};
    SigningKey::generate(&mut csprng)
}

/// Sign a message with a keypair
pub fn sign_message(keypair: &SigningKey, message: &[u8]) -> Signature {
    keypair.sign(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair();
        // Verify keypair has valid public key
        let _pubkey = keypair.verifying_key();
        assert_eq!(keypair.verifying_key().to_bytes().len(), 32);
    }

    #[test]
    fn test_unique_keypairs() {
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();
        assert_ne!(
            keypair1.verifying_key().to_bytes(),
            keypair2.verifying_key().to_bytes(),
            "Generated keypairs should be unique"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = generate_keypair();
        let message = b"Test prescription data";
        let signature = sign_message(&keypair, message);
        
        // Verify signature
        assert!(keypair.verifying_key().verify_strict(message, &signature).is_ok(), "Signature should verify");
    }

    #[test]
    fn test_sign_different_messages() {
        let keypair = generate_keypair();
        let message1 = b"Message 1";
        let message2 = b"Message 2";
        let sig1 = sign_message(&keypair, message1);
        let sig2 = sign_message(&keypair, message2);
        
        assert_ne!(sig1.to_bytes(), sig2.to_bytes(), "Different messages should have different signatures");
    }
}
