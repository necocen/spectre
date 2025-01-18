use crate::{
    geometry::{Spectre, SpectreLike},
    utils::{Angle, HexVec},
};

use super::{Aabb, Anchor, Geometry, SuperSpectre};

#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    anchor1: HexVec,
    anchor2: HexVec,
    anchor3: HexVec,
    anchor4: HexVec,
    before_anchor1: Angle,
    after_anchor1: Angle,
    before_anchor2: Angle,
    after_anchor2: Angle,
    before_anchor3: Angle,
    after_anchor3: Angle,
    before_anchor4: Angle,
    after_anchor4: Angle,
    pub level: usize,
}

impl Geometry for Skeleton {
    fn point(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.anchor1,
            Anchor::Anchor2 => self.anchor2,
            Anchor::Anchor3 => self.anchor3,
            Anchor::Anchor4 => self.anchor4,
        }
    }

    fn edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.after_anchor1,
            Anchor::Anchor2 => self.after_anchor2,
            Anchor::Anchor3 => self.after_anchor3,
            Anchor::Anchor4 => self.after_anchor4,
        }
    }

    fn rev_edge_direction(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.before_anchor1,
            Anchor::Anchor2 => self.before_anchor2,
            Anchor::Anchor3 => self.before_anchor3,
            Anchor::Anchor4 => self.before_anchor4,
        }
    }

    fn aabb(&self) -> Aabb {
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

        Aabb::new(min_x, min_y, max_x, max_y)
    }
}

impl From<Spectre> for Skeleton {
    fn from(spectre: Spectre) -> Self {
        let anchor1 = spectre.point(Anchor::Anchor1);
        let anchor2 = spectre.point(Anchor::Anchor2);
        let anchor3 = spectre.point(Anchor::Anchor3);
        let anchor4 = spectre.point(Anchor::Anchor4);
        let before_anchor1 = spectre.rev_edge_direction(Anchor::Anchor1);
        let after_anchor1 = spectre.edge_direction(Anchor::Anchor1);
        let before_anchor2 = spectre.rev_edge_direction(Anchor::Anchor2);
        let after_anchor2 = spectre.edge_direction(Anchor::Anchor2);
        let before_anchor3 = spectre.rev_edge_direction(Anchor::Anchor3);
        let after_anchor3 = spectre.edge_direction(Anchor::Anchor3);
        let before_anchor4 = spectre.rev_edge_direction(Anchor::Anchor4);
        let after_anchor4 = spectre.edge_direction(Anchor::Anchor4);
        let level = 0;
        Self {
            anchor1,
            anchor2,
            anchor3,
            anchor4,
            before_anchor1,
            after_anchor1,
            before_anchor2,
            after_anchor2,
            before_anchor3,
            after_anchor3,
            before_anchor4,
            after_anchor4,
            level,
        }
    }
}

impl Skeleton {
    pub fn new_with_anchor(
        level: usize,
        anchor_point: impl Into<HexVec>,
        anchor: Anchor,
        angle: impl Into<Angle>,
    ) -> Self {
        let anchor_point = anchor_point.into();
        let angle = angle.into();
        let (g, d, b, a) = match anchor {
            Anchor::Anchor1 => {
                let g = if level == 1 {
                    Spectre::new_with_anchor(anchor_point, Anchor::Anchor3, angle).into()
                } else {
                    Skeleton::new_with_anchor(level - 1, anchor_point, Anchor::Anchor3, angle)
                };
                let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                (g, d, b, a)
            }
            Anchor::Anchor2 => {
                let d = if level == 1 {
                    Spectre::new_with_anchor(anchor_point, Anchor::Anchor2, angle).into()
                } else {
                    Skeleton::new_with_anchor(level - 1, anchor_point, Anchor::Anchor2, angle)
                };
                let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                let b = a.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                (g, d, b, a)
            }
            Anchor::Anchor3 => {
                let b = if level == 1 {
                    Spectre::new_with_anchor(anchor_point, Anchor::Anchor3, angle).into()
                } else {
                    Skeleton::new_with_anchor(level - 1, anchor_point, Anchor::Anchor3, angle)
                };
                let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                let a = h.adjacent_skeleton(Anchor::Anchor1, Anchor::Anchor1);
                (g, d, b, a)
            }
            Anchor::Anchor4 => {
                let a = if level == 1 {
                    Spectre::new_with_anchor(anchor_point, Anchor::Anchor2, angle).into()
                } else {
                    Skeleton::new_with_anchor(level - 1, anchor_point, Anchor::Anchor2, angle)
                };
                let b = a.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
                let g = f.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
                // let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);
                (g, d, b, a)
            }
        };

        let anchor1 = g.point(Anchor::Anchor3);
        let anchor2 = d.point(Anchor::Anchor2);
        let anchor3 = b.point(Anchor::Anchor3);
        let anchor4 = a.point(Anchor::Anchor2);
        let before_anchor1 = g.rev_edge_direction(Anchor::Anchor3);
        let after_anchor1 = g.edge_direction(Anchor::Anchor3);
        let before_anchor2 = d.rev_edge_direction(Anchor::Anchor2);
        let after_anchor2 = d.edge_direction(Anchor::Anchor2);
        let before_anchor3 = b.rev_edge_direction(Anchor::Anchor3);
        let after_anchor3 = b.edge_direction(Anchor::Anchor3);
        let before_anchor4 = a.rev_edge_direction(Anchor::Anchor2);
        let after_anchor4 = a.edge_direction(Anchor::Anchor2);
        Self {
            anchor1,
            anchor2,
            anchor3,
            anchor4,
            before_anchor1,
            after_anchor1,
            before_anchor2,
            after_anchor2,
            before_anchor3,
            after_anchor3,
            before_anchor4,
            after_anchor4,
            level,
        }
    }

    pub fn adjacent_skeleton(&self, from_anchor: Anchor, to_anchor: Anchor) -> Skeleton {
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

        // selfをコピー
        let mut new_skeleton = *self;

        // 回転角度を計算（目標のangleと現在のedge_direction(to_anchor)の差分）
        let rotation = angle - self.edge_direction(to_anchor);

        // 全てのanchorを回転
        let base = self.point(to_anchor);
        new_skeleton.anchor1 = self.anchor1.rotate(base, rotation);
        new_skeleton.anchor2 = self.anchor2.rotate(base, rotation);
        new_skeleton.anchor3 = self.anchor3.rotate(base, rotation);
        new_skeleton.anchor4 = self.anchor4.rotate(base, rotation);

        // 全てのangleを回転
        new_skeleton.before_anchor1 += rotation;
        new_skeleton.after_anchor1 += rotation;
        new_skeleton.before_anchor2 += rotation;
        new_skeleton.after_anchor2 += rotation;
        new_skeleton.before_anchor3 += rotation;
        new_skeleton.after_anchor3 += rotation;
        new_skeleton.before_anchor4 += rotation;
        new_skeleton.after_anchor4 += rotation;

        // 平行移動（self.point(from_anchor)とnew_skeleton.point(to_anchor)を一致させる）
        let offset = self.point(from_anchor) - new_skeleton.point(to_anchor);
        new_skeleton.anchor1 += offset;
        new_skeleton.anchor2 += offset;
        new_skeleton.anchor3 += offset;
        new_skeleton.anchor4 += offset;

        new_skeleton
    }

    pub fn to_super_spectre(&self, aabb: &Aabb) -> SuperSpectre {
        if self.level <= 5 {
            // 小さいlevelのSkeletonはそのままSuperSpectreに変換
            return SuperSpectre::new_with_anchor(
                self.level,
                self.anchor1,
                Anchor::Anchor1,
                self.after_anchor1,
            );
        }

        let mut sub_spectre_likes = self
            .to_sub_skeletons()
            .into_iter()
            .map(|sub_skeleton| {
                if sub_skeleton.aabb().has_intersection(aabb) {
                    SpectreLike::from(sub_skeleton.to_super_spectre(aabb))
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

        SuperSpectre::new(
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

    /// 一つ下のlevelのskeletonのリストに変換
    fn to_sub_skeletons(self) -> [Skeleton; 8] {
        let a = if self.level == 1 {
            Spectre::new_with_anchor(
                self.anchor4,
                Anchor::Anchor2,
                self.edge_direction(Anchor::Anchor4),
            )
            .into()
        } else {
            Skeleton::new_with_anchor(
                self.level - 1,
                self.anchor4,
                Anchor::Anchor2,
                self.edge_direction(Anchor::Anchor4),
            )
        };
        let b = a.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let c = b.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let d = c.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let e = d.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let f = e.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor2);
        let g = f.adjacent_skeleton(Anchor::Anchor3, Anchor::Anchor1);
        let h = g.adjacent_skeleton(Anchor::Anchor4, Anchor::Anchor4);

        [a, b, c, d, e, f, g, h]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_and_super_spectre_equivalence() {
        let test_cases = [
            // Test with different angles (0, 2, 4, 6 corresponding to 0, π/3, 2π/3, π)
            (1, HexVec::ZERO, Anchor::Anchor1, Angle::new(0)),
            (1, HexVec::ZERO, Anchor::Anchor2, Angle::new(2)),
            (2, HexVec::ZERO, Anchor::Anchor3, Angle::new(4)),
            (2, HexVec::ZERO, Anchor::Anchor4, Angle::new(6)),
            (3, HexVec::ZERO, Anchor::Anchor1, Angle::new(8)),
            (3, HexVec::ZERO, Anchor::Anchor2, Angle::new(10)),
        ];

        for &(level, point, anchor, angle) in &test_cases {
            let skeleton = Skeleton::new_with_anchor(level, point, anchor, angle);
            let super_spectre = SuperSpectre::new_with_anchor(level, point, anchor, angle);

            for test_anchor in [
                Anchor::Anchor1,
                Anchor::Anchor2,
                Anchor::Anchor3,
                Anchor::Anchor4,
            ] {
                assert_eq!(
                    skeleton.point(test_anchor),
                    super_spectre.point(test_anchor),
                    "Point mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );

                assert_eq!(
                    skeleton.edge_direction(test_anchor),
                    super_spectre.edge_direction(test_anchor),
                    "Edge direction mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );

                assert_eq!(
                    skeleton.rev_edge_direction(test_anchor),
                    super_spectre.rev_edge_direction(test_anchor),
                    "Reverse edge direction mismatch for level {}, anchor {:?} at test_anchor {:?}",
                    level,
                    anchor,
                    test_anchor
                );
            }
        }
    }
}
