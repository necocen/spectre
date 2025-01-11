use crate::utils::{Angle, HexValue, HexVec};

use super::{Aabb, Anchor, Geometry};

/// タイルの形状を表す
#[derive(Clone, Copy)]
pub struct Spectre {
    /// アンカー1から反時計回りに進む辺の向く方向
    pub angle: Angle,
    /// アンカー1の座標
    pub anchor1: HexVec,
    /// bounding box
    pub aabb: Aabb,
}

impl Geometry for Spectre {
    fn point(&self, anchor: Anchor) -> HexVec {
        self.points(anchor.index())
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        Self::DIRECTIONS[anchor.index()] + self.angle
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        Self::DIRECTIONS[(anchor.index() + Self::VERTEX_COUNT - 1) % Self::VERTEX_COUNT]
            + self.angle
    }
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
        anchor_point: impl Into<HexVec>,
        anchor: Anchor,
        angle: impl Into<Angle>,
    ) -> Self {
        Self::new_with_anchor_at(anchor_point, anchor.index(), angle.into())
    }

    /// 指定された角度の方向ベクトルを計算する
    fn direction_vector(angle: Angle, direction: Angle) -> HexVec {
        let total_angle = angle + direction;
        HexVec::new(HexValue::cos(total_angle), HexValue::sin(total_angle))
    }

    /// 指定されたアンカーを基準に点を配置する
    ///
    /// # Arguments
    /// * `anchor_point` - アンカーの座標
    /// * `anchor_index` - アンカーのインデックス
    /// * `size` - 辺の長さ
    /// * `angle` - anchor_pointから出る辺の角度
    fn new_with_anchor_at(
        anchor_point: impl Into<HexVec>,
        anchor_index: usize,
        angle: Angle,
    ) -> Self {
        let mut points = [HexVec::ZERO; Self::VERTEX_COUNT];
        let anchor_point = anchor_point.into();
        points[anchor_index] = anchor_point;
        let angle = angle - Self::DIRECTIONS[anchor_index];

        // アンカーから前方の点を配置
        Self::place_points_before(&mut points[..anchor_index], anchor_point, angle);

        // アンカーから後方の点を配置
        Self::place_points_after(&mut points[anchor_index + 1..], anchor_point, angle);

        // Calculate AABB more efficiently using min/max tracking
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for p in points.iter() {
            let x = p.x.to_f32();
            let y = p.y.to_f32();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let aabb = Aabb::new(min_x, min_y, max_x, max_y);

        Self {
            angle,
            anchor1: points[0],
            aabb,
        }
    }

    /// アンカーより前方の点を配置する（反時計回り）
    fn place_points_before(points: &mut [HexVec], start: HexVec, angle: Angle) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate().rev() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[i]);
            p -= dir;
            *point = p;
        }
    }

    /// アンカーより後方の点を配置する（反時計回り）
    fn place_points_after(points: &mut [HexVec], start: HexVec, angle: Angle) {
        let mut p = start;
        for (i, point) in points.iter_mut().enumerate() {
            let dir = Self::direction_vector(angle, Self::DIRECTIONS[i]);
            p += dir;
            *point = p;
        }
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
            self.edge_direction(to_anchor) - self.rev_edge_direction(to_anchor).opposite();
        let angle = self.edge_direction(from_anchor) + rotation;

        // 新しいSpectreを生成：接続点を基準に配置
        Self::new_with_anchor(self.points(from_anchor.index()), to_anchor, angle)
    }

    fn points(&self, index: usize) -> HexVec {
        if index == 0 {
            return self.anchor1;
        }

        // Calculate points using a cumulative approach
        let mut p = self.anchor1;
        for i in 0..index {
            let dir = Self::direction_vector(self.angle, Self::DIRECTIONS[i]);
            p += dir;
        }
        p
    }

    pub fn all_points(&self) -> Vec<HexVec> {
        let mut points = Vec::with_capacity(Self::VERTEX_COUNT);
        let mut p = self.anchor1;
        points.push(p);

        for i in 0..Self::VERTEX_COUNT - 1 {
            let dir = Self::direction_vector(self.angle, Self::DIRECTIONS[i]);
            p += dir;
            points.push(p);
        }
        points
    }

    pub fn into_mystic(self) -> Mystic {
        let a = self;
        let b = Spectre::new_with_anchor_at(a.points(1), 13, a.angle + Angle::new(9));
        Mystic::new(a, b)
    }

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }
}

pub struct Mystic {
    pub a: Spectre,
    pub b: Spectre,
    pub aabb: Aabb,
}

impl Geometry for Mystic {
    fn point(&self, anchor: Anchor) -> HexVec {
        self.a.point(anchor)
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        self.a.edge_direction(anchor)
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        self.a.rev_edge_direction(anchor)
    }
}

impl Mystic {
    pub fn new(a: Spectre, b: Spectre) -> Self {
        let aabb = a.aabb.union(&b.aabb);
        Self { a, b, aabb }
    }

    pub fn has_intersection(&self, aabb: &Aabb) -> bool {
        !self.aabb.intersection(aabb).is_empty()
    }
}
