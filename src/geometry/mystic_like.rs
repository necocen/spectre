use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Geometry, Mystic, MysticCluster, Skeleton};

pub enum MysticLike {
    Mystic(Mystic),
    Cluster(MysticCluster),
    Skeleton(Skeleton),
}

impl Geometry for MysticLike {
    fn coordinate(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.coordinate(anchor),
            MysticLike::Cluster(cluster) => cluster.coordinate(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.coordinate(anchor),
        }
    }

    fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_from(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_from(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_from(anchor),
        }
    }

    fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_into(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_into(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_into(anchor),
        }
    }

    fn bbox(&self) -> Aabb {
        match self {
            MysticLike::Mystic(mystic) => mystic.bbox(),
            MysticLike::Cluster(cluster) => cluster.bbox(),
            MysticLike::Skeleton(skeleton) => skeleton.bbox(),
        }
    }
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
                // super_mysticをskeletonにする
                *self = MysticLike::Skeleton(cluster.skeleton())
            }
            MysticLike::Skeleton(skeleton) => {
                if !skeleton.bbox().has_intersection(bbox) {
                    return;
                }
                let super_mystic = skeleton.to_spectre_cluster(bbox).into_mystic_cluster();
                *self = super_mystic.into();
            }
        }
    }
}

impl From<Mystic> for MysticLike {
    fn from(mystic: Mystic) -> Self {
        MysticLike::Mystic(mystic)
    }
}

impl From<MysticCluster> for MysticLike {
    fn from(super_mystic: MysticCluster) -> Self {
        MysticLike::Cluster(super_mystic)
    }
}

impl From<Skeleton> for MysticLike {
    fn from(skeleton: Skeleton) -> Self {
        MysticLike::Skeleton(skeleton)
    }
}
