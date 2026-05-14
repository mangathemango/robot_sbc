use std::sync::Arc;

use crate::{
    ROBOT,
    devices::maixcam::{
        circle::MaixcamCircle,
    },
};

#[derive(Debug, Default, Clone)]
pub struct MaixcamState {
    pub driver_is_connected: bool,
    pub circles: Vec<MaixcamCircle>,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl MaixcamState {
    pub fn new() -> Self {
        MaixcamState::default()
    }

    pub fn publish(&self) {
        ROBOT.maixcam_state.store(Arc::new(self.clone()));
    }
}
