use clap::Parser;

use crate::ease::Easing;

/// CLI arguments
#[derive(Parser, Debug, Clone)]
#[command(
    name = "sway-focus-flash",
    about = "Animate focused window opacity in Sway"
)]
pub struct Args {
    /// Starting opacity (0.0 to 1.0)
    #[arg(long, default_value_t = 0.8)]
    pub start_opacity: f32,

    /// Ending opacity (0.0 to 1.0)
    #[arg(long, default_value_t = 1.0)]
    pub end_opacity: f32,

    /// Number of animation steps (frames)
    #[arg(long, default_value_t = 30)]
    pub steps: u32,

    /// Duration of a frame in milliseconds
    #[arg(long, default_value_t = 20)]
    pub frame_time: u64,

    /// Easing function
    #[arg(long, value_enum, default_value_t = Easing::EaseInOutQuint)]
    pub ease: Easing,
}
