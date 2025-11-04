use crate::config::NodeConfig;
use std::sync::{Arc, Mutex};
use securerx_core::blockchain::Blockchain;

#[derive(Clone)]
pub struct Node {
    pub config: NodeConfig,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            config,
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }
}
