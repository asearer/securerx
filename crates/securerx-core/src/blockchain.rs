use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, serde::Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    /// Initialize blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            prev_hash: String::from("0"),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: vec![],
            nonce: 0,
        };
        Self { chain: vec![genesis_block] }
    }

    /// Add a block to the chain
    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> &Block {
        let prev_block = self.chain.last().unwrap();
        let block = Block {
            index: prev_block.index + 1,
            prev_hash: prev_block.calculate_hash(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions,
            nonce: 0,
        };
        self.chain.push(block);
        self.chain.last().unwrap()
    }

    /// Validate the blockchain integrity
    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let curr = &self.chain[i];

            // Validate block hash chain
            if curr.prev_hash != prev.calculate_hash() {
                return false;
            }

            // Validate transactions
            for tx in &curr.transactions {
                if !tx.verify_signature() {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{generate_keypair, sign_message};

    #[test]
    fn test_blockchain_initialization() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1, "Blockchain should start with genesis block");
        assert_eq!(blockchain.chain[0].index, 0, "Genesis block should have index 0");
        assert_eq!(blockchain.chain[0].prev_hash, "0", "Genesis block should have prev_hash '0'");
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        let keypair = generate_keypair();
        let sig = sign_message(&keypair, b"Aspirin");
        
        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Aspirin".to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        let genesis_hash = blockchain.chain[0].calculate_hash();
        let block_index = {
            let block = blockchain.add_block(vec![tx]);
            block.index
        };
        assert_eq!(blockchain.chain.len(), 2, "Blockchain should have 2 blocks after adding one");
        assert_eq!(block_index, 1, "New block should have index 1");
        assert_eq!(blockchain.chain[1].prev_hash, genesis_hash, "New block should reference previous block hash");
    }

    #[test]
    fn test_validate_valid_chain() {
        let mut blockchain = Blockchain::new();
        let keypair = generate_keypair();
        let sig = sign_message(&keypair, b"Aspirin");
        
        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Aspirin".to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        blockchain.add_block(vec![tx]);
        assert!(blockchain.validate_chain(), "Valid chain should pass validation");
    }

    #[test]
    fn test_validate_invalid_chain_broken_link() {
        let mut blockchain = Blockchain::new();
        let keypair = generate_keypair();
        let sig = sign_message(&keypair, b"Aspirin");
        
        let tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Aspirin".to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        blockchain.add_block(vec![tx]);
        // Corrupt the chain by modifying prev_hash
        blockchain.chain[1].prev_hash = "corrupted".to_string();
        assert!(!blockchain.validate_chain(), "Chain with broken link should fail validation");
    }

    #[test]
    fn test_validate_invalid_chain_invalid_transaction() {
        let mut blockchain = Blockchain::new();
        let keypair = generate_keypair();
        let sig = sign_message(&keypair, b"Aspirin");
        
        let mut tx = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Aspirin".to_string(),
            signature: sig.to_bytes().to_vec(),
            pubkey: keypair.verifying_key().to_bytes().to_vec(),
        };

        blockchain.add_block(vec![tx.clone()]);
        // Corrupt the transaction signature
        tx.signature[0] = tx.signature[0].wrapping_add(1);
        blockchain.chain[1].transactions[0] = tx;
        
        assert!(!blockchain.validate_chain(), "Chain with invalid transaction should fail validation");
    }

    #[test]
    fn test_multiple_blocks() {
        let mut blockchain = Blockchain::new();
        let keypair1 = generate_keypair();
        let keypair2 = generate_keypair();

        let sig1 = sign_message(&keypair1, b"Aspirin");
        let tx1 = Transaction {
            doctor_id: "doctor1".to_string(),
            patient_id: "patient1".to_string(),
            drug: "Aspirin".to_string(),
            signature: sig1.to_bytes().to_vec(),
            pubkey: keypair1.verifying_key().to_bytes().to_vec(),
        };

        let sig2 = sign_message(&keypair2, b"Ibuprofen");
        let tx2 = Transaction {
            doctor_id: "doctor2".to_string(),
            patient_id: "patient2".to_string(),
            drug: "Ibuprofen".to_string(),
            signature: sig2.to_bytes().to_vec(),
            pubkey: keypair2.verifying_key().to_bytes().to_vec(),
        };

        blockchain.add_block(vec![tx1]);
        blockchain.add_block(vec![tx2]);
        
        assert_eq!(blockchain.chain.len(), 3, "Blockchain should have 3 blocks");
        assert!(blockchain.validate_chain(), "Multi-block chain should be valid");
    }
}
