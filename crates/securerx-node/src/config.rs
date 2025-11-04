/// Node configuration loaded from environment variables
#[derive(Clone)]
pub struct NodeConfig {
    pub node_id: String,
    pub data_dir: String,
    pub api_addr: String,
    pub peers: Vec<String>,
}

impl NodeConfig {
    pub fn from_env() -> Self {
        let peers = std::env::var("PEERS").unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        Self {
            node_id: std::env::var("NODE_ID").unwrap_or_else(|_| "node1".to_string()),
            data_dir: std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
            api_addr: std::env::var("API_ADDR").unwrap_or_else(|_| "0.0.0.0:8081".to_string()),
            peers,
        }
    }
}
