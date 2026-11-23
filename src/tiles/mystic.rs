use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Spectre};

pub struct Mystic {
    lower: Spectre,
    upper: Spectre,
    bbox: Aabb,
}

impl Mystic {
    #[inline]
    pub fn new(lower: Spectre, upper: Spectre) -> Self {
        let bbox = lower.bbox().union(&upper.bbox());
        Self { lower, upper, bbox }
    }

    #[inline]
    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        self.lower.coordinate(anchor)
    }

    #[inline]
    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        self.lower.edge_direction_from(anchor)
    }

    #[inline]
    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        self.lower.edge_direction_into(anchor)
    }

    #[inline]
    pub fn bbox(&self) -> Aabb {
        self.bbox
    }

    #[inline]
    pub fn lower(&self) -> &Spectre {
        &self.lower
    }

    #[inline]
    pub fn upper(&self) -> &Spectre {
        &self.upper
    }
}
