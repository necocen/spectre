use crate::{
    geometry::Skeleton,
    utils::{Aabb, Angle, HexVec},
};

use super::{
    Anchor, MysticCluster, MysticLike, Spectre, SpectreContainer, SpectreIter, SpectreLike,
    MIN_PARTIAL_SUPER_SPECTRE_LEVEL,
};

pub struct SpectreCluster {
    a: Box<SpectreLike>,
    b: Box<SpectreLike>,
    c: Box<SpectreLike>,
    d: Box<SpectreLike>,
    e: Box<SpectreLike>,
    f: Box<SpectreLike>,
    g: Box<SpectreLike>,
    h: Box<MysticLike>,
    level: usize,
    bbox: Aabb,
}

impl SpectreCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: Box<SpectreLike>,
        b: Box<SpectreLike>,
        c: Box<SpectreLike>,
        d: Box<SpectreLike>,
        e: Box<SpectreLike>,
        f: Box<SpectreLike>,
        g: Box<SpectreLike>,
        h: Box<MysticLike>,
        level: usize,
    ) -> Self {
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
        let (a, b, c, d, e, f, g, h) = match anchor {
            Anchor::Anchor1 => {
                let g = if level == 1 {
                    SpectreLike::from(Spectre::with_anchor(
                        Anchor::Anchor3,
                        coordinate,
                        edge_direction,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::with_anchor(
                        Anchor::Anchor3,
                        coordinate,
                        edge_direction,
                        level - 1,
                    ))
                };
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
                (a, b, c, d, e, f, g, h)
            }
            Anchor::Anchor2 => {
                let d = if level == 1 {
                    SpectreLike::from(Spectre::with_anchor(
                        Anchor::Anchor2,
                        coordinate,
                        edge_direction,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::with_anchor(
                        Anchor::Anchor2,
                        coordinate,
                        edge_direction,
                        level - 1,
                    ))
                };
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
                (a, b, c, d, e, f, g, h)
            }
            Anchor::Anchor3 => {
                let b = if level == 1 {
                    SpectreLike::from(Spectre::with_anchor(
                        Anchor::Anchor3,
                        coordinate,
                        edge_direction,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::with_anchor(
                        Anchor::Anchor3,
                        coordinate,
                        edge_direction,
                        level - 1,
                    ))
                };
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let h = h.into_mystic_like();
                (a, b, c, d, e, f, g, h)
            }
            Anchor::Anchor4 => {
                let a = if level == 1 {
                    SpectreLike::from(Spectre::with_anchor(
                        Anchor::Anchor2,
                        coordinate,
                        edge_direction,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::with_anchor(
                        Anchor::Anchor2,
                        coordinate,
                        edge_direction,
                        level - 1,
                    ))
                };
                let b = a.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let h = h.into_mystic_like();
                (a, b, c, d, e, f, g, h)
            }
        };

        Self::new(
            Box::new(a),
            Box::new(b),
            Box::new(c),
            Box::new(d),
            Box::new(e),
            Box::new(f),
            Box::new(g),
            Box::new(h),
            level,
        )
    }

    pub fn with_child_a(a: SpectreCluster) -> Self {
        let a_skeleton = a.skeleton();
        let b = a_skeleton.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let f = e.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let g = f.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);
        Self::new(
            Box::new(a.into()),
            Box::new(b.into()),
            Box::new(c.into()),
            Box::new(d.into()),
            Box::new(e.into()),
            Box::new(f.into()),
            Box::new(g.into()),
            Box::new(h.into()),
            a_skeleton.level() + 1,
        )
    }

    pub fn with_child_f(f: SpectreCluster) -> Self {
        let f_skeleton = f.skeleton();
        let g = f_skeleton.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);
        let a = h.connected_skeleton(Anchor::Anchor1, Anchor::Anchor1);
        let b = a.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        Self::new(
            Box::new(a.into()),
            Box::new(b.into()),
            Box::new(c.into()),
            Box::new(d.into()),
            Box::new(e.into()),
            Box::new(f.into()),
            Box::new(g.into()),
            Box::new(h.into()),
            f_skeleton.level() + 1,
        )
    }

    pub fn connected_cluster(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreCluster {
        // 新しいSpectreの角度を計算
        // levelによって頂点を合わせる場合に接合する辺の選びかたが変わる
        let angle = if self.level % 2 == 1 {
            // 奇数番目のlevelでは新しいSuperSpectreを辺が密着するまで時計回りに回転させる
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

impl SpectreContainer for SpectreCluster {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike> {
        match index {
            0 => Some(&self.a),
            1 => Some(&self.b),
            2 => Some(&self.c),
            3 => Some(&self.d),
            4 => Some(&self.e),
            5 => Some(&self.f),
            6 => Some(&self.g),
            _ => None,
        }
    }

    fn get_mystic(&self) -> Option<&MysticLike> {
        Some(&self.h)
    }

    fn max_index(&self) -> usize {
        7
    }

    fn level(&self) -> usize {
        self.level
    }
}
