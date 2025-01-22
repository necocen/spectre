use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Geometry, Spectre};

pub struct Mystic {
    pub lower: Spectre,
    pub upper: Spectre,
    bbox: Aabb,
}

impl Geometry for Mystic {
    fn coordinate(&self, anchor: Anchor) -> HexVec {
        self.lower.coordinate(anchor)
    }

    fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        self.lower.edge_direction_from(anchor)
    }

    fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        self.lower.edge_direction_into(anchor)
    }

    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

impl Mystic {
    pub fn new(lower: Spectre, upper: Spectre) -> Self {
        let bbox = lower.bbox().union(&upper.bbox());
        Self { lower, upper, bbox }
    }
}
