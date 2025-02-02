use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use portfolio::Result;
use portfolio::{Config, MetadataBackend, ObjectsBackend};
use portfolio::http;

#[tokio::main]
async fn main() -> Result<()> {
    // load configuration
    let mut dev_config = File::open("./dev-config.yml")?;
    let mut s = String::new();
    dev_config.read_to_string(&mut s)?;
    let config: Config = serde_yaml::from_str(&s)?;

    // initialize persistence layer
    let metadata = match config.metadata {
        MetadataBackend::Postgres(cfg) => cfg.new_metadata().await?,
    };
    let objects = match config.objects {
        ObjectsBackend::S3(cfg) => cfg.new_objects().await?,
    };

    // run HTTP server
    http::serve(Arc::new(metadata), Arc::new(objects)).await;
    Ok(())
}
