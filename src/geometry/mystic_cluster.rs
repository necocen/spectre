use crate::{
    geometry::Skeleton,
    utils::{Aabb, Angle, HexVec},
};

use super::{Anchor, MysticLike, SpectreContainer, SpectreLike, MIN_PARTIAL_SUPER_SPECTRE_LEVEL};

pub struct MysticCluster {
    a: Box<SpectreLike>,
    b: Box<SpectreLike>,
    c: Box<SpectreLike>,
    d: Box<SpectreLike>,
    f: Box<SpectreLike>,
    g: Box<SpectreLike>,
    h: Box<MysticLike>,
    level: usize,
    bbox: Aabb,
}

impl MysticCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: Box<SpectreLike>,
        b: Box<SpectreLike>,
        c: Box<SpectreLike>,
        d: Box<SpectreLike>,
        f: Box<SpectreLike>,
        g: Box<SpectreLike>,
        h: Box<MysticLike>,
        level: usize,
    ) -> Self {
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&a.bbox());
        bbox = bbox.union(&b.bbox());
        bbox = bbox.union(&c.bbox());
        bbox = bbox.union(&d.bbox());
        bbox = bbox.union(&f.bbox());
        bbox = bbox.union(&g.bbox());
        bbox = bbox.union(&h.bbox());
        Self {
            a,
            b,
            c,
            d,
            f,
            g,
            h,
            level,
            bbox,
        }
    }

    pub fn skeleton(&self) -> Skeleton {
        Skeleton::with_anchor(
            Anchor::Anchor1,
            self.g.coordinate(Anchor::Anchor1),
            self.g.edge_direction_from(Anchor::Anchor1),
            self.level,
        )
    }

    pub fn update(&mut self, bbox: &Aabb) {
        if self.level < MIN_PARTIAL_SUPER_SPECTRE_LEVEL {
            return;
        }
        self.a.update(bbox);
        self.b.update(bbox);
        self.c.update(bbox);
        self.d.update(bbox);
        self.f.update(bbox);
        self.g.update(bbox);
        self.h.update(bbox);
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&self.a.bbox());
        bbox = bbox.union(&self.b.bbox());
        bbox = bbox.union(&self.c.bbox());
        bbox = bbox.union(&self.d.bbox());
        bbox = bbox.union(&self.f.bbox());
        bbox = bbox.union(&self.g.bbox());
        bbox = bbox.union(&self.h.bbox());
        self.bbox = bbox;
    }

    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.coordinate(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.coordinate(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.coordinate(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.coordinate(Anchor::Anchor2),
        }
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction_from(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction_from(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction_from(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction_from(Anchor::Anchor2),
        }
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction_into(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction_into(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction_into(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction_into(Anchor::Anchor2),
        }
    }

    pub fn bbox(&self) -> Aabb {
        self.bbox
    }
}

impl SpectreContainer for MysticCluster {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike> {
        match index {
            0 => Some(&self.a),
            1 => Some(&self.b),
            2 => Some(&self.c),
            3 => Some(&self.d),
            4 => Some(&self.f),
            5 => Some(&self.g),
            _ => None,
        }
    }

    fn get_mystic(&self) -> Option<&MysticLike> {
        Some(&self.h)
    }

    fn max_index(&self) -> usize {
        6
    }

    fn level(&self) -> usize {
        self.level
    }
}
