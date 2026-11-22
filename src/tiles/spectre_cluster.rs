use crate::utils::{Aabb, Angle, HexVec};

use super::{
    Anchor, MAX_CLUSTER_LEVEL, MIN_PARTIAL_CLUSTER_LEVEL, MysticCluster, MysticLike, Skeleton,
    SpectreIter, SpectreLike, bounds,
};

pub struct SpectreCluster {
    pub(super) children: [SpectreLike; 7],
    pub(super) h: MysticLike,
    anchor1: HexVec,
    direction: Angle,
    level: u8,
    bbox: Aabb,
}

impl SpectreCluster {
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
        let children = [
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
        ];
        let h = h.into();
        debug_assert_eq!(
            h.coordinate(Anchor::Anchor1),
            children[0].coordinate(Anchor::Anchor1)
        );
        debug_assert_eq!(
            children[0].coordinate(Anchor::Anchor3),
            children[1].coordinate(Anchor::Anchor1)
        );
        debug_assert_eq!(
            children[1].coordinate(Anchor::Anchor4),
            children[2].coordinate(Anchor::Anchor2)
        );
        debug_assert_eq!(
            children[2].coordinate(Anchor::Anchor3),
            children[3].coordinate(Anchor::Anchor1)
        );
        debug_assert_eq!(
            children[3].coordinate(Anchor::Anchor3),
            children[4].coordinate(Anchor::Anchor1)
        );
        debug_assert_eq!(
            children[4].coordinate(Anchor::Anchor4),
            children[5].coordinate(Anchor::Anchor2)
        );
        debug_assert_eq!(
            children[5].coordinate(Anchor::Anchor3),
            children[6].coordinate(Anchor::Anchor1)
        );
        debug_assert_eq!(
            children[6].coordinate(Anchor::Anchor4),
            h.coordinate(Anchor::Anchor4)
        );

        let anchor1 = children[6].coordinate(Anchor::Anchor3);
        let direction = children[6].edge_direction_from(Anchor::Anchor3);

        // Bounds include both materialized children and unloaded skeletons.
        let mut bbox = h.bbox();
        for child in &children {
            bbox = bbox.union(&child.bbox());
        }

        Self {
            children,
            h,
            anchor1,
            direction,
            level: level as u8,
            bbox,
        }
    }

    pub(crate) fn from_anchor1(anchor1: HexVec, direction: Angle, level: usize) -> Self {
        let entry = bounds::table().entry(level, direction);
        let child_level = level - 1;
        let children = std::array::from_fn(|index| {
            let placement = entry.children[index];
            SpectreLike::from_anchor1(anchor1 + placement.offset, placement.direction, child_level)
        });
        let h_placement = entry.children[7];
        let h = MysticLike::from_anchor1(
            anchor1 + h_placement.offset,
            h_placement.direction,
            child_level,
        );

        let mut bbox = h.bbox();
        for child in &children {
            bbox = bbox.union(&child.bbox());
        }

        Self {
            children,
            h,
            anchor1,
            direction,
            level: level as u8,
            bbox,
        }
    }

    pub fn with_anchor(
        anchor: Anchor,
        coordinate: impl Into<HexVec>,
        edge_direction: impl Into<Angle>,
        level: usize,
    ) -> Self {
        assert!(
            (1..=MAX_CLUSTER_LEVEL).contains(&level),
            "unsupported cluster level: {level}"
        );
        let (anchor1, direction) =
            bounds::table().resolve_anchor(level, anchor, coordinate.into(), edge_direction.into());
        Self::from_anchor1(anchor1, direction, level)
    }

    pub fn with_child_a(a: SpectreCluster) -> Self {
        Self::with_child_at(a, 0)
    }

    pub fn with_child_f(f: SpectreCluster) -> Self {
        Self::with_child_at(f, 5)
    }

    fn with_child_at(child: SpectreCluster, index: usize) -> Self {
        let child_level = child.level();
        let level = child_level + 1;
        assert!(
            level <= MAX_CLUSTER_LEVEL,
            "unsupported cluster level: {level}"
        );
        let table = bounds::table();
        let (anchor1, direction) =
            table.wrap_child(child_level, child.anchor1, child.direction, index);
        let entry = table.entry(level, direction);
        let mut child_opt = Some(child);
        let children: [SpectreLike; 7] = std::array::from_fn(|i| {
            if i == index {
                child_opt.take().unwrap().into()
            } else {
                let placement = entry.children[i];
                Skeleton::from_anchor1(
                    anchor1 + placement.offset,
                    placement.direction,
                    child_level,
                    None,
                )
                .into()
            }
        });
        let h_placement = entry.children[7];
        let h = MysticLike::Skeleton(Skeleton::from_anchor1(
            anchor1 + h_placement.offset,
            h_placement.direction,
            child_level,
            None,
        ));

        let mut bbox = h.bbox();
        for c in &children {
            bbox = bbox.union(&c.bbox());
        }

        Self {
            children,
            h,
            anchor1,
            direction,
            level: level as u8,
            bbox,
        }
    }

    pub fn connected_cluster(&self, from_anchor: Anchor, to_anchor: Anchor) -> SpectreCluster {
        let level = self.level as usize;
        let (anchor1, direction) =
            bounds::table().connected(level, self.anchor1, self.direction, from_anchor, to_anchor);
        Self::from_anchor1(anchor1, direction, level)
    }

    pub fn into_mystic_cluster(self) -> MysticCluster {
        let [a, b, c, d, _e, f, g] = self.children;
        MysticCluster::new(a, b, c, d, f, g, self.h, self.level as usize)
    }

    #[inline]
    pub fn to_skeleton(&self) -> Skeleton {
        Skeleton::from_anchor1(
            self.anchor1,
            self.direction,
            self.level as usize,
            Some(self.bbox),
        )
    }

    pub fn update(&mut self, bbox: &Aabb) {
        if (self.level as usize) < MIN_PARTIAL_CLUSTER_LEVEL {
            return;
        }
        for child in &mut self.children {
            child.update(bbox);
        }
        self.h.update(bbox);
        let mut new_bbox = self.h.bbox();
        for child in &self.children {
            new_bbox = new_bbox.union(&child.bbox());
        }
        self.bbox = new_bbox;
    }

    #[inline]
    pub fn spectres_in(&self, bbox: Aabb) -> SpectreIter<'_> {
        SpectreIter::new(self, bbox)
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
    pub fn bbox(&self) -> Aabb {
        self.bbox
    }

    #[inline]
    pub fn level(&self) -> usize {
        self.level as usize
    }
}
