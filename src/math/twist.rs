use glam::Vec2;

#[derive(Debug, Clone, Copy, Default)]
pub struct Twist {
    pub linear: Vec2, // (vx, vy)
    pub omega: f32,
}

impl Twist {
    pub fn new(linear: Vec2, omega: f32) -> Self {
        Self { linear, omega }
    }

    pub fn from_components(vx: f32, vy: f32, omega: f32) -> Self {
        Self {
            linear: Vec2::new(vx, vy),
            omega,
        }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn linear_speed(&self) -> f32 {
        self.linear.length()
    }
    
    pub fn clamp_linear(mut self, max: f32) -> Self {
        let len = self.linear.length();

        if len > max && len > 0.0 {
            self.linear *= max / len;
        }

        self
    }

    pub fn clamp_omega(mut self, max: f32) -> Self {
        self.omega = self.omega.clamp(-max, max);
        self
    }
}