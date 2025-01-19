use crate::utils::{Angle, HexVec};

use super::{
    Aabb, Anchor, Geometry, Mystic, Skeleton, Spectre, SpectreIter, SuperMystic, SuperSpectre,
};

pub enum SpectreLike {
    Spectre(Spectre),
    SuperSpectre(SuperSpectre),
    Skeleton(Skeleton),
}

impl Geometry for SpectreLike {
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            SpectreLike::Spectre(spectre) => spectre.point(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.point(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.edge_direction(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.rev_edge_direction(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.rev_edge_direction(anchor),
            SpectreLike::Skeleton(skeleton) => skeleton.rev_edge_direction(anchor),
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            SpectreLike::Spectre(spectre) => spectre.aabb(),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.aabb(),
            SpectreLike::Skeleton(skeleton) => skeleton.aabb(),
        }
    }
}

impl SpectreLike {
    pub fn adjacent_spectre_like(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreLike {
        match self {
            SpectreLike::Spectre(spectre) => {
                SpectreLike::Spectre(spectre.adjacent_spectre(from_anchor, to_anchor))
            }
            SpectreLike::SuperSpectre(super_spectre) => SpectreLike::SuperSpectre(
                super_spectre.adjacent_super_spectre(from_anchor, to_anchor),
            ),
            SpectreLike::Skeleton(skeleton) => {
                SpectreLike::Skeleton(skeleton.adjacent_skeleton(from_anchor, to_anchor))
            }
        }
    }

    pub fn into_mystic_like(self) -> MysticLike {
        match self {
            SpectreLike::Spectre(spectre) => MysticLike::Mystic(spectre.into_mystic()),
            SpectreLike::SuperSpectre(super_spectre) => {
                MysticLike::SuperMystic(super_spectre.into_super_mystic())
            }
            SpectreLike::Skeleton(skeleton) => MysticLike::Skeleton(skeleton),
        }
    }

    pub fn iter(&self, aabb: Aabb) -> SpectreIter {
        match self {
            SpectreLike::Spectre(_) => unimplemented!("SpectreLike::Spectre"),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.iter(aabb),
            SpectreLike::Skeleton(_) => unimplemented!("SpectreLike::Skeleton"),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            SpectreLike::Spectre(_) => 0,
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.level,
            SpectreLike::Skeleton(skeleton) => skeleton.level,
        }
    }

    pub fn update(&mut self, aabb: &Aabb) {
        match self {
            SpectreLike::Spectre(_) => {}
            SpectreLike::SuperSpectre(super_spectre) => {
                if super_spectre.aabb().has_intersection(aabb) {
                    super_spectre.update_children(aabb);
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
                if !skeleton.aabb().has_intersection(aabb) {
                    return;
                }
                let super_spectre = skeleton.to_super_spectre(aabb);
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

impl From<SuperSpectre> for SpectreLike {
    fn from(super_spectre: SuperSpectre) -> Self {
        SpectreLike::SuperSpectre(super_spectre)
    }
}

impl From<Skeleton> for SpectreLike {
    fn from(skeleton: Skeleton) -> Self {
        SpectreLike::Skeleton(skeleton)
    }
}

pub enum MysticLike {
    Mystic(Mystic),
    SuperMystic(SuperMystic),
    Skeleton(Skeleton),
}

impl Geometry for MysticLike {
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.point(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.point(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.edge_direction(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.rev_edge_direction(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.rev_edge_direction(anchor),
            MysticLike::Skeleton(skeleton) => skeleton.rev_edge_direction(anchor),
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            MysticLike::Mystic(mystic) => mystic.aabb(),
            MysticLike::SuperMystic(super_mystic) => super_mystic.aabb(),
            MysticLike::Skeleton(skeleton) => skeleton.aabb(),
        }
    }
}

impl MysticLike {
    pub fn update(&mut self, aabb: &Aabb) {
        match self {
            MysticLike::Mystic(_) => {}
            MysticLike::SuperMystic(super_mystic) => {
                if super_mystic.aabb().has_intersection(aabb) {
                    super_mystic.update_children(aabb);
                    return;
                }
                // super_mysticをskeletonにする
                *self = MysticLike::Skeleton(Skeleton::new_with_anchor(
                    super_mystic.level,
                    super_mystic.point(Anchor::Anchor1),
                    Anchor::Anchor1,
                    super_mystic.edge_direction(Anchor::Anchor1),
                ))
            }
            MysticLike::Skeleton(skeleton) => {
                if !skeleton.aabb().has_intersection(aabb) {
                    return;
                }
                let super_mystic = skeleton.to_super_spectre(aabb).into_super_mystic();
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

impl From<SuperMystic> for MysticLike {
    fn from(super_mystic: SuperMystic) -> Self {
        MysticLike::SuperMystic(super_mystic)
    }
}

impl From<Skeleton> for MysticLike {
    fn from(skeleton: Skeleton) -> Self {
        MysticLike::Skeleton(skeleton)
    }
}
