#[cfg(target_os = "linux")]
pub mod camera;
#[cfg(target_os = "linux")]
pub use camera::*;

#[cfg(not(target_os = "linux"))]
pub mod stub;
#[cfg(not(target_os = "linux"))]
pub use stub::*;

