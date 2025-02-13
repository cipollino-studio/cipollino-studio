
use crate::{Color, Id, Vec2, UI};

pub trait Animatable: Copy + Clone + Default {

    fn similar(&self, other: Self) -> bool;
    fn lerp(&self, other: Self, t: f32) -> Self;

}

#[derive(Default)]
struct AnimationState<T: Animatable>(T);

pub fn animate<T: Animatable + 'static>(ui: &mut UI, node: Id, target: T, rate: f32) -> T {
    if !ui.memory().has::<AnimationState<T>>(node) {
        ui.memory().get::<AnimationState<T>>(node).0 = target;
        return target;
    }

    let state = ui.memory().get::<AnimationState<T>>(node);
    state.0 = state.0.lerp(target, rate);
    let value = state.0;
    if !state.0.similar(target) {
        ui.request_redraw();
        value
    } else {
        state.0 = target;
        target
    }
}

impl Animatable for f32 {

    fn similar(&self, other: Self) -> bool {
        (*self - other).abs() < 0.005
    }

    fn lerp(&self, other: Self, t: f32) -> Self {
        *self + (other - *self) * t
    }

}

impl Animatable for Vec2 {

    fn similar(&self, other: Self) -> bool {
        self.distance(other) < 0.05
    }

    fn lerp(&self, other: Self, t: f32) -> Self {
        *self + (other - *self) * t
    }

}

impl Animatable for Color {

    fn similar(&self, other: Self) -> bool {
        self.r.similar(other.r) && self.g.similar(other.g) && self.b.similar(other.b) && self.a.similar(other.a)
    }

    fn lerp(&self, other: Self, t: f32) -> Self {
        Self::rgba(
            self.r.lerp(other.r, t), 
            self.g.lerp(other.g, t), 
            self.b.lerp(other.b, t),
            self.a.lerp(other.a, t)
        )
    }

}