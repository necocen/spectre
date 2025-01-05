use crate::utils::{Angle, HexValue, HexVec};

use super::{Anchor, Geometry};

/// タイルの形状を表す
#[derive(Clone)]
pub struct Spectre {
    /// アンカー1から反時計回りに進む辺の向く方向
    pub angle: Angle,
    /// アンカー1の座標
    pub anchor1: HexVec,
}

impl Geometry for Spectre {
    fn anchor(&self, anchor: Anchor) -> HexVec {
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

        // TODO: ここもうちょっと効率化したほうがいいけどね

        // アンカーから前方の点を配置
        Self::place_points_before(&mut points[..anchor_index], anchor_point, angle);

        Self {
            angle,
            anchor1: points[0],
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
        // FIXME: memoizeしたほうがいい
        let mut p = self.anchor1;
        for i in 0..index {
            let dir = Self::direction_vector(self.angle, Self::DIRECTIONS[i]);
            p += dir;
        }
        p
    }

    pub fn all_points(&self) -> Vec<HexVec> {
        (0..Self::VERTEX_COUNT).map(|i| self.points(i)).collect()
    }

    pub fn into_mystic(self) -> Mystic {
        let a = self.clone();
        let b = Spectre::new_with_anchor_at(a.points(1), 13, a.angle + Angle::new(9));
        Mystic { a, b }
    }
}

pub struct Mystic {
    a: Spectre,
    b: Spectre,
}

impl Geometry for Mystic {
    fn anchor(&self, anchor: Anchor) -> HexVec {
        self.a.anchor(anchor)
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        self.a.edge_direction(anchor)
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        self.a.rev_edge_direction(anchor)
    }
}

impl Mystic {
    pub fn spectres(&self) -> impl Iterator<Item = &Spectre> {
        std::iter::once(&self.a).chain(std::iter::once(&self.b))
    }
}
