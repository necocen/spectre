use bevy::math::Vec2;

use crate::{
    anchor::Anchor,
    angle::Angle,
    spectre_like::{MysticLike, SpectreLike},
};

/// タイルの形状を表す
#[derive(Clone)]
pub struct Spectre {
    /// アンカー1から反時計回りに進む辺の向く方向
    pub angle: Angle,
    /// アンカー1の座標
    pub anchor1: Vec2,
}

impl Spectre {
    /// 頂点数
    const VERTEX_COUNT: usize = 14;
    /// 各頂点から反時計回りに進む辺の角度（0〜ANGLE_COUNT-1）
    const DIRECTIONS: [Angle; Self::VERTEX_COUNT] = [
        Angle::new(0),
        Angle::new(0),
        Angle::new(2),
        Angle::new(11),
        Angle::new(1),
        Angle::new(4),
        Angle::new(6),
        Angle::new(3),
        Angle::new(5),
        Angle::new(8),
        Angle::new(6),
        Angle::new(9),
        Angle::new(7),
        Angle::new(10),
    ];

    /// 指定されたアンカーを基準点としてタイルを生成する
    pub fn new_with_anchor(anchor_point: Vec2, anchor: Anchor, angle: impl Into<Angle>) -> Self {
        Self::new_with_anchor_at(anchor_point, anchor.index(), angle.into())
    }

    /// 指定された角度の方向ベクトルを計算する
    fn direction_vector(angle: Angle, direction: Angle) -> Vec2 {
        let total_angle = angle + direction;
        let rad = total_angle.to_radians();
        Vec2::new(rad.cos(), rad.sin())
    }

    /// 指定されたアンカーを基準に点を配置する
    ///
    /// # Arguments
    /// * `anchor_point` - アンカーの座標
    /// * `anchor_index` - アンカーのインデックス
    /// * `size` - 辺の長さ
    /// * `angle` - anchor_pointから出る辺の角度
    fn new_with_anchor_at(anchor_point: Vec2, anchor_index: usize, angle: Angle) -> Self {
        let mut points = [Vec2::ZERO; Self::VERTEX_COUNT];
        points[anchor_index] = anchor_point;
        let angle = angle - Self::DIRECTIONS[anchor_index];

        // TODO: ここもうちょっと効率化したほうがいいけどね

        // アンカーから前方の点を配置
        Self::place_points_before(&mut points[..anchor_index], anchor_point, angle);

        Self {
            angle,
            anchor1: points[0],
        }
    }

    /// アンカーより前方の点を配置する（反時計回り）
    fn place_points_before(points: &mut [Vec2], start: Vec2, angle: Angle) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate().rev() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[i]);
            p -= dir;
            *point = p;
        }
    }

    /// アンカーから出る辺の方向を取得する
    pub fn edge_direction(&self, anchor: Anchor) -> Angle {
        Self::DIRECTIONS[anchor.index()] + self.angle
    }

    /// アンカーに入る辺の方向を取得する
    pub fn prev_edge_direction(&self, anchor: Anchor) -> Angle {
        Self::DIRECTIONS[(anchor.index() + Self::VERTEX_COUNT - 1) % Self::VERTEX_COUNT]
            + self.angle
    }

    /// 指定されたアンカー同士を接続した新しいSpectreを生成する
    ///
    /// # Arguments
    /// * `from_anchor` - このSpectreの接続元アンカー
    /// * `to_anchor` - 新しいSpectreの接続先アンカー
    ///
    /// # Returns
    /// 接続された新しいSpectre。このSpectreのfrom_anchorと新しいSpectreのto_anchorが接続される。
    pub fn adjacent_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> Spectre {
        let rotation =
            self.edge_direction(to_anchor) - self.prev_edge_direction(to_anchor).opposite();
        let angle = self.edge_direction(from_anchor) + rotation;

        // 新しいSpectreを生成：接続点を基準に配置
        Self::new_with_anchor(self.points(from_anchor.index()), to_anchor, angle)
    }

    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.points(anchor.index())
    }

    fn points(&self, index: usize) -> Vec2 {
        // FIXME: memoizeしたほうがいい
        let mut p = self.anchor1;
        for i in 0..index {
            let dir = Self::direction_vector(self.angle, Self::DIRECTIONS[i]);
            p += dir;
        }
        p
    }

    pub fn all_points(&self) -> Vec<Vec2> {
        (0..Self::VERTEX_COUNT).map(|i| self.points(i)).collect()
    }

    fn into_mystic(self) -> Mystic {
        let a = self.clone();
        let b = Spectre::new_with_anchor_at(a.points(1), 13, a.angle + Angle::new(9));
        Mystic { a, b }
    }
}

pub struct Mystic {
    a: Spectre,
    b: Spectre,
}

impl Mystic {
    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.a.anchor(anchor)
    }
    pub fn spectres(&self) -> impl Iterator<Item = &Spectre> {
        std::iter::once(&self.a).chain(std::iter::once(&self.b))
    }
}

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
}

impl SuperSpectre {
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
        // assert!(
        //     h.anchor(Anchor::Anchor1)
        //         .distance_squared(a.anchor(Anchor::Anchor1))
        //         < 0.01
        // );
        // assert!(
        //     a.anchor(Anchor::Anchor3)
        //         .distance_squared(b.anchor(Anchor::Anchor1))
        //         < 0.01
        // );
        // assert!(
        //     b.anchor(Anchor::Anchor4)
        //         .distance_squared(c.anchor(Anchor::Anchor2))
        //         < 0.01
        // );
        // assert!(
        //     c.anchor(Anchor::Anchor3)
        //         .distance_squared(d.anchor(Anchor::Anchor1))
        //         < 0.01
        // );
        // assert!(
        //     d.anchor(Anchor::Anchor3)
        //         .distance_squared(e.anchor(Anchor::Anchor1))
        //         < 0.01
        // );
        // assert!(
        //     e.anchor(Anchor::Anchor4)
        //         .distance_squared(f.anchor(Anchor::Anchor2))
        //         < 0.01
        // );
        // assert!(
        //     f.anchor(Anchor::Anchor3)
        //         .distance_squared(g.anchor(Anchor::Anchor1))
        //         < 0.01
        // );
        // assert!(
        //     g.anchor(Anchor::Anchor4)
        //         .distance_squared(h.anchor(Anchor::Anchor4))
        //         < 0.01
        // );

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
        }
    }

    pub fn new_with_anchor(
        level: usize,
        anchor_point: Vec2,
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

    fn adjacent_super_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> SuperSpectre {
        // 新しいSpectreの角度を計算
        let rotation =
            self.edge_direction(to_anchor) - self.prev_edge_direction(to_anchor).opposite();
        let angle = self.edge_direction(from_anchor) + rotation;

        SuperSpectre::new_with_anchor(self.level, self.anchor(from_anchor), to_anchor, angle)
    }

    fn into_super_mystic(self) -> SuperMystic {
        SuperMystic {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            f: self.f,
            g: self.g,
            h: self.h,
        }
    }

    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        match anchor {
            Anchor::Anchor1 => self.g.anchor(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.anchor(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.anchor(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.anchor(Anchor::Anchor2),
        }
    }

    pub fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.edge_direction(Anchor::Anchor2),
        }
    }

    pub fn prev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.prev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.prev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.prev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.prev_edge_direction(Anchor::Anchor2),
        }
    }

    pub fn spectres(&self) -> impl Iterator<Item = &Spectre> {
        vec![
            self.a.spectres(),
            self.b.spectres(),
            self.c.spectres(),
            self.d.spectres(),
            self.e.spectres(),
            self.f.spectres(),
            self.g.spectres(),
            self.h.spectres(),
        ]
        .into_iter()
        .flatten()
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
}

impl SuperMystic {
    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        match anchor {
            Anchor::Anchor1 => self.g.anchor(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.anchor(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.anchor(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.anchor(Anchor::Anchor2),
        }
    }

    pub fn spectres(&self) -> impl Iterator<Item = &Spectre> {
        vec![
            self.a.spectres(),
            self.b.spectres(),
            self.c.spectres(),
            self.d.spectres(),
            self.f.spectres(),
            self.g.spectres(),
            self.h.spectres(),
        ]
        .into_iter()
        .flatten()
    }
}
