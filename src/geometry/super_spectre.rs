use crate::utils::{Angle, HexVec};

use super::{Aabb, Anchor, Geometry, MysticLike, Spectre, SpectreLike};

pub struct SuperSpectre {
    a: SpectreLike,
    b: SpectreLike,
    c: SpectreLike,
    d: SpectreLike,
    e: SpectreLike,
    f: SpectreLike,
    g: SpectreLike,
    h: MysticLike,
    level: usize,
    pub aabb: Aabb,
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
}

impl SuperSpectre {
    const SPECTRE_COUNT: usize = 7;
    const SPECTRE_REFS: [fn(&SuperSpectre) -> &SpectreLike; Self::SPECTRE_COUNT] = [
        |s| &s.a, |s| &s.b, |s| &s.c, |s| &s.d, |s| &s.e, |s| &s.f, |s| &s.g,
    ];

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: impl Into<SpectreLike>,
        b: impl Into<SpectreLike>,
        c: impl Into<SpectreLike>,
        d: impl Into<SpectreLike>,
        e: impl Into<SpectreLike>,
        f: impl Into<SpectreLike>,
        g: impl Into<SpectreLike>,
        h: impl Into<MysticLike>,
        level: usize,
    ) -> Self {
        let a: SpectreLike = a.into();
        let b: SpectreLike = b.into();
        let c: SpectreLike = c.into();
        let d: SpectreLike = d.into();
        let e: SpectreLike = e.into();
        let f: SpectreLike = f.into();
        let g: SpectreLike = g.into();
        let h: MysticLike = h.into();
        assert!(h.point(Anchor::Anchor1) == a.point(Anchor::Anchor1));
        assert!(a.point(Anchor::Anchor3) == b.point(Anchor::Anchor1));
        assert!(b.point(Anchor::Anchor4) == c.point(Anchor::Anchor2));
        assert!(c.point(Anchor::Anchor3) == d.point(Anchor::Anchor1));
        assert!(d.point(Anchor::Anchor3) == e.point(Anchor::Anchor1));
        assert!(e.point(Anchor::Anchor4) == f.point(Anchor::Anchor2));
        assert!(f.point(Anchor::Anchor3) == g.point(Anchor::Anchor1));
        assert!(g.point(Anchor::Anchor4) == h.point(Anchor::Anchor4));

        // Calculate AABB more efficiently
        let mut aabb = a.aabb();
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
        if level == 1 {
            // Spectreを8つ作る
            match anchor {
                Anchor::Anchor1 => {
                    // G3
                    let g = Spectre::new_with_anchor(anchor_point, Anchor::Anchor3, angle);
                    let h = g.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let b = a.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let h = h.into_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor2 => {
                    // D2
                    let d = Spectre::new_with_anchor(anchor_point, Anchor::Anchor2, angle);
                    let e = d.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let b = a.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let h = h.into_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor3 => {
                    // B3
                    let b = Spectre::new_with_anchor(anchor_point, Anchor::Anchor3, angle);
                    let c = b.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let h = h.into_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor4 => {
                    // A2
                    let a = Spectre::new_with_anchor(anchor_point, Anchor::Anchor2, angle);
                    let b = a.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let h = h.into_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
            }
        } else if level % 2 == 0 {
            // level is even
            match anchor {
                Anchor::Anchor1 => {
                    let g = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    );
                    let f = g.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let e = f.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let d = e.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let c = d.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let b = c.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let a = b.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let h = a
                        .adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1)
                        .into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor2 => {
                    let d = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    );
                    let c = d.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let b = c.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let a = b.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let h = a.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let g = h.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let f = g.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let e = f.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor3 => {
                    let b = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    );
                    let a = b.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let h = a.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let g = h.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let f = g.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let e = f.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let d = e.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let c = d.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor4 => {
                    let a = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    );
                    let h = a.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let g = h.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let f = g.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let e = f.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let d = e.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let c = d.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor3);
                    let b = c.adjacent_super_spectre(Anchor::Anchor2, Anchor::Anchor4);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
            }
        } else {
            // level is odd
            match anchor {
                Anchor::Anchor1 => {
                    let g = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    );
                    let h = g.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let b = a.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor2 => {
                    let d = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    );
                    let e = d.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let b = a.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor3 => {
                    let b = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor3,
                        angle,
                    );
                    let c = b.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let a = h.adjacent_super_spectre(Anchor::Anchor1, Anchor::Anchor1);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
                Anchor::Anchor4 => {
                    let a = SuperSpectre::new_with_anchor(
                        level - 1,
                        anchor_point,
                        Anchor::Anchor2,
                        angle,
                    );
                    let b = a.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let c = b.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let d = c.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let e = d.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let f = e.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor2);
                    let g = f.adjacent_super_spectre(Anchor::Anchor3, Anchor::Anchor1);
                    let h = g.adjacent_super_spectre(Anchor::Anchor4, Anchor::Anchor4);
                    let h = h.into_super_mystic();
                    Self::new(a, b, c, d, e, f, g, h, level)
                }
            }
        }
    }

    pub fn adjacent_super_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> SuperSpectre {
        // 新しいSpectreの角度を計算
        let rotation =
            self.edge_direction(to_anchor) - self.rev_edge_direction(to_anchor).opposite();
        let angle = self.edge_direction(from_anchor) + rotation;

        SuperSpectre::new_with_anchor(self.level, self.point(from_anchor), to_anchor, angle)
    }

    pub fn into_super_mystic(self) -> SuperMystic {
        // Calculate AABB more efficiently
        let mut aabb = self.a.aabb();
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
            aabb,
        }
    }

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }

    pub fn spectres_in<'a, 'b: 'a>(&'a self, aabb: &'b Aabb) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        // Early return if no intersection with the entire SuperSpectre
        if !self.has_intersection(aabb) {
            return Box::new(std::iter::empty());
        }

        // Pre-allocate Vec with estimated capacity
        let mut spectres = Vec::with_capacity(Self::SPECTRE_COUNT * 2);

        // Use static array of field accessors
        for get_spectre in Self::SPECTRE_REFS.iter() {
            let s = get_spectre(self);
            if s.has_intersection(aabb) {
                spectres.extend(s.spectres_in(aabb));
            }
        }

        if self.h.has_intersection(aabb) {
            spectres.extend(self.h.spectres_in(aabb));
        }

        Box::new(spectres.into_iter())
    }
}

pub struct SuperMystic {
    a: SpectreLike,
    b: SpectreLike,
    c: SpectreLike,
    d: SpectreLike,
    f: SpectreLike,
    g: SpectreLike,
    h: MysticLike,
    pub aabb: Aabb,
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
}

impl SuperMystic {
    const SPECTRE_COUNT: usize = 6;
    const SPECTRE_REFS: [fn(&SuperMystic) -> &SpectreLike; Self::SPECTRE_COUNT] = [
        |s| &s.a, |s| &s.b, |s| &s.c, |s| &s.d, |s| &s.f, |s| &s.g,
    ];

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }

    pub fn spectres_in<'a, 'b: 'a>(&'a self, aabb: &'b Aabb) -> Box<dyn Iterator<Item = &'a Spectre> + 'a> {
        // Early return if no intersection with the entire SuperMystic
        if !self.has_intersection(aabb) {
            return Box::new(std::iter::empty());
        }

        // Pre-allocate Vec with estimated capacity
        let mut spectres = Vec::with_capacity(Self::SPECTRE_COUNT * 2);

        // Use static array of field accessors
        for get_spectre in Self::SPECTRE_REFS.iter() {
            let s = get_spectre(self);
            if s.has_intersection(aabb) {
                spectres.extend(s.spectres_in(aabb));
            }
        }

        if self.h.has_intersection(aabb) {
            spectres.extend(self.h.spectres_in(aabb));
        }

        Box::new(spectres.into_iter())
    }
}
