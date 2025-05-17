
use crate::UserPref;

pub(super) enum PencilStrokeWidthPref {}

impl UserPref for PencilStrokeWidthPref {
    type Type = f32;

    fn default() -> f32 {
        5.0
    }

    fn name() -> &'static str {
        "pencil_stroke_width"
    }
}

pub(super) enum PencilUsePressure {}

impl UserPref for PencilUsePressure {
    type Type = bool;

    fn default() -> bool {
        true
    }

    fn name() -> &'static str {
        "pencil_use_pressure"
    }
}

pub(super) enum StabilizationRadius {}

impl UserPref for StabilizationRadius {
    type Type = u32;

    fn default() -> Self::Type {
        0
    }

    fn name() -> &'static str {
        "stabilization_radius"
    }
}
