use crate::utils::{Aabb, Angle, HexVec};

use super::{
    Anchor, MAX_CLUSTER_LEVEL, MIN_PARTIAL_CLUSTER_LEVEL, MysticCluster, MysticLike, Spectre,
    SpectreCluster, SpectreLike, bounds,
};

#[derive(Clone, Copy, Debug)]
pub struct Skeleton {
    anchor1: HexVec,
    direction: Angle,
    level: u8,
    inherited_bbox: Option<Aabb>,
}

impl Skeleton {
    #[inline]
    pub(crate) fn from_anchor1(
        anchor1: HexVec,
        direction: Angle,
        level: usize,
        inherited_bbox: Option<Aabb>,
    ) -> Self {
        Self {
            anchor1,
            direction,
            level: level as u8,
            inherited_bbox,
        }
    }

    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
        level: usize,
        inherited_bbox: Option<Aabb>,
    ) -> Self {
        assert!(
            (1..=MAX_CLUSTER_LEVEL).contains(&level),
            "unsupported cluster level: {level}"
        );
        let (anchor1, direction) =
            bounds::table().resolve_anchor(level, anchor, coordinate.into(), edge_direction.into());
        Self::from_anchor1(anchor1, direction, level, inherited_bbox)
    }

    #[inline]
    pub fn connected_skeleton(&self, from_anchor: Anchor, to_anchor: Anchor) -> Skeleton {
        let level = self.level as usize;
        let (anchor1, direction) =
            bounds::table().connected(level, self.anchor1, self.direction, from_anchor, to_anchor);
        Self::from_anchor1(anchor1, direction, level, None)
    }

    #[inline]
    pub(crate) fn wrap_as_child(&self, child_index: usize) -> Self {
        let child_level = self.level as usize;
        let parent_level = child_level + 1;
        assert!(
            parent_level <= MAX_CLUSTER_LEVEL,
            "unsupported cluster level: {parent_level}"
        );
        let (anchor1, direction) =
            bounds::table().wrap_child(child_level, self.anchor1, self.direction, child_index);
        Self::from_anchor1(anchor1, direction, parent_level, None)
    }

    pub fn to_spectre_cluster(&self, bbox: &Aabb) -> SpectreCluster {
        let level = self.level as usize;
        if level < MIN_PARTIAL_CLUSTER_LEVEL {
            // 小さいlevelのSkeletonはそのままClusterに変換
            return SpectreCluster::from_anchor1(self.anchor1, self.direction, level);
        }

        let children = self.split_into_skeletons();
        let make_spectre = |sub: Skeleton| {
            if sub.estimated_bbox().has_intersection(bbox) {
                SpectreLike::from(sub.to_spectre_cluster(bbox))
            } else {
                SpectreLike::from(sub)
            }
        };
        let [a, b, c, d, e, f, g, h] = children;
        let h = if h.estimated_mystic_bbox().has_intersection(bbox) {
            MysticLike::from(h.to_mystic_cluster(bbox))
        } else {
            MysticLike::from(h)
        };

        SpectreCluster::new(
            make_spectre(a),
            make_spectre(b),
            make_spectre(c),
            make_spectre(d),
            make_spectre(e),
            make_spectre(f),
            make_spectre(g),
            h,
            level,
        )
    }

    pub(crate) fn to_mystic_cluster(self, bbox: &Aabb) -> MysticCluster {
        let level = self.level as usize;
        if level < MIN_PARTIAL_CLUSTER_LEVEL {
            return MysticCluster::from_anchor1(self.anchor1, self.direction, level);
        }

        let [a, b, c, d, _e, f, g, h] = self.split_into_skeletons();
        let make_spectre = |sub: Skeleton| {
            if sub.estimated_bbox().has_intersection(bbox) {
                SpectreLike::from(sub.to_spectre_cluster(bbox))
            } else {
                SpectreLike::from(sub)
            }
        };
        let h = if h.estimated_mystic_bbox().has_intersection(bbox) {
            MysticLike::from(h.to_mystic_cluster(bbox))
        } else {
            MysticLike::from(h)
        };

        MysticCluster::new(
            make_spectre(a),
            make_spectre(b),
            make_spectre(c),
            make_spectre(d),
            make_spectre(f),
            make_spectre(g),
            h,
            level,
        )
    }

    #[inline]
    pub(crate) fn anchor1(&self) -> HexVec {
        self.anchor1
    }

    #[inline]
    pub(crate) fn direction(&self) -> Angle {
        self.direction
    }

    #[inline]
    pub fn coordinate(&self, anchor: Anchor) -> HexVec {
        match anchor {
            Anchor::Anchor1 => self.anchor1,
            _ => {
                self.anchor1
                    + bounds::table()
                        .entry(self.level as usize, self.direction)
                        .anchors[anchor.slot()]
            }
        }
    }

    #[inline]
    pub fn edge_direction_from(&self, anchor: Anchor) -> Angle {
        match anchor {
            Anchor::Anchor1 => self.direction,
            _ => {
                bounds::table()
                    .entry(self.level as usize, self.direction)
                    .edge_from[anchor.slot()]
            }
        }
    }

    #[inline]
    pub fn edge_direction_into(&self, anchor: Anchor) -> Angle {
        bounds::table()
            .entry(self.level as usize, self.direction)
            .edge_into[anchor.slot()]
    }

    #[inline]
    pub fn estimated_bbox(&self) -> Aabb {
        if let Some(inherited_bbox) = self.inherited_bbox {
            return inherited_bbox;
        }
        bounds::table().bounds_aabb(self.level as usize, self.direction, self.anchor1, false)
    }

    #[inline]
    pub(crate) fn estimated_mystic_bbox(&self) -> Aabb {
        if let Some(inherited_bbox) = self.inherited_bbox {
            return inherited_bbox;
        }
        bounds::table().bounds_aabb(self.level as usize, self.direction, self.anchor1, true)
    }

    #[inline]
    pub fn level(&self) -> usize {
        self.level as usize
    }

    /// 一つ下のlevelのskeletonのリストに変換
    #[inline]
    pub(super) fn split_into_skeletons(self) -> [Skeleton; 8] {
        let level = self.level as usize;
        let child_level = level - 1;
        let entry = bounds::table().entry(level, self.direction);
        entry.children.map(|child| {
            Skeleton::from_anchor1(
                self.anchor1 + child.offset,
                child.direction,
                child_level,
                None,
            )
        })
    }
}

impl From<Spectre> for Skeleton {
    #[inline]
    fn from(spectre: Spectre) -> Self {
        Self {
            anchor1: spectre.anchor1(),
            direction: spectre.rotation(),
            level: 0,
            inherited_bbox: Some(spectre.bbox()),
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
            let skeleton = Skeleton::with_anchor(anchor, coordinate, edge_direction, level, None);
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
