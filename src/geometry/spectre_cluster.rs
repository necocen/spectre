use crate::{
    geometry::Skeleton,
    utils::{Aabb, Angle, HexVec},
};

use super::{
    Anchor, MysticCluster, MysticLike, SpectreIter, SpectreLike, MIN_PARTIAL_CLUSTER_LEVEL,
};

pub struct SpectreCluster {
    pub(super) a: Box<SpectreLike>,
    pub(super) b: Box<SpectreLike>,
    pub(super) c: Box<SpectreLike>,
    pub(super) d: Box<SpectreLike>,
    pub(super) e: Box<SpectreLike>,
    pub(super) f: Box<SpectreLike>,
    pub(super) g: Box<SpectreLike>,
    pub(super) h: Box<MysticLike>,
    level: usize,
    bbox: Aabb,
}

impl SpectreCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: impl Into<Box<SpectreLike>>,
        b: impl Into<Box<SpectreLike>>,
        c: impl Into<Box<SpectreLike>>,
        d: impl Into<Box<SpectreLike>>,
        e: impl Into<Box<SpectreLike>>,
        f: impl Into<Box<SpectreLike>>,
        g: impl Into<Box<SpectreLike>>,
        h: impl Into<Box<MysticLike>>,
        level: usize,
    ) -> Self {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        let d = d.into();
        let e = e.into();
        let f = f.into();
        let g = g.into();
        let h = h.into();
        assert_eq!(h.coordinate(Anchor::Anchor1), a.coordinate(Anchor::Anchor1));
        assert_eq!(a.coordinate(Anchor::Anchor3), b.coordinate(Anchor::Anchor1));
        assert_eq!(b.coordinate(Anchor::Anchor4), c.coordinate(Anchor::Anchor2));
        assert_eq!(c.coordinate(Anchor::Anchor3), d.coordinate(Anchor::Anchor1));
        assert_eq!(d.coordinate(Anchor::Anchor3), e.coordinate(Anchor::Anchor1));
        assert_eq!(e.coordinate(Anchor::Anchor4), f.coordinate(Anchor::Anchor2));
        assert_eq!(f.coordinate(Anchor::Anchor3), g.coordinate(Anchor::Anchor1));
        assert_eq!(g.coordinate(Anchor::Anchor4), h.coordinate(Anchor::Anchor4));

        // Calculate AABB only for existing parts
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&a.bbox());
        bbox = bbox.union(&b.bbox());
        bbox = bbox.union(&c.bbox());
        bbox = bbox.union(&d.bbox());
        bbox = bbox.union(&e.bbox());
        bbox = bbox.union(&f.bbox());
        bbox = bbox.union(&g.bbox());
        bbox = bbox.union(&h.bbox());

        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            level,
            bbox,
        }
    }

    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
        level: usize,
    ) -> Self {
        let edge_direction: Angle = edge_direction.into();
        let coordinate: HexVec = coordinate.into();
        match anchor {
            Anchor::Anchor1 => {
                let g = SpectreLike::with_anchor(
                    Anchor::Anchor3,
                    coordinate,
                    edge_direction,
                    level - 1,
                );
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
                Self::new(a, b, c, d, e, f, g, h, level)
            }
            Anchor::Anchor2 => {
                let d = SpectreLike::with_anchor(
                    Anchor::Anchor2,
                    coordinate,
                    edge_direction,
                    level - 1,
                );
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
                Self::new(a, b, c, d, e, f, g, h, level)
            }
            Anchor::Anchor3 => {
                let b = SpectreLike::with_anchor(
                    Anchor::Anchor3,
                    coordinate,
                    edge_direction,
                    level - 1,
                );
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let h = h.into_mystic_like();
                Self::new(a, b, c, d, e, f, g, h, level)
            }
            Anchor::Anchor4 => {
                let a = SpectreLike::with_anchor(
                    Anchor::Anchor2,
                    coordinate,
                    edge_direction,
                    level - 1,
                );
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let h = h.into_mystic_like();
                Self::new(a, b, c, d, e, f, g, h, level)
            }
        }
    }

    pub fn with_child_a(a: SpectreCluster) -> Self {
        let level = a.level() + 1;
        let a_skeleton = SpectreLike::from(a.skeleton());
        let a = SpectreLike::from(a);
        let b = a_skeleton.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
        let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
        let h = h.into_mystic_like();
        Self::new(a, b, c, d, e, f, g, h, level)
    }

    pub fn with_child_f(f: SpectreCluster) -> Self {
        let level = f.level() + 1;
        let f_skeleton = SpectreLike::from(f.skeleton());
        let f = SpectreLike::from(f);
        let g = f_skeleton.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
        let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
        let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
        let h = h.into_mystic_like();
        Self::new(a, b, c, d, e, f, g, h, level)
    }

    pub fn connected_cluster(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreCluster {
        // 新しいSpectreの角度を計算
        // levelによって頂点を合わせる場合に接合する辺の選びかたが変わる
        let angle = if self.level % 2 == 1 {
            // 奇数番目のlevelでは新しいClusterを辺が密着するまで時計回りに回転させる
            self.edge_direction_into(from_anchor).opposite()
        } else {
            // 偶数番目のlevelでは反時計回りに回転させる
            let rotation = self.edge_direction_from(to_anchor)
                - self.edge_direction_into(to_anchor).opposite();
            self.edge_direction_from(from_anchor) + rotation
        };

        SpectreCluster::with_anchor(to_anchor, self.coordinate(from_anchor), angle, self.level)
    }

    pub fn into_mystic_cluster(self) -> MysticCluster {
        MysticCluster::new(
            self.a, self.b, self.c, self.d, self.f, self.g, self.h, self.level,
        )
    }

    pub fn skeleton(&self) -> Skeleton {
        Skeleton::with_anchor(
            Anchor::Anchor1,
            self.coordinate(Anchor::Anchor1),
            self.edge_direction_from(Anchor::Anchor1),
            self.level,
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
        self.e.update(bbox);
        self.f.update(bbox);
        self.g.update(bbox);
        self.h.update(bbox);
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&self.a.bbox());
        bbox = bbox.union(&self.b.bbox());
        bbox = bbox.union(&self.c.bbox());
        bbox = bbox.union(&self.d.bbox());
        bbox = bbox.union(&self.e.bbox());
        bbox = bbox.union(&self.f.bbox());
        bbox = bbox.union(&self.g.bbox());
        bbox = bbox.union(&self.h.bbox());
        self.bbox = bbox;
    }

    pub fn spectres_in(&self, bbox: Aabb) -> SpectreIter<'_> {
        SpectreIter::new(self, bbox)
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
