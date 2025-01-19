use crate::{
    geometry::Skeleton,
    utils::{Angle, HexVec},
};

use super::{
    Aabb, Anchor, Geometry, MysticLike, Spectre, SpectreContainer, SpectreIter, SpectreLike,
    MIN_PARTIAL_SUPER_SPECTRE_LEVEL,
};

pub struct SuperSpectre {
    a: Box<SpectreLike>,
    b: Box<SpectreLike>,
    c: Box<SpectreLike>,
    d: Box<SpectreLike>,
    e: Box<SpectreLike>,
    f: Box<SpectreLike>,
    g: Box<SpectreLike>,
    h: Box<MysticLike>,
    pub level: usize,
    aabb: Aabb,
}

impl Geometry for SuperSpectre {
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

    fn aabb(&self) -> Aabb {
        self.aabb
    }
}

impl SuperSpectre {
    pub fn iter(&self, aabb: Aabb) -> SpectreIter<'_> {
        SpectreIter {
            parents: vec![(self, 0)],
            aabb,
        }
    }

    pub fn spectres_in<'a>(&'a self, aabb: Aabb) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        if !self.aabb().has_intersection(&aabb) {
            return Box::new(std::iter::empty());
        }
        Box::new(self.iter(aabb))
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
        assert!(h.point(Anchor::Anchor1) == a.point(Anchor::Anchor1));
        assert!(a.point(Anchor::Anchor3) == b.point(Anchor::Anchor1));
        assert!(b.point(Anchor::Anchor4) == c.point(Anchor::Anchor2));
        assert!(c.point(Anchor::Anchor3) == d.point(Anchor::Anchor1));
        assert!(d.point(Anchor::Anchor3) == e.point(Anchor::Anchor1));
        assert!(e.point(Anchor::Anchor4) == f.point(Anchor::Anchor2));
        assert!(f.point(Anchor::Anchor3) == g.point(Anchor::Anchor1));
        assert!(g.point(Anchor::Anchor4) == h.point(Anchor::Anchor4));

        // Calculate AABB only for existing parts
        let mut aabb = Aabb::NULL;
        aabb = aabb.union(&a.aabb());
        aabb = aabb.union(&b.aabb());
        aabb = aabb.union(&c.aabb());
        aabb = aabb.union(&d.aabb());
        aabb = aabb.union(&e.aabb());
        aabb = aabb.union(&f.aabb());
        aabb = aabb.union(&g.aabb());
        aabb = aabb.union(&h.aabb());

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
            aabb,
        }
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
                    SpectreLike::from(SuperSpectre::new_with_anchor(
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
                    SpectreLike::from(SuperSpectre::new_with_anchor(
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
                    SpectreLike::from(SuperSpectre::new_with_anchor(
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
                    SpectreLike::from(SuperSpectre::new_with_anchor(
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

    pub fn adjacent_super_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> SuperSpectre {
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

        SuperSpectre::new_with_anchor(self.level, self.point(from_anchor), to_anchor, angle)
    }

    pub fn into_super_mystic(self) -> SuperMystic {
        // Calculate AABB only for existing parts
        let mut aabb = Aabb::NULL;
        aabb = aabb.union(&self.a.aabb());
        aabb = aabb.union(&self.b.aabb());
        aabb = aabb.union(&self.c.aabb());
        aabb = aabb.union(&self.d.aabb());
        aabb = aabb.union(&self.f.aabb());
        aabb = aabb.union(&self.g.aabb());
        aabb = aabb.union(&self.h.aabb());

        SuperMystic {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            f: self.f,
            g: self.g,
            h: self.h,
            level: self.level,
            aabb,
        }
    }

    pub fn update_children(&mut self, aabb: &Aabb) {
        if self.level < MIN_PARTIAL_SUPER_SPECTRE_LEVEL {
            return;
        }
        self.a.update(aabb);
        self.b.update(aabb);
        self.c.update(aabb);
        self.d.update(aabb);
        self.e.update(aabb);
        self.f.update(aabb);
        self.g.update(aabb);
        self.h.update(aabb);
        let mut aabb = Aabb::NULL;
        aabb = aabb.union(&self.a.aabb());
        aabb = aabb.union(&self.b.aabb());
        aabb = aabb.union(&self.c.aabb());
        aabb = aabb.union(&self.d.aabb());
        aabb = aabb.union(&self.e.aabb());
        aabb = aabb.union(&self.f.aabb());
        aabb = aabb.union(&self.g.aabb());
        aabb = aabb.union(&self.h.aabb());
        self.aabb = aabb;
    }
}

pub struct SuperMystic {
    a: Box<SpectreLike>,
    b: Box<SpectreLike>,
    c: Box<SpectreLike>,
    d: Box<SpectreLike>,
    f: Box<SpectreLike>,
    g: Box<SpectreLike>,
    h: Box<MysticLike>,
    pub level: usize,
    aabb: Aabb,
}

impl SuperMystic {
    pub fn update_children(&mut self, aabb: &Aabb) {
        if self.level < MIN_PARTIAL_SUPER_SPECTRE_LEVEL {
            return;
        }
        self.a.update(aabb);
        self.b.update(aabb);
        self.c.update(aabb);
        self.d.update(aabb);
        self.f.update(aabb);
        self.g.update(aabb);
        self.h.update(aabb);
        let mut aabb = Aabb::NULL;
        aabb = aabb.union(&self.a.aabb());
        aabb = aabb.union(&self.b.aabb());
        aabb = aabb.union(&self.c.aabb());
        aabb = aabb.union(&self.d.aabb());
        aabb = aabb.union(&self.f.aabb());
        aabb = aabb.union(&self.g.aabb());
        aabb = aabb.union(&self.h.aabb());
        self.aabb = aabb;
    }
}

impl Geometry for SuperMystic {
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

    fn aabb(&self) -> Aabb {
        self.aabb
    }
}

impl SpectreContainer for SuperSpectre {
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

impl SpectreContainer for SuperMystic {
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
