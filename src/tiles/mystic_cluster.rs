use crate::utils::{Aabb, Angle, HexVec};

use super::{Anchor, MIN_PARTIAL_CLUSTER_LEVEL, MysticLike, Skeleton, SpectreLike, bounds};

pub struct MysticCluster {
    pub(super) children: [SpectreLike; 6],
    pub(super) h: MysticLike,
    anchor1: HexVec,
    direction: Angle,
    level: u8,
    bbox: Aabb,
}

impl MysticCluster {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        a: impl Into<SpectreLike>,
        b: impl Into<SpectreLike>,
        c: impl Into<SpectreLike>,
        d: impl Into<SpectreLike>,
        f: impl Into<SpectreLike>,
        g: impl Into<SpectreLike>,
        h: impl Into<MysticLike>,
        level: usize,
    ) -> Self {
        let children = [a.into(), b.into(), c.into(), d.into(), f.into(), g.into()];
        let h = h.into();
        let anchor1 = children[5].coordinate(Anchor::Anchor3);
        let direction = children[5].edge_direction_from(Anchor::Anchor3);
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
        const CHILD_INDICES: [usize; 6] = [0, 1, 2, 3, 5, 6];
        let entry = bounds::table().entry(level, direction);
        let child_level = level - 1;
        let children = std::array::from_fn(|i| {
            let placement = entry.children[CHILD_INDICES[i]];
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
