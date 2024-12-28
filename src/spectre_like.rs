use bevy::math::Vec2;

use crate::{anchor::Anchor, angle::Angle, spectre::Spectre};

pub trait SpectreLike {
    fn size(&self) -> f32;
    fn anchor(&self, anchor: Anchor) -> Vec2;
    fn edge_direction(&self, anchor: Anchor) -> Angle;
    fn prev_edge_direction(&self, anchor: Anchor) -> Angle;
    fn to_mystic_like(self) -> Box<dyn MysticLike>
    where
        Self: Sized;
    fn spectres(&self) -> Vec<&Spectre>;
}

pub trait MysticLike {
    fn size(&self) -> f32;
    fn anchor(&self, anchor: Anchor) -> Vec2;
    fn spectres(&self) -> Vec<&Spectre>;
}
