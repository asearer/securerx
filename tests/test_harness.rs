/// Integration test harness for consensus validation
/// This test validates that multiple nodes can reach consensus on the blockchain state

use securerx_core::blockchain::Blockchain;
use securerx_core::transaction::Transaction;
use securerx_core::crypto::{generate_keypair, sign_message};

#[test]
fn test_consensus_validation() {
    // Simulate three nodes starting with empty blockchains
    let mut node1 = Blockchain::new();
    let mut node2 = Blockchain::new();
    let mut node3 = Blockchain::new();

    // All nodes should start with the same genesis block
    assert_eq!(node1.chain.len(), 1);
    assert_eq!(node2.chain.len(), 1);
    assert_eq!(node3.chain.len(), 1);
    assert_eq!(node1.chain[0].index, 0);
    assert_eq!(node2.chain[0].index, 0);
    assert_eq!(node3.chain[0].index, 0);

    // Node 1 adds a transaction
    let keypair1 = generate_keypair();
    let sig1 = sign_message(&keypair1, b"Aspirin");
    let tx1 = Transaction {
        doctor_id: "doctor1".to_string(),
        patient_id: "patient1".to_string(),
        drug: "Aspirin".to_string(),
        signature: sig1.to_bytes().to_vec(),
        pubkey: keypair1.public.to_bytes().to_vec(),
    };
    node1.add_block(vec![tx1.clone()]);

    // Simulate gossip: nodes sync with node1
    // In a real scenario, node1 would broadcast the new block
    // Here we simulate by manually adding the same block to other nodes
    let node1_block1 = node1.chain[1].clone();
    node2.chain.push(node1_block1.clone());
    node3.chain.push(node1_block1.clone());

    // All nodes should now have the same chain length
    assert_eq!(node1.chain.len(), 2);
    assert_eq!(node2.chain.len(), 2);
    assert_eq!(node3.chain.len(), 2);

    // All nodes should validate the chain
    assert!(node1.validate_chain(), "Node1 chain should be valid");
    assert!(node2.validate_chain(), "Node2 chain should be valid");
    assert!(node3.validate_chain(), "Node3 chain should be valid");

    // Verify consensus: all nodes have the same block hash
    assert_eq!(
        node1.chain[1].calculate_hash(),
        node2.chain[1].calculate_hash(),
        "Nodes should have same block hash"
    );
    assert_eq!(
        node2.chain[1].calculate_hash(),
        node3.chain[1].calculate_hash(),
        "Nodes should have same block hash"
    );
}

#[test]
fn test_chain_validation_integrity() {
    let mut blockchain = Blockchain::new();
    
    // Add multiple valid transactions
    let keypair1 = generate_keypair();
    let keypair2 = generate_keypair();
    
    let sig1 = sign_message(&keypair1, b"Aspirin");
    let tx1 = Transaction {
        doctor_id: "doctor1".to_string(),
        patient_id: "patient1".to_string(),
        drug: "Aspirin".to_string(),
        signature: sig1.to_bytes().to_vec(),
        pubkey: keypair1.public.to_bytes().to_vec(),
    };
    
    let sig2 = sign_message(&keypair2, b"Ibuprofen");
    let tx2 = Transaction {
        doctor_id: "doctor2".to_string(),
        patient_id: "patient2".to_string(),
        drug: "Ibuprofen".to_string(),
        signature: sig2.to_bytes().to_vec(),
        pubkey: keypair2.public.to_bytes().to_vec(),
    };

    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    // Chain should be valid
    assert!(blockchain.validate_chain(), "Multi-block chain should be valid");
    assert_eq!(blockchain.chain.len(), 3, "Chain should have 3 blocks (genesis + 2)");
}

