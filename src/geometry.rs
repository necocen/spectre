mod aabb;
mod anchor;
mod skeleton;
mod spectre;
mod spectre_iter;
mod spectre_like;
mod super_spectre;

pub use aabb::Aabb;
pub use anchor::Anchor;
pub use skeleton::Skeleton;
pub use spectre::{Mystic, Spectre};
pub use spectre_iter::{SpectreContainer, SpectreIter};
pub use spectre_like::{MysticLike, SpectreLike};
pub use super_spectre::{SuperMystic, SuperSpectre};

use crate::utils::{Angle, HexVec};

pub trait Geometry {
    fn point(&self, anchor: Anchor) -> HexVec;
    fn edge_direction(&self, anchor: Anchor) -> Angle;
    fn rev_edge_direction(&self, anchor: Anchor) -> Angle;
    fn aabb(&self) -> Aabb;
}
