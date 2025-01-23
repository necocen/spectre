use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, MysticLike, Skeleton, Spectre, SpectreCluster};

pub enum SpectreLike {
    Spectre(Spectre),
    Cluster(SpectreCluster),
    Skeleton(Skeleton),
}

impl SpectreLike {
    pub fn with_anchor(
        anchor: Anchor,
        coordinate: HexVec,
        edge_direction: Angle,
        level: usize,
    ) -> Self {
        if level == 0 {
            SpectreLike::from(Spectre::with_anchor(anchor, coordinate, edge_direction))
        } else {
            SpectreLike::from(SpectreCluster::with_anchor(
                anchor,
                coordinate,
                edge_direction,
                level,
            ))
        }
    }

    pub fn connected_spectre_like(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreLike {
        match self {
            SpectreLike::Spectre(spectre) => {
                SpectreLike::Spectre(spectre.connected_spectre(from_anchor, to_anchor))
            }
            SpectreLike::Cluster(cluster) => {
                SpectreLike::Cluster(cluster.connected_cluster(from_anchor, to_anchor))
            }
            SpectreLike::Skeleton(skeleton) => {
                SpectreLike::Skeleton(skeleton.connected_skeleton(from_anchor, to_anchor))
            }
        }
    }

    pub fn into_mystic_like(self) -> MysticLike {
        match self {
            SpectreLike::Spectre(spectre) => MysticLike::Mystic(spectre.into_mystic()),
            SpectreLike::Cluster(cluster) => MysticLike::Cluster(cluster.into_mystic_cluster()),
            SpectreLike::Skeleton(skeleton) => MysticLike::Skeleton(skeleton),
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        match self {
            SpectreLike::Spectre(_) => {}
            SpectreLike::Cluster(cluster) => {
                if cluster.bbox().has_intersection(bbox) {
                    cluster.update(bbox);
                    return;
                }
                // spectre_#lusterをskeletonにする
                *self = SpectreLike::Skeleton(cluster.to_skeleton());
            }
            SpectreLike::Skeleton(skeleton) => {
                if !skeleton.bbox().has_intersection(bbox) {
                    return;
                }
                let cluster = skeleton.to_spectre_cluster(bbox);
                *self = cluster.into();
            }
        }
    }

    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match self {
            SpectreLike::Spectre(spectre) => spectre.coordinate(anchor),
            SpectreLike::Cluster(cluster) => cluster.coordinate(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.coordinate(anchor),
        }
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction_from(anchor),
            SpectreLike::Cluster(cluster) => cluster.edge_direction_from(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction_from(anchor),
        }
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction_into(anchor),
            SpectreLike::Cluster(cluster) => cluster.edge_direction_into(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction_into(anchor),
        }
    }

    pub fn bbox(&self) -> Aabb {
        match self {
            SpectreLike::Spectre(spectre) => spectre.bbox(),
            SpectreLike::Cluster(cluster) => cluster.bbox(),
            SpectreLike::Skeleton(skeleton) => skeleton.bbox(),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            SpectreLike::Spectre(_) => 0,
            SpectreLike::Cluster(cluster) => cluster.level(),
            SpectreLike::Skeleton(skeleton) => skeleton.level(),
        }
    }
}

impl From<Spectre> for SpectreLike {
    fn from(spectre: Spectre) -> Self {
        SpectreLike::Spectre(spectre)
    }
}

impl From<SpectreCluster> for SpectreLike {
    fn from(cluster: SpectreCluster) -> Self {
        SpectreLike::Cluster(cluster)
    }
}

impl From<Skeleton> for SpectreLike {
    fn from(skeleton: Skeleton) -> Self {
        SpectreLike::Skeleton(skeleton)
    }
}
