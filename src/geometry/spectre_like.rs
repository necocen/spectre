use crate::utils::{Angle, HexVec};

use super::{Anchor, Geometry, Mystic, Spectre, SuperMystic, SuperSpectre};

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
    pub fn spectres(&self) -> Box<dyn Iterator<Item = &Spectre> + '_> {
        match self {
            SpectreLike::Spectre(spectre) => Box::new(std::iter::once(spectre)),
            SpectreLike::SuperSpectre(super_spectre) => Box::new(super_spectre.spectres()),
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
    pub fn spectres(&self) -> Box<dyn Iterator<Item = &Spectre> + '_> {
        match self {
            MysticLike::Mystic(mystic) => Box::new(mystic.spectres()),
            MysticLike::SuperMystic(super_mystic) => Box::new(super_mystic.spectres()),
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
