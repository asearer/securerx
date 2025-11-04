use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use sha2::{Sha256, Digest};

/// Represents a blockchain block
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub prev_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    /// Calculate SHA256 hash of the block
    pub fn calculate_hash(&self) -> String {
        let data = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_hash_consistency() {
        let block = Block {
            index: 0,
            prev_hash: "0".to_string(),
            timestamp: 1234567890,
            transactions: vec![],
            nonce: 0,
        };
        let hash1 = block.calculate_hash();
        let hash2 = block.calculate_hash();
        assert_eq!(hash1, hash2, "Hash should be deterministic");
        assert!(!hash1.is_empty(), "Hash should not be empty");
    }

    #[test]
    fn test_block_hash_changes_with_data() {
        let block1 = Block {
            index: 0,
            prev_hash: "0".to_string(),
            timestamp: 1234567890,
            transactions: vec![],
            nonce: 0,
        };
        let block2 = Block {
            index: 1,
            prev_hash: "0".to_string(),
            timestamp: 1234567890,
            transactions: vec![],
            nonce: 0,
        };
        assert_ne!(block1.calculate_hash(), block2.calculate_hash(), "Different blocks should have different hashes");
    }
}
