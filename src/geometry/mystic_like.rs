use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Mystic, MysticCluster, Skeleton};

pub enum MysticLike {
    Mystic(Mystic),
    Cluster(MysticCluster),
    Skeleton(Skeleton),
}

impl MysticLike {
    pub fn update(&mut self, bbox: &Aabb) {
        match self {
            MysticLike::Mystic(_) => {}
            MysticLike::Cluster(cluster) => {
                if cluster.bbox().has_intersection(bbox) {
                    cluster.update(bbox);
                    return;
                }
                // mystic_clusterをskeletonにする
                *self = MysticLike::Skeleton(cluster.skeleton())
            }
            MysticLike::Skeleton(skeleton) => {
                if !skeleton.bbox().has_intersection(bbox) {
                    return;
                }
                let cluster = skeleton.to_spectre_cluster(bbox).into_mystic_cluster();
                *self = cluster.into();
            }
        }
    }

    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.coordinate(anchor),
            MysticLike::Cluster(cluster) => cluster.coordinate(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.coordinate(anchor),
        }
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_from(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_from(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_from(anchor),
        }
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_into(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_into(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_into(anchor),
        }
    }

    pub fn bbox(&self) -> Aabb {
        match self {
            MysticLike::Mystic(mystic) => mystic.bbox(),
            MysticLike::Cluster(cluster) => cluster.bbox(),
            MysticLike::Skeleton(skeleton) => skeleton.bbox(),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            MysticLike::Mystic(_) => 0,
            MysticLike::Cluster(cluster) => cluster.level(),
            MysticLike::Skeleton(skeleton) => skeleton.level(),
        }
    }
}

impl From<Mystic> for MysticLike {
    fn from(mystic: Mystic) -> Self {
        MysticLike::Mystic(mystic)
    }
}

impl From<MysticCluster> for MysticLike {
    fn from(cluster: MysticCluster) -> Self {
        MysticLike::Cluster(cluster)
    }
}

impl From<Skeleton> for MysticLike {
    fn from(skeleton: Skeleton) -> Self {
        MysticLike::Skeleton(skeleton)
    }
}
