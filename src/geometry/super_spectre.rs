use crate::utils::{Angle, HexVec};

use super::{Aabb, Anchor, Geometry, MysticLike, Spectre, SpectreLike};

pub trait SpectreContainer {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike>;
    fn get_mystic(&self) -> Option<&MysticLike>;
    fn max_index(&self) -> usize;
    fn has_intersection(&self, aabb: &Aabb) -> bool;
    fn level(&self) -> usize;
}

pub struct SpectreIter<'a> {
    parents: Vec<(&'a dyn SpectreContainer, usize)>,
    aabb: Aabb,
}

impl<'a> Iterator for SpectreIter<'a> {
    type Item = &'a Spectre;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((parent, mut index)) = self.parents.pop() {
            if parent.level() == 1 {
                // abcdefgを辿る
                for i in index..parent.max_index() {
                    if let Some(SpectreLike::Spectre(spectre)) = parent.get_spectre(i) {
                        if !spectre.has_intersection(&self.aabb) {
                            continue;
                        }
                        self.parents.push((parent, i + 1));
                        return Some(spectre);
                    }
                }

                // hを辿る
                if let Some(MysticLike::Mystic(mystic)) = parent.get_mystic() {
                    if mystic.has_intersection(&self.aabb) {
                        if index == parent.max_index() {
                            // Mysticのaを判定する
                            if mystic.a.has_intersection(&self.aabb) {
                                self.parents.push((parent, index + 1));
                                return Some(&mystic.a);
                            }
                            index += 1;
                        }
                        if index > parent.max_index() {
                            // Mysticのbを判定する
                            if mystic.b.has_intersection(&self.aabb) {
                                // 最後なのでparentsに追加しない
                                return Some(&mystic.b);
                            }
                        }
                    }
                }
            } else {
                // SuperSpectreを辿る
                for i in index..parent.max_index() {
                    if let Some(SpectreLike::SuperSpectre(super_spectre)) = parent.get_spectre(i) {
                        if !super_spectre.has_intersection(&self.aabb) {
                            continue;
                        }
                        self.parents.push((parent, i + 1));
                        self.parents.push((&**super_spectre, 0));
                        return self.next();
                    }
                }
                // SuperMysticを辿る
                if index == parent.max_index() {
                    if let Some(MysticLike::SuperMystic(super_mystic)) = parent.get_mystic() {
                        if super_mystic.has_intersection(&self.aabb) {
                            self.parents.push((parent, parent.max_index() + 1));
                            self.parents.push((&**super_mystic, 0));
                            return self.next();
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct SuperSpectre {
    a: Option<SpectreLike>,
    b: Option<SpectreLike>,
    c: Option<SpectreLike>,
    d: Option<SpectreLike>,
    e: Option<SpectreLike>,
    f: Option<SpectreLike>,
    g: Option<SpectreLike>,
    h: Option<MysticLike>,
    level: usize,
    pub aabb: Aabb,
}

impl Geometry for SuperSpectre {
    fn point(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").point(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").point(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").point(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").point(Anchor::Anchor2),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").edge_direction(Anchor::Anchor2),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").rev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").rev_edge_direction(Anchor::Anchor2),
        }
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
        if !SpectreContainer::has_intersection(self, &aabb) {
            return Box::new(std::iter::empty());
        }
        Box::new(self.iter(aabb))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: Option<impl Into<SpectreLike>>,
        b: Option<impl Into<SpectreLike>>,
        c: Option<impl Into<SpectreLike>>,
        d: Option<impl Into<SpectreLike>>,
        e: Option<impl Into<SpectreLike>>,
        f: Option<impl Into<SpectreLike>>,
        g: Option<impl Into<SpectreLike>>,
        h: Option<impl Into<MysticLike>>,
        level: usize,
    ) -> Self {
        let a = a.map(Into::into);
        let b = b.map(Into::into);
        let c = c.map(Into::into);
        let d = d.map(Into::into);
        let e = e.map(Into::into);
        let f = f.map(Into::into);
        let g = g.map(Into::into);
        let h = h.map(Into::into);

        // Assertions only if both parts exist
        if let (Some(h), Some(a)) = (&h, &a) {
            assert!(h.point(Anchor::Anchor1) == a.point(Anchor::Anchor1));
        }
        if let (Some(a), Some(b)) = (&a, &b) {
            assert!(a.point(Anchor::Anchor3) == b.point(Anchor::Anchor1));
        }
        if let (Some(b), Some(c)) = (&b, &c) {
            assert!(b.point(Anchor::Anchor4) == c.point(Anchor::Anchor2));
        }
        if let (Some(c), Some(d)) = (&c, &d) {
            assert!(c.point(Anchor::Anchor3) == d.point(Anchor::Anchor1));
        }
        if let (Some(d), Some(e)) = (&d, &e) {
            assert!(d.point(Anchor::Anchor3) == e.point(Anchor::Anchor1));
        }
        if let (Some(e), Some(f)) = (&e, &f) {
            assert!(e.point(Anchor::Anchor4) == f.point(Anchor::Anchor2));
        }
        if let (Some(f), Some(g)) = (&f, &g) {
            assert!(f.point(Anchor::Anchor3) == g.point(Anchor::Anchor1));
        }
        if let (Some(g), Some(h)) = (&g, &h) {
            assert!(g.point(Anchor::Anchor4) == h.point(Anchor::Anchor4));
        }

        // Calculate AABB only for existing parts
        let mut aabb = Aabb::NULL;
        if let Some(a) = &a { aabb = aabb.union(&a.aabb()); }
        if let Some(b) = &b { aabb = aabb.union(&b.aabb()); }
        if let Some(c) = &c { aabb = aabb.union(&c.aabb()); }
        if let Some(d) = &d { aabb = aabb.union(&d.aabb()); }
        if let Some(e) = &e { aabb = aabb.union(&e.aabb()); }
        if let Some(f) = &f { aabb = aabb.union(&f.aabb()); }
        if let Some(g) = &g { aabb = aabb.union(&g.aabb()); }
        if let Some(h) = &h { aabb = aabb.union(&h.aabb()); }

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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
                    Self::new(Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), level)
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
        // Calculate AABB only for existing parts
        let mut aabb = Aabb::NULL;
        if let Some(a) = &self.a { aabb = aabb.union(&a.aabb()); }
        if let Some(b) = &self.b { aabb = aabb.union(&b.aabb()); }
        if let Some(c) = &self.c { aabb = aabb.union(&c.aabb()); }
        if let Some(d) = &self.d { aabb = aabb.union(&d.aabb()); }
        if let Some(f) = &self.f { aabb = aabb.union(&f.aabb()); }
        if let Some(g) = &self.g { aabb = aabb.union(&g.aabb()); }
        if let Some(h) = &self.h { aabb = aabb.union(&h.aabb()); }

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
}

pub struct SuperMystic {
    a: Option<SpectreLike>,
    b: Option<SpectreLike>,
    c: Option<SpectreLike>,
    d: Option<SpectreLike>,
    f: Option<SpectreLike>,
    g: Option<SpectreLike>,
    h: Option<MysticLike>,
    level: usize,
    pub aabb: Aabb,
}

impl Geometry for SuperMystic {
    fn point(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").point(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").point(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").point(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").point(Anchor::Anchor2),
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").edge_direction(Anchor::Anchor2),
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.as_ref().expect("g must exist").rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.as_ref().expect("d must exist").rev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.as_ref().expect("b must exist").rev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.as_ref().expect("a must exist").rev_edge_direction(Anchor::Anchor2),
        }
    }
}

impl SpectreContainer for SuperSpectre {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike> {
        match index {
            0 => self.a.as_ref(),
            1 => self.b.as_ref(),
            2 => self.c.as_ref(),
            3 => self.d.as_ref(),
            4 => self.e.as_ref(),
            5 => self.f.as_ref(),
            6 => self.g.as_ref(),
            _ => None,
        }
    }

    fn get_mystic(&self) -> Option<&MysticLike> {
        self.h.as_ref()
    }

    fn max_index(&self) -> usize {
        7
    }

    fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }

    fn level(&self) -> usize {
        self.level
    }
}

impl SpectreContainer for SuperMystic {
    fn get_spectre(&self, index: usize) -> Option<&SpectreLike> {
        match index {
            0 => self.a.as_ref(),
            1 => self.b.as_ref(),
            2 => self.c.as_ref(),
            3 => self.d.as_ref(),
            4 => self.f.as_ref(),
            5 => self.g.as_ref(),
            _ => None,
        }
    }

    fn get_mystic(&self) -> Option<&MysticLike> {
        self.h.as_ref()
    }

    fn max_index(&self) -> usize {
        6
    }

    fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }

    fn level(&self) -> usize {
        self.level
    }
}
