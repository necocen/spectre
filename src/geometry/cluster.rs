use crate::{
    geometry::Skeleton,
    utils::{Aabb, Angle, HexVec},
};

use super::{
    Anchor, Geometry, MysticLike, Spectre, SpectreContainer, SpectreIter, SpectreLike,
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
    pub level: usize,
    bbox: Aabb,
}

impl Geometry for SpectreCluster {
    fn point(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.point(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.point(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.point(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.point(Anchor::Anchor2),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction(Anchor::Anchor2),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.rev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.rev_edge_direction(Anchor::Anchor2),
        }
    }

    fn bbox(&self) -> Aabb {
        self.bbox
    }
}

impl SpectreCluster {
    pub fn iter(&self, bbox: Aabb) -> SpectreIter<'_> {
        SpectreIter {
            parents: vec![(self, 0)],
            bbox,
        }
    }

    pub fn spectres_in<'a>(&'a self, bbox: Aabb) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        if !self.bbox().has_intersection(&bbox) {
            return Box::new(std::iter::empty());
        }
        Box::new(self.iter(bbox))
    }

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
        assert_eq!(h.point(Anchor::Anchor1), a.point(Anchor::Anchor1));
        assert_eq!(a.point(Anchor::Anchor3), b.point(Anchor::Anchor1));
        assert_eq!(b.point(Anchor::Anchor4), c.point(Anchor::Anchor2));
        assert_eq!(c.point(Anchor::Anchor3), d.point(Anchor::Anchor1));
        assert_eq!(d.point(Anchor::Anchor3), e.point(Anchor::Anchor1));
        assert_eq!(e.point(Anchor::Anchor4), f.point(Anchor::Anchor2));
        assert_eq!(f.point(Anchor::Anchor3), g.point(Anchor::Anchor1));
        assert_eq!(g.point(Anchor::Anchor4), h.point(Anchor::Anchor4));

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

    pub fn from_child_a(a: SpectreCluster) -> Self {
        let a_skeleton = Skeleton::new_with_anchor(
            a.level,
            a.point(Anchor::Anchor1),
            Anchor::Anchor1,
            a.edge_direction(Anchor::Anchor1),
        );
        let b = a_skeleton.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let g = f.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
        Self::new(
            Box::new(a.into()),
            Box::new(b.into()),
            Box::new(c.into()),
            Box::new(d.into()),
            Box::new(e.into()),
            Box::new(f.into()),
            Box::new(g.into()),
            Box::new(h.into()),
            a_skeleton.level + 1,
        )
    }

    pub fn from_child_f(f: SpectreCluster) -> Self {
        let f_skeleton = Skeleton::new_with_anchor(
            f.level,
            f.point(Anchor::Anchor1),
            Anchor::Anchor1,
            f.edge_direction(Anchor::Anchor1),
        );
        let g = f_skeleton.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
        let a = h.adjacent_skeleton(Anchor::Anchor1, Anchor::Anchor1);
        let b = a.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        Self::new(
            Box::new(a.into()),
            Box::new(b.into()),
            Box::new(c.into()),
            Box::new(d.into()),
            Box::new(e.into()),
            Box::new(f.into()),
            Box::new(g.into()),
            Box::new(h.into()),
            f_skeleton.level + 1,
        )
    }

    pub fn new_with_anchor(
        level: usize,
        anchor_point: impl Into<HexVec>,
        anchor: Anchor,
        angle: impl Into<Angle>,
    ) -> Self {
        let angle: Angle = angle.into();
        match anchor {
            Anchor::Anchor1 => {
                let g = if level == 1 {
                    SpectreLike::from(Spectre::new_with_anchor(
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    ))
                };
                let h = g.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
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
            Anchor::Anchor2 => {
                let d = if level == 1 {
                    SpectreLike::from(Spectre::new_with_anchor(
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    ))
                };
                let e = d.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let h = h.into_mystic_like();
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
            Anchor::Anchor3 => {
                let b = if level == 1 {
                    SpectreLike::from(Spectre::new_with_anchor(
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    ))
                };
                let c = b.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_spectre_like(Anchor::Anchor1, Anchor::Anchor1);
                let h = h.into_mystic_like();
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
            Anchor::Anchor4 => {
                let a = if level == 1 {
                    SpectreLike::from(Spectre::new_with_anchor(
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    ))
                } else {
                    SpectreLike::from(SpectreCluster::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    ))
                };
                let b = a.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_spectre_like(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.adjacent_spectre_like(Anchor::Anchor4, Anchor::Anchor4);
                let h = h.into_mystic_like();
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
        }
    }

    pub fn adjacent_super_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreCluster {
        // 新しいSpectreの角度を計算
        // levelによって頂点を合わせる場合に接合する辺の選びかたが変わる
        let angle = if self.level % 2 == 1 {
            // 奇数番目のlevelでは新しいSuperSpectreを辺が密着するまで時計回りに回転させる
            self.rev_edge_direction(from_anchor).opposite()
        } else {
            // 偶数番目のlevelでは反時計回りに回転させる
            let rotation =
                self.edge_direction(to_anchor) - self.rev_edge_direction(to_anchor).opposite();
            self.edge_direction(from_anchor) + rotation
        };

        SpectreCluster::new_with_anchor(self.level, self.point(from_anchor), to_anchor, angle)
    }

    pub fn into_mystic_cluster(self) -> MysticCluster {
        // Calculate AABB only for existing parts
        let mut bbox = Aabb::NULL;
        bbox = bbox.union(&self.a.bbox());
        bbox = bbox.union(&self.b.bbox());
        bbox = bbox.union(&self.c.bbox());
        bbox = bbox.union(&self.d.bbox());
        bbox = bbox.union(&self.f.bbox());
        bbox = bbox.union(&self.g.bbox());
        bbox = bbox.union(&self.h.bbox());

        MysticCluster {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            f: self.f,
            g: self.g,
            h: self.h,
            level: self.level,
            bbox,
        }
    }

    pub fn update_children(&mut self, bbox: &Aabb) {
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
}

pub struct MysticCluster {
    a: Box<SpectreLike>,
    b: Box<SpectreLike>,
    c: Box<SpectreLike>,
    d: Box<SpectreLike>,
    f: Box<SpectreLike>,
    g: Box<SpectreLike>,
    h: Box<MysticLike>,
    pub level: usize,
    bbox: Aabb,
}

impl MysticCluster {
    pub fn update_children(&mut self, bbox: &Aabb) {
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
}

impl Geometry for MysticCluster {
    fn point(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.point(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.point(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.point(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.point(Anchor::Anchor2),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction(Anchor::Anchor2),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.rev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.rev_edge_direction(Anchor::Anchor2),
        }
    }

    fn bbox(&self) -> Aabb {
        self.bbox
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
