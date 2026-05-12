use ratatui::{Frame, layout::Rect};

use crate::ROBOT;

use crate::debug::helpers::{bool_icon, paragraph};

pub fn draw_stm32(f: &mut Frame, area: Rect) {
    let s = ROBOT.stm32_state.load();

    let text = format!(
        "Running: {}\nYaw Servo: {:?}\nWheel Velocities{:#?}\nConnected: {}\nFPS: {:.1}\n",
        bool_icon(s.start_flag),
        s.yaw_servo_state.current_angle,
        s.actual_wheel_velocities,
        bool_icon(s.driver_is_connected),
        if s.dt.as_secs_f32() > 0.0 {
            1.0 / s.dt.as_secs_f32()
        } else {
            0.0
        }
    );

    paragraph(f, area, "STM32", text);
}
