use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::error::Error;

/// CLI for SecureRx blockchain
#[derive(Parser)]
#[clap(name="SecureRx CLI")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Node API URL (default: http://localhost:8080)
    #[clap(long, default_value="http://localhost:8080")]
    node_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Issue a new prescription
    IssuePrescription {
        doctor_id: String,
        patient_id: String,
        drug: String,
    },
    /// Query all blocks
    GetBlocks,
    /// Query a specific block
    GetBlock {
        index: usize,
    },
    /// Health check
    Health,
}

/// Prescription submission payload
#[derive(Serialize)]
struct PrescriptionRequest {
    doctor_id: String,
    patient_id: String,
    drug: String,
}

/// Response for prescription submission
#[derive(Deserialize)]
struct PrescriptionResponse {
    status: String,
    block_index: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::IssuePrescription { doctor_id, patient_id, drug } => {
            let payload = PrescriptionRequest { doctor_id, patient_id, drug };
            let resp = client.post(&format!("{}/prescription", cli.node_url))
                .json(&payload)
                .send()?
                .json::<PrescriptionResponse>()?;
            println!("Prescription submitted successfully. Block index: {}", resp.block_index);
        }
        Commands::GetBlocks => {
            let resp = client.get(&format!("{}/blocks", cli.node_url))
                .send()?
                .text()?;
            println!("{}", resp);
        }
        Commands::GetBlock { index } => {
            let resp = client.get(&format!("{}/blocks/{}", cli.node_url, index))
                .send()?
                .text()?;
            println!("{}", resp);
        }
        Commands::Health => {
            let resp = client.get(&format!("{}/health", cli.node_url))
                .send()?
                .text()?;
            println!("Health status: {}", resp);
        }
    }

    Ok(())
}
