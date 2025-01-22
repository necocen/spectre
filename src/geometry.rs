mod anchor;
mod mystic;
mod mystic_cluster;
mod mystic_like;
mod skeleton;
mod spectre;
mod spectre_cluster;
mod spectre_iter;
mod spectre_like;

pub use anchor::Anchor;
pub use mystic::Mystic;
pub use mystic_cluster::MysticCluster;
pub use mystic_like::MysticLike;
pub use skeleton::Skeleton;
pub use spectre::Spectre;
pub use spectre_cluster::SpectreCluster;
pub use spectre_iter::SpectreIter;
pub use spectre_like::SpectreLike;

/// これより細かいSuperSpectreは必ずまとめてロードする
const MIN_PARTIAL_SUPER_SPECTRE_LEVEL: usize = 4;
