//! Worker node management and task execution

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::{ComponentId, TaskId, WorkerId};

/// Capabilities of a worker node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapabilities {
    /// Worker identifier
    pub worker_id: WorkerId,
    /// Available compute cores
    pub cores: usize,
    /// Available memory (bytes)
    pub memory: u64,
    /// GPU availability
    pub has_gpu: bool,
    /// Network bandwidth (Mbps)
    pub bandwidth: f64,
}

/// A worker node in the distributed system
pub struct WorkerNode {
    pub capabilities: WorkerCapabilities,
    /// Current load (0.0 - 1.0)
    load: f32,
    /// Last heartbeat time
    last_heartbeat: SystemTime,
}

impl WorkerNode {
    pub fn new(capabilities: WorkerCapabilities) -> Self {
        Self {
            capabilities,
            load: 0.0,
            last_heartbeat: SystemTime::now(),
        }
    }

    pub fn update_load(&mut self, load: f32) {
        self.load = load.clamp(0.0, 1.0);
    }

    pub fn heartbeat(&mut self) {
        self.last_heartbeat = SystemTime::now();
    }

    pub fn is_alive(&self, timeout: Duration) -> bool {
        self.last_heartbeat.elapsed().unwrap_or(Duration::MAX) < timeout
    }
}

/// A task to be executed by a worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkTask {
    pub task_id: TaskId,
    pub component_id: ComponentId,
    pub input_data: Vec<f32>,
    pub priority: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_node() {
        let caps = WorkerCapabilities {
            worker_id: WorkerId::new(),
            cores: 8,
            memory: 16_000_000_000,
            has_gpu: true,
            bandwidth: 1000.0,
        };

        let mut worker = WorkerNode::new(caps);
        worker.update_load(0.5);
        assert_eq!(worker.load, 0.5);
    }
}
