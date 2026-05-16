use std::sync::Arc;

use crate::{
    ROBOT,
    devices::maixcam::circle::{MaixcamCircle, MaixcamCircleColor},
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
        ROBOT.set_maixcam_state(self.clone());
    }

    pub fn find_priority_circle(&self, priority_list: &[MaixcamCircleColor]) -> Option<&MaixcamCircle> {
        for color in priority_list {
            if let Some(circle) = self.find_circle(color) {
                return Some(circle)
            }
        }
        None
    }

    pub fn find_circle(&self, color: &MaixcamCircleColor) -> Option<&MaixcamCircle> {
        self.circles
            .iter()
            .find(|circle| circle.color == *color)
    }
}
