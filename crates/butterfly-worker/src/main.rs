//! Butterfly Worker Node

use butterfly_core::{WorkerCapabilities, WorkerId, WorkerNode};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Butterfly Worker Node");

    let capabilities = WorkerCapabilities {
        worker_id: WorkerId::new(),
        cores: num_cpus::get(),
        memory: sys_info::mem_info()
            .map(|info| info.total * 1024)
            .unwrap_or(8_000_000_000),
        has_gpu: false, // Detection would go here
        bandwidth: 100.0,
    };

    let mut worker = WorkerNode::new(capabilities);

    // Main worker loop
    loop {
        worker.heartbeat();
        worker.update_load(0.3); // Placeholder

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
