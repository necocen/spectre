use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, Geometry, Mystic, MysticCluster, Skeleton, Spectre, SpectreCluster};

pub enum SpectreLike {
    Spectre(Spectre),
    Cluster(SpectreCluster),
    Skeleton(Skeleton),
}

impl Geometry for SpectreLike {
    fn coordinate(&self, anchor: Anchor) -> HexVec {
        match self {
            SpectreLike::Spectre(spectre) => spectre.coordinate(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.coordinate(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.coordinate(anchor),
        }
    }

    fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction_from(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.edge_direction_from(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction_from(anchor),
        }
    }

    fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction_into(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.edge_direction_into(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction_into(anchor),
        }
    }

    fn bbox(&self) -> Aabb {
        match self {
            SpectreLike::Spectre(spectre) => spectre.bbox(),
            SpectreLike::Cluster(super_spectre) => super_spectre.bbox(),
            SpectreLike::Skeleton(skeleton) => skeleton.bbox(),
        }
    }
}

impl SpectreLike {
    pub fn connected_spectre_like(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreLike {
        match self {
            SpectreLike::Spectre(spectre) => {
                SpectreLike::Spectre(spectre.connected_spectre(from_anchor, to_anchor))
            }
            SpectreLike::Cluster(super_spectre) => {
                SpectreLike::Cluster(super_spectre.connected_cluster(from_anchor, to_anchor))
            }
            SpectreLike::Skeleton(skeleton) => {
                SpectreLike::Skeleton(skeleton.connected_skeleton(from_anchor, to_anchor))
            }
        }
    }

    pub fn into_mystic_like(self) -> MysticLike {
        match self {
            SpectreLike::Spectre(spectre) => MysticLike::Mystic(spectre.into_mystic()),
            SpectreLike::Cluster(super_spectre) => {
                MysticLike::Cluster(super_spectre.into_mystic_cluster())
            }
            SpectreLike::Skeleton(skeleton) => MysticLike::Skeleton(skeleton),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            SpectreLike::Spectre(_) => 0,
            SpectreLike::Cluster(cluster) => cluster.level,
            SpectreLike::Skeleton(skeleton) => skeleton.level,
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
                // super_spectreをskeletonにする
                *self = SpectreLike::Skeleton(cluster.skeleton());
            }
            SpectreLike::Skeleton(skeleton) => {
                if !skeleton.bbox().has_intersection(bbox) {
                    return;
                }
                let super_spectre = skeleton.to_spectre_cluster(bbox);
                *self = super_spectre.into();
            }
        }
    }
}

impl From<Spectre> for SpectreLike {
    fn from(spectre: Spectre) -> Self {
        SpectreLike::Spectre(spectre)
    }
}

impl From<SpectreCluster> for SpectreLike {
    fn from(super_spectre: SpectreCluster) -> Self {
        SpectreLike::Cluster(super_spectre)
    }
}

impl From<Skeleton> for SpectreLike {
    fn from(skeleton: Skeleton) -> Self {
        SpectreLike::Skeleton(skeleton)
    }
}

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
