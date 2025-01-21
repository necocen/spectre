mod anchor;
mod cluster;
mod skeleton;
mod spectre;
mod spectre_iter;
mod spectre_like;

pub use anchor::Anchor;
pub use cluster::{MysticCluster, SpectreCluster};
pub use skeleton::Skeleton;
pub use spectre::{Mystic, Spectre};
pub use spectre_iter::{SpectreContainer, SpectreIter};
pub use spectre_like::{MysticLike, SpectreLike};

use crate::utils::{Angle, HexVec};

pub trait Geometry {
    fn coordinate(&self, anchor: Anchor) -> HexVec;
    fn edge_direction_from(&self, anchor: Anchor) -> Angle;
    fn edge_direction_into(&self, anchor: Anchor) -> Angle;
    fn bbox(&self) -> crate::utils::Aabb;
}

/// これより細かいSuperSpectreは必ずまとめてロードする
const MIN_PARTIAL_SUPER_SPECTRE_LEVEL: usize = 4;
