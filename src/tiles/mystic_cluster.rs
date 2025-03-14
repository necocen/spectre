use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, MysticLike, Skeleton, SpectreLike, MIN_PARTIAL_CLUSTER_LEVEL};

pub struct MysticCluster {
    pub(super) a: Box<SpectreLike>,
    pub(super) b: Box<SpectreLike>,
    pub(super) c: Box<SpectreLike>,
    pub(super) d: Box<SpectreLike>,
    pub(super) f: Box<SpectreLike>,
    pub(super) g: Box<SpectreLike>,
    pub(super) h: Box<MysticLike>,
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

    pub fn to_skeleton(&self) -> Skeleton {
        Skeleton::with_anchor(
            Anchor::Anchor1,
            self.coordinate(Anchor::Anchor1),
            self.edge_direction_from(Anchor::Anchor1),
            self.level,
            Some(self.bbox()),
        )
    }

    pub fn update(&mut self, bbox: &Aabb) {
        if self.level < MIN_PARTIAL_CLUSTER_LEVEL {
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

    pub fn level(&self) -> usize {
        self.level
    }
}
