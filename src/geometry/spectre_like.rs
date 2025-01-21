use crate::utils::{Aabb, Angle, HexVec};

use super::{
    Anchor, Geometry, Mystic, MysticCluster, Skeleton, Spectre, SpectreCluster, SpectreIter,
};

pub enum SpectreLike {
    Spectre(Spectre),
    Cluster(SpectreCluster),
    Skeleton(Skeleton),
}

impl Geometry for SpectreLike {
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            SpectreLike::Spectre(spectre) => spectre.point(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.point(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.edge_direction(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.rev_edge_direction(anchor),
            SpectreLike::Cluster(super_spectre) => super_spectre.rev_edge_direction(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.rev_edge_direction(anchor),
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
    pub fn adjacent_spectre_like(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreLike {
        match self {
            SpectreLike::Spectre(spectre) => {
                SpectreLike::Spectre(spectre.adjacent_spectre(from_anchor, to_anchor))
            }
            SpectreLike::Cluster(super_spectre) => {
                SpectreLike::Cluster(super_spectre.adjacent_super_spectre(from_anchor, to_anchor))
            }
            SpectreLike::Skeleton(skeleton) => {
                SpectreLike::Skeleton(skeleton.adjacent_skeleton(from_anchor, to_anchor))
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

    pub fn iter(&self, bbox: Aabb) -> SpectreIter {
        match self {
            SpectreLike::Spectre(_) => unimplemented!("SpectreLike::Spectre"),
            SpectreLike::Cluster(super_spectre) => super_spectre.iter(bbox),
            SpectreLike::Skeleton(_) => unimplemented!("SpectreLike::Skeleton"),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            SpectreLike::Spectre(_) => 0,
            SpectreLike::Cluster(super_spectre) => super_spectre.level,
            SpectreLike::Skeleton(skeleton) => skeleton.level,
        }
    }

    pub fn update(&mut self, bbox: &Aabb) {
        match self {
            SpectreLike::Spectre(_) => {}
            SpectreLike::Cluster(super_spectre) => {
                if super_spectre.bbox().has_intersection(bbox) {
                    super_spectre.update_children(bbox);
                    return;
                }
                // super_spectreをskeletonにする
                *self = SpectreLike::Skeleton(Skeleton::new_with_anchor(
                    super_spectre.level,
                    super_spectre.point(Anchor::Anchor1),
                    Anchor::Anchor1,
                    super_spectre.edge_direction(Anchor::Anchor1),
                ));
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
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.point(anchor),
            MysticLike::Cluster(cluster) => cluster.point(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction(anchor),
            MysticLike::Cluster(cluster) => cluster.edge_direction(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.rev_edge_direction(anchor),
            MysticLike::Cluster(cluster) => cluster.rev_edge_direction(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.rev_edge_direction(anchor),
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
                    cluster.update_children(bbox);
                    return;
                }
                // super_mysticをskeletonにする
                *self = MysticLike::Skeleton(Skeleton::new_with_anchor(
                    cluster.level,
                    cluster.point(Anchor::Anchor1),
                    Anchor::Anchor1,
                    cluster.edge_direction(Anchor::Anchor1),
                ))
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
