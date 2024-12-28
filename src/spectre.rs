use bevy::math::Vec2;

use crate::{
    anchor::Anchor,
    angle::Angle,
    spectre_like::{MysticLike, SpectreLike},
};

/// タイルの形状を表す
#[derive(Clone)]
pub struct Spectre {
    /// 辺の長さ
    pub size: f32,
    /// タイルの回転角度
    pub angle: Angle,
    /// タイルを構成する頂点の座標
    pub points: [Vec2; Spectre::VERTEX_COUNT],
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
    pub fn new_with_anchor(
        anchor_point: Vec2,
        anchor: Anchor,
        size: f32,
        angle: impl Into<Angle>,
    ) -> Self {
        Self::new_with_anchor_at(anchor_point, anchor.index(), size, angle.into())
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
    /// * `angle` - 基準角度（0〜ANGLE_COUNT-1）
    fn new_with_anchor_at(
        anchor_point: Vec2,
        anchor_index: usize,
        size: f32,
        angle: Angle,
    ) -> Self {
        let mut points = [Vec2::ZERO; Self::VERTEX_COUNT];
        points[anchor_index] = anchor_point;

        // アンカーから前方の点を配置
        Self::place_points_before(&mut points[..anchor_index], anchor_point, angle, size);

        // アンカーから後方の点を配置
        Self::place_points_after(
            &mut points[anchor_index + 1..],
            anchor_point,
            anchor_index,
            angle,
            size,
        );

        Self {
            size,
            angle,
            points,
        }
    }

    /// アンカーより前方の点を配置する（反時計回り）
    fn place_points_before(points: &mut [Vec2], start: Vec2, angle: Angle, size: f32) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate().rev() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[i]);
            p -= dir * size;
            *point = p;
        }
    }

    /// アンカーより後方の点を配置する（時計回り）
    fn place_points_after(
        points: &mut [Vec2],
        start: Vec2,
        anchor_index: usize,
        angle: Angle,
        size: f32,
    ) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[anchor_index + i]);
            p += dir * size;
            *point = p;
        }
    }

    /// アンカーから出る辺の方向を取得する
    fn edge_direction(anchor: Anchor) -> Angle {
        Self::DIRECTIONS[anchor.index()]
    }

    /// アンカーに入る辺の方向を取得する
    fn prev_edge_direction(anchor: Anchor) -> Angle {
        Self::DIRECTIONS[(anchor.index() + Self::VERTEX_COUNT - 1) % Self::VERTEX_COUNT]
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
        // 接続する辺の方向を取得
        let out_dir = Self::edge_direction(from_anchor);
        let in_dir = Self::prev_edge_direction(to_anchor);

        // 新しいSpectreの角度を計算
        // 1. 現在の角度を基準に
        // 2. 出る辺の方向を加える
        // 3. 入る辺の方向を引く
        // 4. 180度（6方向）回転させて反対向きにする
        let angle = self.angle + out_dir - (in_dir - Angle::OPPOSITE);

        // 新しいSpectreを生成：接続点を基準に配置
        Self::new_with_anchor(
            self.points[from_anchor.index()],
            to_anchor,
            self.size,
            angle,
        )
    }

    pub fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.points[anchor.index()]
    }
}

pub struct Mystic {
    a: Spectre,
    b: Spectre,
}

impl Mystic {
    fn new_with_anchor(
        anchor_point: Vec2,
        anchor: Anchor,
        size: f32,
        angle: impl Into<Angle>,
    ) -> Self {
        let angle: Angle = angle.into();
        let a = Spectre::new_with_anchor(anchor_point, anchor, size, angle);
        let b = Spectre::new_with_anchor_at(a.points[1], 13, size, angle - Angle::new(1));
        Self { a, b }
    }
}

pub struct SuperSpectre {
    a: Box<dyn SpectreLike>,
    b: Box<dyn SpectreLike>,
    c: Box<dyn SpectreLike>,
    d: Box<dyn SpectreLike>,
    e: Box<dyn SpectreLike>,
    f: Box<dyn SpectreLike>,
    g: Box<dyn SpectreLike>,
    h: Box<dyn MysticLike>,
}

impl SuperSpectre {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: Box<dyn SpectreLike>,
        b: Box<dyn SpectreLike>,
        c: Box<dyn SpectreLike>,
        d: Box<dyn SpectreLike>,
        e: Box<dyn SpectreLike>,
        f: Box<dyn SpectreLike>,
        g: Box<dyn SpectreLike>,
        h: Box<dyn MysticLike>,
    ) -> Self {
        assert!(a.size() == b.size());
        assert!(b.size() == c.size());
        assert!(c.size() == d.size());
        assert!(d.size() == e.size());
        assert!(e.size() == f.size());
        assert!(f.size() == g.size());
        assert!(g.size() == h.size());

        assert!(
            h.anchor(Anchor::Anchor1)
                .distance_squared(a.anchor(Anchor::Anchor1))
                / a.size()
                < 0.01
        );
        assert!(
            a.anchor(Anchor::Anchor3)
                .distance_squared(b.anchor(Anchor::Anchor1))
                / a.size()
                < 0.01
        );
        assert!(
            b.anchor(Anchor::Anchor4)
                .distance_squared(c.anchor(Anchor::Anchor2))
                / a.size()
                < 0.01
        );
        assert!(
            c.anchor(Anchor::Anchor3)
                .distance_squared(d.anchor(Anchor::Anchor1))
                / a.size()
                < 0.01
        );
        assert!(
            d.anchor(Anchor::Anchor3)
                .distance_squared(e.anchor(Anchor::Anchor1))
                / a.size()
                < 0.01
        );
        assert!(
            e.anchor(Anchor::Anchor4)
                .distance_squared(f.anchor(Anchor::Anchor2))
                / a.size()
                < 0.01
        );
        assert!(
            f.anchor(Anchor::Anchor3)
                .distance_squared(g.anchor(Anchor::Anchor1))
                / a.size()
                < 0.01
        );
        assert!(
            g.anchor(Anchor::Anchor4)
                .distance_squared(h.anchor(Anchor::Anchor4))
                / a.size()
                < 0.01
        );

        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        }
    }
}

impl SpectreLike for Spectre {
    fn size(&self) -> f32 {
        self.size
    }
    fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.anchor(anchor)
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        Spectre::edge_direction(anchor) + self.angle
    }

    fn prev_edge_direction(&self, anchor: Anchor) -> Angle {
        Spectre::prev_edge_direction(anchor) + self.angle
    }

    fn to_mystic_like(self) -> Box<dyn MysticLike> {
        let a = self;
        let b = Spectre::new_with_anchor_at(a.points[1], 13, a.size, a.angle - Angle::new(1));
        Box::new(Mystic { a, b })
    }

    fn spectres(&self) -> Vec<&Spectre> {
        vec![self]
    }
}

impl SpectreLike for SuperSpectre {
    fn size(&self) -> f32 {
        self.a.size()
    }
    fn anchor(&self, anchor: Anchor) -> Vec2 {
        match anchor {
            Anchor::Anchor1 => self.g.anchor(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.anchor(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.anchor(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.anchor(Anchor::Anchor2),
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

    fn prev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.g.prev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.prev_edge_direction(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.prev_edge_direction(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.prev_edge_direction(Anchor::Anchor2),
        }
    }

    fn to_mystic_like(self) -> Box<dyn MysticLike> {
        Box::new(SuperMystic {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            f: self.f,
            g: self.g,
            h: self.h,
        })
    }

    fn spectres(&self) -> Vec<&Spectre> {
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
        .collect()
    }
}

pub struct SuperMystic {
    a: Box<dyn SpectreLike>,
    b: Box<dyn SpectreLike>,
    c: Box<dyn SpectreLike>,
    d: Box<dyn SpectreLike>,

    f: Box<dyn SpectreLike>,
    g: Box<dyn SpectreLike>,
    h: Box<dyn MysticLike>,
}

impl MysticLike for Mystic {
    fn size(&self) -> f32 {
        self.a.size
    }
    fn anchor(&self, anchor: Anchor) -> Vec2 {
        self.a.anchor(anchor)
    }
    fn spectres(&self) -> Vec<&Spectre> {
        vec![&self.a, &self.b]
    }
}

impl MysticLike for SuperMystic {
    fn size(&self) -> f32 {
        self.a.size()
    }
    fn anchor(&self, anchor: Anchor) -> Vec2 {
        match anchor {
            Anchor::Anchor1 => self.g.anchor(Anchor::Anchor3),
            Anchor::Anchor2 => self.d.anchor(Anchor::Anchor2),
            Anchor::Anchor3 => self.b.anchor(Anchor::Anchor3),
            Anchor::Anchor4 => self.a.anchor(Anchor::Anchor2),
        }
    }
    fn spectres(&self) -> Vec<&Spectre> {
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
        .collect()
    }
}
