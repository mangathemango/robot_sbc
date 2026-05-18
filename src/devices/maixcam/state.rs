use std::fmt::Display;

use crate::{
    ROBOT,
    devices::maixcam::circle::{MaixcamCircle, MaixcamCircleColor, MaixcamCircleKind},
};

#[derive(Debug, Default, Clone)]
pub struct MaixcamState {
    pub driver_is_connected: bool,
    pub circles: Vec<MaixcamCircle>,
    pub error: Option<String>,
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

    pub fn find_priority_ring(
        &self,
        priority_list: &[MaixcamCircleColor],
    ) -> Option<MaixcamCircle> {
        for color in priority_list {
            if let Some(circle) = self.find_ring(color) {
                return Some(circle.clone());
            }
        }
        None
    }

    pub fn find_ring(&self, color: &MaixcamCircleColor) -> Option<&MaixcamCircle> {
        self.circles.iter().find(|circle| circle.color == *color && circle.kind == MaixcamCircleKind::Ring)
    }
}

impl Display for MaixcamState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Connected: {}\nCircles {}\ndt: {:?}\nError: {:?}",
            self.driver_is_connected,
            self.circles.iter().fold("".to_string(), |acc, circle| (acc
                + format!("{}\n", circle).as_str())),
            self.dt,
            self.error
        )
    }
}
