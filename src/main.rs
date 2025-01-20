mod db;

use crate::db::get_db;
use clap::Parser;
use fuel_core::types::fuel_types::BlockHeight;
use fuel_core_gas_price_service::ports::{
    GasPriceData, GetLatestRecordedHeight, GetMetadataStorage,
};
use std::path::Path;

/// Simple CLI utility to inspect the database used by fuel-core
#[derive(Parser, Debug)]
#[clap(name = "gastap")]
pub enum Command {
    /// Gets the latest stored metadata in the gas price database
    GetGasPriceMetadata {
        /// Path to the database
        #[clap(long, short)]
        db_path: String,
    },
    /// Gets the latest DA recorded height in the gas price database
    GetDaRecordedHeight {
        /// Path to the database
        #[clap(long, short)]
        db_path: String,
    },
    /// Shows the difference in metadatas
    MetadataDiff {
        /// Path to the database
        #[clap(long, short)]
        db_path: String,
        /// Number of blocks to go back
        #[clap(long, short)]
        number_of_blocks: u32,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();
    match args {
        Command::GetGasPriceMetadata { db_path } => {
            let path = Path::new(db_path.as_str());
            let combined_db = get_db(path)?;

            let gas_price_db = combined_db.gas_price();
            let height = gas_price_db
                .latest_height()
                .ok_or(anyhow::anyhow!("Failed to get latest height"))?;
            let metadata = gas_price_db
                .get_metadata(&height)?
                .ok_or(anyhow::anyhow!("Failed to get metadata"))?;
            println!("Metadata: {:#?}", metadata);
        }
        Command::GetDaRecordedHeight { db_path } => {
            let path = Path::new(db_path.as_str());
            let combined_db = get_db(path)?;

            let gas_price_db = combined_db.gas_price();
            let recorded_height = gas_price_db
                .get_recorded_height()?
                .ok_or(anyhow::anyhow!("No recorded height"))?;
            println!("Recorded height: {}", recorded_height);
        }
        Command::MetadataDiff {
            db_path,
            number_of_blocks,
        } => {
            let path = Path::new(db_path.as_str());
            let combined_db = get_db(path)?;

            let gas_price_db = combined_db.gas_price();
            let current_height = gas_price_db
                .latest_height()
                .ok_or(anyhow::anyhow!("Failed to get latest height"))?;

            let prev_height = BlockHeight::new(*current_height - number_of_blocks);
            let current_metadata = gas_price_db
                .get_metadata(&current_height)?
                .ok_or(anyhow::anyhow!("Failed to get current metadata"))?;
            let current_metadata = current_metadata
                .v1()
                .ok_or(anyhow::anyhow!("Couldn't convert current metadata to v1"))?;
            let prev_metadata = gas_price_db
                .get_metadata(&prev_height)?
                .ok_or(anyhow::anyhow!("Failed to get old metadata"))?;
            let prev_metadata = prev_metadata
                .v1()
                .ok_or(anyhow::anyhow!("Couldn't convert old metadata to v1"))?;

            println!("Current metadata: {:#?}", current_metadata);
            println!("Previous metadata: {:#?}", prev_metadata);
        }
    }

    Ok(())
}
