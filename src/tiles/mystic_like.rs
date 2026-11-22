use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Mystic, MysticCluster, Skeleton, Spectre};

pub enum MysticLike {
    Mystic(Mystic),
    Cluster(Box<MysticCluster>),
    Skeleton(Skeleton),
}

impl MysticLike {
    #[inline]
    pub(crate) fn from_anchor1(anchor1: HexVec, direction: Angle, level: usize) -> Self {
        if level == 0 {
            MysticLike::Mystic(Spectre::from_anchor1(anchor1, direction).into_mystic())
        } else {
            MysticLike::from(MysticCluster::from_anchor1(anchor1, direction, level))
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        match self {
            MysticLike::Mystic(_) => {}
            MysticLike::Cluster(cluster) => {
                if cluster.bbox().has_intersection(bbox) {
                    cluster.update(bbox);
                    return;
                }
                // mystic_clusterをskeletonにする
                *self = cluster.to_skeleton().into();
            }
            MysticLike::Skeleton(skeleton) => {
                if !skeleton.estimated_mystic_bbox().has_intersection(bbox) {
                    return;
                }
                let cluster = skeleton.to_mystic_cluster(bbox);
                *self = cluster.into();
            }
        }
    }

    #[inline]
    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.coordinate(anchor),
            MysticLike::Cluster(cluster) => cluster.coordinate(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.coordinate(anchor),
        }
    }

    #[inline]
    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_from(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_from(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_from(anchor),
        }
    }

    #[inline]
    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction_into(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction_into(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction_into(anchor),
        }
    }

    #[inline]
    pub fn bbox(&self) -> Aabb {
        match self {
            MysticLike::Mystic(mystic) => mystic.bbox(),
            MysticLike::Cluster(cluster) => cluster.bbox(),
            MysticLike::Skeleton(skeleton) => skeleton.estimated_mystic_bbox(),
        }
    }

    #[inline]
    pub fn level(&self) -> usize {
        match self {
            MysticLike::Mystic(_) => 0,
            MysticLike::Cluster(cluster) => cluster.level(),
            MysticLike::Skeleton(skeleton) => skeleton.level(),
        }
    }
}

impl From<Mystic> for MysticLike {
    #[inline]
    fn from(mystic: Mystic) -> Self {
        MysticLike::Mystic(mystic)
    }
}

impl From<MysticCluster> for MysticLike {
    #[inline]
    fn from(cluster: MysticCluster) -> Self {
        MysticLike::Cluster(Box::new(cluster))
    }
}

impl From<Skeleton> for MysticLike {
    #[inline]
    fn from(skeleton: Skeleton) -> Self {
        MysticLike::Skeleton(skeleton)
    }
}

impl From<Box<MysticLike>> for MysticLike {
    #[inline]
    fn from(value: Box<MysticLike>) -> Self {
        *value
    }
}
