use crate::utils::{Angle, HexVec};

use super::{Aabb, Anchor, Geometry, Mystic, Spectre, SuperMystic, SuperSpectre};

pub enum SpectreLike {
    Spectre(Spectre),
    SuperSpectre(Box<SuperSpectre>),
}

impl Geometry for SpectreLike {
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            SpectreLike::Spectre(spectre) => spectre.point(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.edge_direction(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            SpectreLike::Spectre(spectre) => spectre.rev_edge_direction(anchor),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.rev_edge_direction(anchor),
        }
    }
}

impl SpectreLike {
    pub fn adjacent_spectre_like(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreLike {
        match self {
            SpectreLike::Spectre(spectre) => {
                SpectreLike::Spectre(spectre.adjacent_spectre(from_anchor, to_anchor))
            }
            SpectreLike::SuperSpectre(super_spectre) => SpectreLike::SuperSpectre(Box::new(
                super_spectre.adjacent_super_spectre(from_anchor, to_anchor),
            )),
        }
    }

    pub fn into_mystic_like(self) -> MysticLike {
        match self {
            SpectreLike::Spectre(spectre) => MysticLike::Mystic(spectre.into_mystic()),
            SpectreLike::SuperSpectre(super_spectre) => {
                MysticLike::SuperMystic(Box::new(super_spectre.into_super_mystic()))
            }
        }
    }

    pub fn aabb(&self) -> Aabb {
        match self {
            SpectreLike::Spectre(spectre) => spectre.aabb,
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.aabb,
        }
    }

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        match self {
            SpectreLike::Spectre(spectre) => spectre.has_intersection(aabb),
            SpectreLike::SuperSpectre(super_spectre) => super_spectre.has_intersection(aabb),
        }
    }

    pub fn spectres_in<'a, 'b: 'a>(
        &'a self,
        aabb: &'b Aabb,
    ) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        match self {
            SpectreLike::Spectre(spectre) => {
                if spectre.has_intersection(aabb) {
                    Box::new(std::iter::once(spectre))
                } else {
                    Box::new(std::iter::empty())
                }
            }
            SpectreLike::SuperSpectre(super_spectre) => Box::new(super_spectre.spectres_in(aabb)),
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
        SpectreLike::SuperSpectre(Box::new(super_spectre))
    }
}

pub enum MysticLike {
    Mystic(Mystic),
    SuperMystic(Box<SuperMystic>),
}

impl Geometry for MysticLike {
    fn point(&self, anchor: Anchor) -> HexVec {
        match self {
            MysticLike::Mystic(mystic) => mystic.point(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.point(anchor),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.edge_direction(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.edge_direction(anchor),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match self {
            MysticLike::Mystic(mystic) => mystic.rev_edge_direction(anchor),
            MysticLike::SuperMystic(super_mystic) => super_mystic.rev_edge_direction(anchor),
        }
    }
}

impl MysticLike {
    pub fn aabb(&self) -> Aabb {
        match self {
            MysticLike::Mystic(mystic) => mystic.aabb,
            MysticLike::SuperMystic(super_mystic) => super_mystic.aabb,
        }
    }

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        match self {
            MysticLike::Mystic(mystic) => mystic.has_intersection(aabb),
            MysticLike::SuperMystic(super_mystic) => super_mystic.has_intersection(aabb),
        }
    }

    pub fn spectres_in<'a, 'b: 'a>(
        &'a self,
        aabb: &'b Aabb,
    ) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        match self {
            MysticLike::Mystic(mystic) => Box::new(mystic.spectres_in(aabb)),
            MysticLike::SuperMystic(super_mystic) => Box::new(super_mystic.spectres_in(aabb)),
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
        MysticLike::SuperMystic(Box::new(super_mystic))
    }
}
