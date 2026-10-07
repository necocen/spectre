use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, MIN_PARTIAL_CLUSTER_LEVEL, MysticLike, Skeleton, SpectreLike};

pub struct MysticCluster {
    pub(super) a: SpectreLike,
    pub(super) b: SpectreLike,
    pub(super) c: SpectreLike,
    pub(super) d: SpectreLike,
    pub(super) f: SpectreLike,
    pub(super) g: SpectreLike,
    pub(super) h: MysticLike,
    level: usize,
    bbox: Aabb,
}

impl MysticCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: impl Into<SpectreLike>,
        b: impl Into<SpectreLike>,
        c: impl Into<SpectreLike>,
        d: impl Into<SpectreLike>,
        f: impl Into<SpectreLike>,
        g: impl Into<SpectreLike>,
        h: impl Into<MysticLike>,
        level: usize,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        let d = d.into();
        let f = f.into();
        let g = g.into();
        let h = h.into();
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
