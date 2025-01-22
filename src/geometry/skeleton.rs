use crate::{
    geometry::{Spectre, SpectreLike},
    utils::{Aabb, Angle, HexVec},
};

use super::{Anchor, SpectreCluster, MIN_PARTIAL_CLUSTER_LEVEL};

#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    anchor1: HexVec,
    anchor2: HexVec,
    anchor3: HexVec,
    anchor4: HexVec,
    edge_direction_into_anchor1: Angle,
    edge_direction_from_anchor1: Angle,
    edge_direction_into_anchor2: Angle,
    edge_direction_from_anchor2: Angle,
    edge_direction_into_anchor3: Angle,
    edge_direction_from_anchor3: Angle,
    edge_direction_into_anchor4: Angle,
    edge_direction_from_anchor4: Angle,
    level: usize,
}

impl Skeleton {
    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
        level: usize,
    ) -> Self {
        let coordinate = coordinate.into();
        let edge_direction = edge_direction.into();
        let (g, d, b, a) = match anchor {
            Anchor::Anchor1 => {
                let g = if level == 1 {
                    Spectre::with_anchor(Anchor::Anchor3, coordinate, edge_direction).into()
                } else {
                    Skeleton::with_anchor(Anchor::Anchor3, coordinate, edge_direction, level - 1)
                };
                let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                (g, d, b, a)
            }
            Anchor::Anchor2 => {
                let d = if level == 1 {
                    Spectre::with_anchor(Anchor::Anchor2, coordinate, edge_direction).into()
                } else {
                    Skeleton::with_anchor(Anchor::Anchor2, coordinate, edge_direction, level - 1)
                };
                let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                (g, d, b, a)
            }
            Anchor::Anchor3 => {
                let b = if level == 1 {
                    Spectre::with_anchor(Anchor::Anchor3, coordinate, edge_direction).into()
                } else {
                    Skeleton::with_anchor(Anchor::Anchor3, coordinate, edge_direction, level - 1)
                };
                let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.connected_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                (g, d, b, a)
            }
            Anchor::Anchor4 => {
                let a = if level == 1 {
                    Spectre::with_anchor(Anchor::Anchor2, coordinate, edge_direction).into()
                } else {
                    Skeleton::with_anchor(Anchor::Anchor2, coordinate, edge_direction, level - 1)
                };
                let b = a.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                (g, d, b, a)
            }
        };

        let anchor1 = g.coordinate(Anchor::Anchor3);
        let anchor2 = d.coordinate(Anchor::Anchor2);
        let anchor3 = b.coordinate(Anchor::Anchor3);
        let anchor4 = a.coordinate(Anchor::Anchor2);
        let edge_direction_into_anchor1 = g.edge_direction_into(Anchor::Anchor3);
        let edge_direction_from_anchor1 = g.edge_direction_from(Anchor::Anchor3);
        let edge_direction_into_anchor2 = d.edge_direction_into(Anchor::Anchor2);
        let edge_direction_from_anchor2 = d.edge_direction_from(Anchor::Anchor2);
        let edge_direction_into_anchor3 = b.edge_direction_into(Anchor::Anchor3);
        let edge_direction_from_anchor3 = b.edge_direction_from(Anchor::Anchor3);
        let edge_direction_into_anchor4 = a.edge_direction_into(Anchor::Anchor2);
        let edge_direction_from_anchor4 = a.edge_direction_from(Anchor::Anchor2);
        Self {
            anchor1,
            anchor2,
            anchor3,
            anchor4,
            edge_direction_into_anchor1,
            edge_direction_from_anchor1,
            edge_direction_into_anchor2,
            edge_direction_from_anchor2,
            edge_direction_into_anchor3,
            edge_direction_from_anchor3,
            edge_direction_into_anchor4,
            edge_direction_from_anchor4,
            level,
        }
    }

    pub fn connected_skeleton(&self, from_anchor: Anchor, to_anchor: Anchor) -> Skeleton {
        // 新しいSpectreの角度を計算
        // levelによって頂点を合わせる場合に接合する辺の選びかたが変わる
        let angle = if self.level % 2 == 1 {
            // 奇数番目のlevelでは新しいSpectreClusterを辺が密着するまで時計回りに回転させる
            self.edge_direction_into(from_anchor).opposite()
        } else {
            // 偶数番目のlevelでは反時計回りに回転させる
            let rotation = self.edge_direction_from(to_anchor)
                - self.edge_direction_into(to_anchor).opposite();
            self.edge_direction_from(from_anchor) + rotation
        };

        // selfをコピー
        let mut new_skeleton = *self;

        // 回転角度を計算（目標のangleと現在のedge_direction(to_anchor)の差分）
        let rotation = angle - self.edge_direction_from(to_anchor);

        // 全てのanchorを回転
        let base = self.coordinate(to_anchor);
        new_skeleton.anchor1 = self.anchor1.rotate(base, rotation);
        new_skeleton.anchor2 = self.anchor2.rotate(base, rotation);
        new_skeleton.anchor3 = self.anchor3.rotate(base, rotation);
        new_skeleton.anchor4 = self.anchor4.rotate(base, rotation);

        // 全てのangleを回転
        new_skeleton.edge_direction_into_anchor1 += rotation;
        new_skeleton.edge_direction_from_anchor1 += rotation;
        new_skeleton.edge_direction_into_anchor2 += rotation;
        new_skeleton.edge_direction_from_anchor2 += rotation;
        new_skeleton.edge_direction_into_anchor3 += rotation;
        new_skeleton.edge_direction_from_anchor3 += rotation;
        new_skeleton.edge_direction_into_anchor4 += rotation;
        new_skeleton.edge_direction_from_anchor4 += rotation;

        // 平行移動（self.point(from_anchor)とnew_skeleton.point(to_anchor)を一致させる）
        let offset = self.coordinate(from_anchor) - new_skeleton.coordinate(to_anchor);
        new_skeleton.anchor1 += offset;
        new_skeleton.anchor2 += offset;
        new_skeleton.anchor3 += offset;
        new_skeleton.anchor4 += offset;

        new_skeleton
    }

    pub fn to_spectre_cluster(&self, bbox: &Aabb) -> SpectreCluster {
        if self.level < MIN_PARTIAL_CLUSTER_LEVEL {
            // 小さいlevelのSkeletonはそのままClusterに変換
            return SpectreCluster::with_anchor(
                Anchor::Anchor1,
                self.anchor1,
                self.edge_direction_from_anchor1,
                self.level,
            );
        }

        let mut sub_spectre_likes = self
            .into_sub_skeletons()
            .into_iter()
            .map(|sub_skeleton| {
                if sub_skeleton.bbox().has_intersection(bbox) {
                    SpectreLike::from(sub_skeleton.to_spectre_cluster(bbox))
                } else {
                    SpectreLike::Skeleton(sub_skeleton)
                }
            })
            .collect::<Vec<_>>();
        let h = sub_spectre_likes.pop().unwrap().into_mystic_like();
        let g = sub_spectre_likes.pop().unwrap();
        let f = sub_spectre_likes.pop().unwrap();
        let e = sub_spectre_likes.pop().unwrap();
        let d = sub_spectre_likes.pop().unwrap();
        let c = sub_spectre_likes.pop().unwrap();
        let b = sub_spectre_likes.pop().unwrap();
        let a = sub_spectre_likes.pop().unwrap();

        SpectreCluster::new(
            Box::new(a),
            Box::new(b),
            Box::new(c),
            Box::new(d),
            Box::new(e),
            Box::new(f),
            Box::new(g),
            Box::new(h),
            self.level,
        )
    }

    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.anchor1,
            Anchor::Anchor2 => self.anchor2,
            Anchor::Anchor3 => self.anchor3,
            Anchor::Anchor4 => self.anchor4,
        }
    }

    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.edge_direction_from_anchor1,
            Anchor::Anchor2 => self.edge_direction_from_anchor2,
            Anchor::Anchor3 => self.edge_direction_from_anchor3,
            Anchor::Anchor4 => self.edge_direction_from_anchor4,
        }
    }

    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.edge_direction_into_anchor1,
            Anchor::Anchor2 => self.edge_direction_into_anchor2,
            Anchor::Anchor3 => self.edge_direction_into_anchor3,
            Anchor::Anchor4 => self.edge_direction_into_anchor4,
        }
    }

    pub fn bbox(&self) -> Aabb {
        let points = [self.anchor1, self.anchor2, self.anchor3, self.anchor4];
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

        let expanded_min_x = min_x - (max_x - min_x) * 0.5;
        let expanded_min_y = min_y - (max_y - min_y) * 0.5;
        let expanded_max_x = max_x + (max_x - min_x) * 0.5;
        let expanded_max_y = max_y + (max_y - min_y) * 0.5;

        Aabb::new(
            expanded_min_x,
            expanded_min_y,
            expanded_max_x,
            expanded_max_y,
        )
    }

    pub fn level(&self) -> usize {
        self.level
    }

    /// 一つ下のlevelのskeletonのリストに変換
    fn into_sub_skeletons(self) -> [Skeleton; 8] {
        let a = if self.level == 1 {
            Spectre::with_anchor(
                Anchor::Anchor2,
                self.anchor4,
                self.edge_direction_from(Anchor::Anchor4),
            )
            .into()
        } else {
            Skeleton::with_anchor(
                Anchor::Anchor2,
                self.anchor4,
                self.edge_direction_from(Anchor::Anchor4),
                self.level - 1,
            )
        };
        let b = a.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let f = e.connected_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let g = f.connected_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.connected_skeleton(Anchor::Anchor4, Anchor::Anchor4);

        [a, b, c, d, e, f, g, h]
    }
}

impl From<Spectre> for Skeleton {
    fn from(spectre: Spectre) -> Self {
        let anchor1 = spectre.coordinate(Anchor::Anchor1);
        let anchor2 = spectre.coordinate(Anchor::Anchor2);
        let anchor3 = spectre.coordinate(Anchor::Anchor3);
        let anchor4 = spectre.coordinate(Anchor::Anchor4);
        let edge_direction_into_anchor1 = spectre.edge_direction_into(Anchor::Anchor1);
        let edge_direction_from_anchor1 = spectre.edge_direction_from(Anchor::Anchor1);
        let edge_direction_into_anchor2 = spectre.edge_direction_into(Anchor::Anchor2);
        let edge_direction_from_anchor2 = spectre.edge_direction_from(Anchor::Anchor2);
        let edge_direction_into_anchor3 = spectre.edge_direction_into(Anchor::Anchor3);
        let edge_direction_from_anchor3 = spectre.edge_direction_from(Anchor::Anchor3);
        let edge_direction_into_anchor4 = spectre.edge_direction_into(Anchor::Anchor4);
        let edge_direction_from_anchor4 = spectre.edge_direction_from(Anchor::Anchor4);
        let level = 0;
        Self {
            anchor1,
            anchor2,
            anchor3,
            anchor4,
            edge_direction_into_anchor1,
            edge_direction_from_anchor1,
            edge_direction_into_anchor2,
            edge_direction_from_anchor2,
            edge_direction_into_anchor3,
            edge_direction_from_anchor3,
            edge_direction_into_anchor4,
            edge_direction_from_anchor4,
            level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_and_spectre_cluster_equivalence() {
        let test_cases = [
            // Test with different angles (0, 2, 4, 6 corresponding to 0, π/3, 2π/3, π)
            (1, HexVec::ZERO, Anchor::Anchor1, Angle::new(0)),
            (1, HexVec::ZERO, Anchor::Anchor2, Angle::new(2)),
            (2, HexVec::ZERO, Anchor::Anchor3, Angle::new(4)),
            (2, HexVec::ZERO, Anchor::Anchor4, Angle::new(6)),
            (3, HexVec::ZERO, Anchor::Anchor1, Angle::new(8)),
            (3, HexVec::ZERO, Anchor::Anchor2, Angle::new(10)),
        ];

        for &(level, coordinate, anchor, edge_direction) in &test_cases {
            let skeleton = Skeleton::with_anchor(anchor, coordinate, edge_direction, level);
            let cluster = SpectreCluster::with_anchor(anchor, coordinate, edge_direction, level);

            for test_anchor in [
                Anchor::Anchor1,
                Anchor::Anchor2,
                Anchor::Anchor3,
                Anchor::Anchor4,
            ] {
                assert_eq!(
                    skeleton.coordinate(test_anchor),
                    cluster.coordinate(test_anchor),
                    "Point mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );

                assert_eq!(
                    skeleton.edge_direction_from(test_anchor),
                    cluster.edge_direction_from(test_anchor),
                    "Edge direction mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );

                assert_eq!(
                    skeleton.edge_direction_into(test_anchor),
                    cluster.edge_direction_into(test_anchor),
                    "Reverse edge direction mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );
            }
        }
    }
}
