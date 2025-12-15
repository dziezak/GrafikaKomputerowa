use serde::{Serialize, Deserialize};
use crate::BgFitMode;

#[derive(Serialize, Deserialize)]
pub struct MyAppState {
    pub(crate) points: Vec<(f32, f32)>,
    pub(crate) wavelengths: Vec<f32>,
    pub(crate) current_xy: (f32, f32),
    pub(crate) current_rgb: (u8, u8, u8),

    pub(crate) control_points: Vec<(f32, f32)>,
    pub(crate) max_points: usize,

    pub(crate) xyz_samples: Vec<(f32, f32, f32, f32)>,

    #[serde(skip)]
    pub(crate) bg_mode: BgFitMode,
    pub(crate) bg_opacity: f32,

    pub(crate) curve: bool,
}
