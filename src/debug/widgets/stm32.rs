use ratatui::{Frame, layout::Rect};

use crate::ROBOT;
use crate::devices::stm32::Stm32State;

use crate::debug::helpers::{paragraph, bool_icon};

pub fn draw_stm32(f: &mut Frame, area: Rect, s: &Stm32State) {
    let motion_state = ROBOT.motion_state.load();
    let text = format!(
        "Running: {}\nWheels: {:?}\nConnected: {}\n{:#?} ",
        bool_icon(s.start_flag),
        s.actual_wheel_velocities,
        bool_icon(s.driver_is_connected),
        motion_state.current_pose
    );

    paragraph(f, area, "STM32", text);
}