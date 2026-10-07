use crate::utils::{Aabb, Angle, HexValue, HexVec};
use glam::DVec2;
use std::sync::OnceLock;

use super::{Anchor, Mystic, bounds::ExactBounds};

struct Geometry {
    vertices: [HexVec; Spectre::VERTEX_COUNT],
    bounds: ExactBounds,
}

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
        Self::geometry(self.rotation)
            .vertices
            .map(|p| p + self.anchor1)
            .to_vec()
    }

    /// 指定された頂点と方向を基準にSpectreを生成する
    ///
    /// # Arguments
    /// * `vertex` - 基準点の座標
    /// * `index` - 基準点のインデックス
    /// * `edge_direction` - anchor_pointから出る辺の角度
    fn with_vertex(vertex: HexVec, index: usize, edge_direction: Angle) -> Self {
        let angle = edge_direction - Self::EDGE_DIRECTIONS[index];
        let geometry = Self::geometry(angle);
        let anchor1 = vertex - geometry.vertices[index];
        Self {
            rotation: angle,
            anchor1,
            bbox: geometry.bounds.translated(anchor1).to_aabb(),
        }
    }

    /// 指定された角度の方向ベクトルを計算する
    fn direction_vector(angle: Angle, direction: Angle) -> HexVec {
        let total_angle = angle + direction;
        HexVec::new(HexValue::cos(total_angle), HexValue::sin(total_angle))
    }

    fn vertex(&self, index: usize) -> HexVec {
        self.anchor1 + Self::geometry(self.rotation).vertices[index]
    }

    fn geometry(rotation: Angle) -> &'static Geometry {
        static GEOMETRY: OnceLock<[Geometry; 12]> = OnceLock::new();
        &GEOMETRY.get_or_init(|| {
            std::array::from_fn(|direction| {
                let rotation = Angle::new(direction as i32);
                let mut vertices = [HexVec::ZERO; Self::VERTEX_COUNT];
                for index in 1..Self::VERTEX_COUNT {
                    vertices[index] = vertices[index - 1]
                        + Self::direction_vector(rotation, Self::EDGE_DIRECTIONS[index - 1]);
                }
                Geometry {
                    bounds: ExactBounds::from_points(vertices),
                    vertices,
                }
            })
        })[rotation.value() as usize]
    }

    pub fn area() -> f64 {
        3.0 * (1.0 + 3.0_f64.sqrt())
    }

    /// Area covered inside the rectangle. Unlike a bounding-box or centroid
    /// check, clipping also detects gaps along the concave cluster boundary.
    pub fn area_in(&self, bbox: &Aabb) -> f64 {
        if !self.bbox.has_intersection(bbox) {
            return 0.0;
        }
        if bbox.contains_aabb(&self.bbox) {
            return Self::area();
        }

        // Work relative to the tile's anchor to avoid subtracting large products
        // in the shoelace formula. Each half-plane can at most double the number
        // of vertices; both buffers fit all four clipping steps without allocation.
        const CAPACITY: usize = Spectre::VERTEX_COUNT * 16;
        let anchor = DVec2::new(self.anchor1.x.to_f64(), self.anchor1.y.to_f64());
        let min = bbox.min.as_dvec2() - anchor;
        let max = bbox.max.as_dvec2() - anchor;
        let mut polygon = [DVec2::ZERO; CAPACITY];
        let mut scratch = [DVec2::ZERO; CAPACITY];
        for (out, vertex) in polygon
            .iter_mut()
            .zip(Self::geometry(self.rotation).vertices)
        {
            *out = DVec2::new(vertex.x.to_f64(), vertex.y.to_f64());
        }
        let (mut source, mut target) = (&mut polygon, &mut scratch);
        let mut count = Self::VERTEX_COUNT;
        for (axis, boundary, keep_greater) in [
            (0, min.x, true),
            (0, max.x, false),
            (1, min.y, true),
            (1, max.y, false),
        ] {
            if count == 0 {
                return 0.0;
            }
            let inside = |p: DVec2| {
                if keep_greater {
                    p[axis] >= boundary
                } else {
                    p[axis] <= boundary
                }
            };
            let mut output_count = 0;
            let mut previous = source[count - 1];
            for current in source[..count].iter().copied() {
                if inside(previous) != inside(current) {
                    let t = (boundary - previous[axis]) / (current[axis] - previous[axis]);
                    let mut intersection = previous + (current - previous) * t;
                    intersection[axis] = boundary;
                    target[output_count] = intersection;
                    output_count += 1;
                }
                if inside(current) {
                    target[output_count] = current;
                    output_count += 1;
                }
                previous = current;
            }
            count = output_count;
            std::mem::swap(&mut source, &mut target);
        }
        if count < 3 {
            return 0.0;
        }
        (0..count)
            .map(|index| source[index].perp_dot(source[(index + 1) % count]))
            .sum::<f64>()
            .abs()
            * 0.5
    }
}
