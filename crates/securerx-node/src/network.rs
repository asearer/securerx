use crate::node::Node;
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;

/// P2P gossip: periodically sync blocks with peers using http for internal testing
impl Node {
    pub async fn gossip_loop(&self) {
        let client = Client::new();
        loop {
            for peer in &self.config.peers {
                let url = format!("http://{}/blocks", peer);
                if let Ok(resp) = client.get(&url).send().await {
                    if let Ok(remote_blocks) = resp.json::<Vec<securerx_core::block::Block>>().await {
                        self.sync_blocks(remote_blocks).await;
                    }
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    }

    async fn sync_blocks(&self, remote_blocks: Vec<securerx_core::block::Block>) {
        // Simple longest-chain sync
        let mut blockchain = self.blockchain.lock().unwrap();
        if remote_blocks.len() > blockchain.chain.len() {
            blockchain.chain = remote_blocks;
            crate::metrics::CHAIN_HEIGHT.set(blockchain.chain.len() as i64);
        }
    }
}
