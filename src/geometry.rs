use crate::{anchor::Anchor, angle::Angle, hex_vec::HexVec};

pub trait Geometry {
    fn anchor(&self, anchor: Anchor) -> HexVec;
    fn edge_direction(&self, anchor: Anchor) -> Angle;
    fn prev_edge_direction(&self, anchor: Anchor) -> Angle;
}
