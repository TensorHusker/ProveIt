//! Butterfly Coordinator Node

use butterfly_core::{
    FunctionalDecomposition, ModelId, ModelSplit, WorkerId,
};
use dashmap::DashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Butterfly Coordinator");

    let workers: Arc<DashMap<WorkerId, String>> = Arc::new(DashMap::new());
    let models: Arc<DashMap<ModelId, ModelSplit>> = Arc::new(DashMap::new());

    // Create example model split
    let decomp = FunctionalDecomposition::decompose_transformer(12, 768);
    let split = ModelSplit::new(ModelId::new(), decomp);

    tracing::info!(
        "Created model split with {} components",
        split.decomposition.components.len()
    );

    // Main coordinator loop
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
