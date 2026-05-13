use std::sync::Arc;

use glam::Vec2;

use crate::{ROBOT, devices::maixcam::{MAIXCAM_CAMERA_RESOLUTION_HEIGHT, MAIXCAM_CAMERA_RESOLUTION_WIDTH, color::MaixcamCircleColor, sample::MaixcamSample}};


#[derive(Debug, Default, Clone)]
pub struct MaixcamState {
    pub driver_is_connected: bool,
    pub circle_position: Vec2,
    pub circle_color: MaixcamCircleColor,
    /// Delta time for FPS calculation
    pub dt: std::time::Duration,
}

impl MaixcamState {
    pub fn new() -> Self {
        MaixcamState::default()
    }

    pub fn update(&mut self, sample: MaixcamSample) {
        self.circle_position = Vec2 {
            x: sample.circle_position_x as f32 / 10000.0 * MAIXCAM_CAMERA_RESOLUTION_WIDTH,
            y: sample.circle_position_y as f32 / 10000.0 * MAIXCAM_CAMERA_RESOLUTION_HEIGHT,
        };
        self.circle_color = MaixcamCircleColor::from_id(sample.circle_color_id);
    }

    pub fn publish(&self) {
        ROBOT.maixcam_state.store(Arc::new(self.clone()));
    }
}
