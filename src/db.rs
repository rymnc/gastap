use fuel_core::combined_database::CombinedDatabase;
use fuel_core::state::rocks_db::DatabaseConfig;
use std::path::Path;

pub fn get_db(path: &Path) -> anyhow::Result<CombinedDatabase> {
    CombinedDatabase::open_read_only(
        path,
        DatabaseConfig {
            cache_capacity: None,
            max_fds: -1,
            columns_policy: Default::default(),
        },
    )
    .map_err(|e| anyhow::anyhow!("Failed to open database: {:?}", e))
}
