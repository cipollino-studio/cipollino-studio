
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Axis {
    X,
    Y
}

pub const AXES: [Axis; 2] = [Axis::X, Axis::Y];

impl Axis {

    pub fn other(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PerAxis<T> {
    pub x: T,
    pub y: T
}

impl<T> PerAxis<T> {

    pub const fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }

    pub const fn along_across(axis: Axis, along: T, across: T) -> Self {
        match axis {
            Axis::X => Self::new(along, across),
            Axis::Y => Self::new(across, along)
        }
    }

    pub fn splat(val: T) -> Self where T: Clone {
        Self {
            x: val.clone(),
            y: val 
        }
    }

    pub fn on_axis(&self, axis: Axis) -> &T {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        } 
    }

}
