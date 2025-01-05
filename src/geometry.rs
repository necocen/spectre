mod anchor;
mod spectre;
mod spectre_like;
mod super_spectre;

pub use anchor::Anchor;
pub use spectre::{Mystic, Spectre};
pub use spectre_like::{MysticLike, SpectreLike};
pub use super_spectre::{SuperMystic, SuperSpectre};

use crate::utils::{Angle, HexVec};

pub trait Geometry {
    fn anchor(&self, anchor: Anchor) -> HexVec;
    fn edge_direction(&self, anchor: Anchor) -> Angle;
    fn prev_edge_direction(&self, anchor: Anchor) -> Angle;
}
