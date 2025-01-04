use bevy::math::Vec2;

use crate::{anchor::Anchor, angle::Angle, spectre::Spectre};

pub trait SpectreLike {
    fn anchor(&self, anchor: Anchor) -> Vec2;
    fn edge_direction(&self, anchor: Anchor) -> Angle;
    fn prev_edge_direction(&self, anchor: Anchor) -> Angle;
    fn spectres(&self) -> Box<dyn Iterator<Item = &Spectre> + '_>;
}

pub trait MysticLike {
    fn anchor(&self, anchor: Anchor) -> Vec2;
    fn spectres(&self) -> Box<dyn Iterator<Item = &Spectre> + '_>;
}
