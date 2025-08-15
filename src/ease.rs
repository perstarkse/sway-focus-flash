use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Easing {
    Linear,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutQuint,
    EaseInOutQuint,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => {
                let p = t - 1.0;
                p * p * p + 1.0
            }
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let p = 2.0 * t - 2.0;
                    0.5 * p * p * p + 1.0
                }
            }
            Self::EaseOutQuint => {
                let p = t - 1.0;
                p * p * p * p * p + 1.0
            }
            Self::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let p = 2.0 * t - 2.0;
                    0.5 * p * p * p * p * p + 1.0
                }
            }
        }
    }
}
