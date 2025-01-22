use crate::utils::{Aabb, Angle, HexValue, HexVec};

use super::{Anchor, Mystic};

/// タイルの形状を表す
#[derive(Clone, Copy)]
pub struct Spectre {
    /// アンカー1から反時計回りに進む辺の向く方向
    rotation: Angle,
    /// アンカー1の座標
    anchor1: HexVec,
    /// bounding box
    bbox: Aabb,
}

impl Spectre {
    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        self.vertex(anchor.index())
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        Self::EDGE_DIRECTIONS[anchor.index()] + self.rotation
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        Self::EDGE_DIRECTIONS[(anchor.index() + Self::VERTEX_COUNT - 1) % Self::VERTEX_COUNT]
            + self.rotation
    }

    pub fn bbox(&self) -> Aabb {
        self.bbox
    }

    pub fn rotation(&self) -> Angle {
        self.rotation
    }
}

impl Spectre {
    /// 頂点数
    const VERTEX_COUNT: usize = 14;
    /// 各頂点から反時計回りに進む辺の角度（0〜VERTEX_COUNT-1）
    const EDGE_DIRECTIONS: [Angle; Self::VERTEX_COUNT] = [
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
    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
    ) -> Self {
        Self::with_vertex(coordinate.into(), anchor.index(), edge_direction.into())
    }

    /// Mysticに変換する
    pub fn into_mystic(self) -> Mystic {
        let lower = self;
        let upper = Spectre::with_vertex(lower.vertex(1), 13, lower.rotation + Angle::new(9));
        Mystic::new(lower, upper)
    }

    /// 指定されたアンカー同士を接続した新しいSpectreを生成する
    ///
    /// # Arguments
    /// * `from_anchor` - このSpectreの接続元アンカー
    /// * `to_anchor` - 新しいSpectreの接続先アンカー
    ///
    /// # Returns
    /// 接続された新しいSpectre。このSpectreのfrom_anchorと新しいSpectreのto_anchorが接続される。
    pub fn connected_spectre(&self, from_anchor: Anchor, to_anchor: Anchor) -> Spectre {
        let rotation =
            self.edge_direction_from(to_anchor) - self.edge_direction_into(to_anchor).opposite();
        let angle = self.edge_direction_from(from_anchor) + rotation;

        // 新しいSpectreを生成：接続点を基準に配置
        Self::with_anchor(to_anchor, self.vertex(from_anchor.index()), angle)
    }

    /// 頂点
    pub fn vertices(&self) -> Vec<HexVec> {
        let mut points = Vec::with_capacity(Self::VERTEX_COUNT);
        let mut p = self.anchor1;
        points.push(p);

        for i in 0..Self::VERTEX_COUNT - 1 {
            let dir = Self::direction_vector(self.rotation, Self::EDGE_DIRECTIONS[i]);
            p += dir;
            points.push(p);
        }
        points
    }

    /// 指定された頂点と方向を基準にSpectreを生成する
    ///
    /// # Arguments
    /// * `vertex` - 基準点の座標
    /// * `index` - 基準点のインデックス
    /// * `edge_direction` - anchor_pointから出る辺の角度
    fn with_vertex(vertex: HexVec, index: usize, edge_direction: Angle) -> Self {
        let mut vertices = [HexVec::ZERO; Self::VERTEX_COUNT];
        vertices[index] = vertex;
        let angle = edge_direction - Self::EDGE_DIRECTIONS[index];

        // アンカーから前方の点を配置
        Self::place_vertices_before(&mut vertices[..index], vertex, angle);

        // アンカーから後方の点を配置
        Self::place_vertices_after(&mut vertices[index + 1..], vertex, index, angle);

        // Calculate AABB more efficiently using min/max tracking
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for p in vertices.iter() {
            let x = p.x.to_f32();
            let y = p.y.to_f32();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let bbox = Aabb::new(min_x, min_y, max_x, max_y);

        Self {
            rotation: angle,
            anchor1: vertices[0],
            bbox,
        }
    }

    /// 指定された角度の方向ベクトルを計算する
    fn direction_vector(angle: Angle, direction: Angle) -> HexVec {
        let total_angle = angle + direction;
        HexVec::new(HexValue::cos(total_angle), HexValue::sin(total_angle))
    }

    /// アンカーより前方の点を配置する（時計回り）
    fn place_vertices_before(vertices: &mut [HexVec], start: HexVec, angle: Angle) {
        let mut p = start;
        for (i, point) in vertices.iter_mut().enumerate().rev() {
            let dir = Self::direction_vector(angle, Self::EDGE_DIRECTIONS[i]);
            p -= dir;
            *point = p;
        }
    }

    /// アンカーより後方の点を配置する（反時計回り）
    fn place_vertices_after(
        vertices: &mut [HexVec],
        start: HexVec,
        anchor_index: usize,
        angle: Angle,
    ) {
        let mut p = start;
        for (i, point) in vertices.iter_mut().enumerate() {
            let dir = Self::direction_vector(angle, Self::EDGE_DIRECTIONS[anchor_index + i]);
            p += dir;
            *point = p;
        }
    }

    fn vertex(&self, index: usize) -> HexVec {
        if index == 0 {
            return self.anchor1;
        }

        // Calculate points using a cumulative approach
        let mut p = self.anchor1;
        for i in 0..index {
            let dir = Self::direction_vector(self.rotation, Self::EDGE_DIRECTIONS[i]);
            p += dir;
        }
        p
    }
}
