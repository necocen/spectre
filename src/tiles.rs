mod anchor;
mod bounds;
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

/// これより細かいClusterは必ずまとめてロードする
const MIN_PARTIAL_CLUSTER_LEVEL: usize = 4;

/// Supported substitution depth; larger clusters can overflow their i32 coordinates.
pub const MAX_CLUSTER_LEVEL: usize = 18;

// children[i] connects to children[(i + 1) % 8]. Shared by eager and lazy construction.
const EDGE_CHAIN: [(Anchor, Anchor); 8] = [
    (Anchor::Anchor3, Anchor::Anchor1),
    (Anchor::Anchor4, Anchor::Anchor2),
    (Anchor::Anchor3, Anchor::Anchor1),
    (Anchor::Anchor3, Anchor::Anchor1),
    (Anchor::Anchor4, Anchor::Anchor2),
    (Anchor::Anchor3, Anchor::Anchor1),
    (Anchor::Anchor4, Anchor::Anchor4),
    (Anchor::Anchor1, Anchor::Anchor1),
];
