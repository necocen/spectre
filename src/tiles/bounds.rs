use std::sync::OnceLock;

use mikage::glam::DVec2;

use super::{Anchor, EDGE_CHAIN, MAX_CLUSTER_LEVEL, Spectre};
use crate::utils::{Aabb, Angle, HexVec};

const SQRT_3_F64: f64 = 1.7320508075688772;

/// Extrema stay in the exact coordinate system until translated to world space.
/// `magnitude` bounds the sum of the absolute terms in each coordinate, including
/// cancellation, so the resulting AABB also contains f32-rounded tile vertices.
#[derive(Clone, Copy)]
pub(crate) struct ExactBounds {
    min: HexVec,
    max: HexVec,
    magnitude: DVec2,
}

impl ExactBounds {
    #[inline]
    fn term_magnitude(p: HexVec) -> DVec2 {
        let coordinate = |value: crate::utils::HexValue| {
            (f64::from(value.rational).abs() + f64::from(value.irrational).abs() * SQRT_3_F64) * 0.5
        };
        DVec2::new(coordinate(p.x), coordinate(p.y))
    }

    pub(crate) fn from_points(points: impl IntoIterator<Item = HexVec>) -> Self {
        let mut points = points.into_iter();
        let first = points.next().expect("a tile has vertices");
        let mut bounds = Self {
            min: first,
            max: first,
            magnitude: Self::term_magnitude(first),
        };
        for point in points {
            bounds.min.x = bounds.min.x.min(point.x);
            bounds.min.y = bounds.min.y.min(point.y);
            bounds.max.x = bounds.max.x.max(point.x);
            bounds.max.y = bounds.max.y.max(point.y);
            bounds.magnitude = bounds.magnitude.max(Self::term_magnitude(point));
        }
        bounds
    }

    fn union(self, other: Self) -> Self {
        Self {
            min: HexVec::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: HexVec::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
            magnitude: self.magnitude.max(other.magnitude),
        }
    }

    #[inline]
    pub(crate) fn translated(self, offset: HexVec) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
            magnitude: self.magnitude + Self::term_magnitude(offset),
        }
    }

    #[inline]
    pub(crate) fn to_aabb(self) -> Aabb {
        let error = self.magnitude * (4.0 * f64::from(f32::EPSILON));
        Aabb::new(
            ((self.min.x.to_f64() - error.x) as f32).next_down(),
            ((self.min.y.to_f64() - error.y) as f32).next_down(),
            ((self.max.x.to_f64() + error.x) as f32).next_up(),
            ((self.max.y.to_f64() + error.y) as f32).next_up(),
        )
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ChildPlacement {
    pub(crate) offset: HexVec,
    pub(crate) direction: Angle,
}

impl ChildPlacement {
    const ZERO: Self = Self {
        offset: HexVec::ZERO,
        direction: Angle::ZERO,
    };
}

pub(crate) struct LevelEntry {
    pub(crate) anchors: [HexVec; 4],
    pub(crate) edge_from: [Angle; 4],
    pub(crate) edge_into: [Angle; 4],
    pub(crate) spectre_bounds: ExactBounds,
    pub(crate) mystic_bounds: ExactBounds,
    pub(crate) children: [ChildPlacement; 8],
}

pub(crate) struct LevelTable {
    pub(crate) entries: [LevelEntry; 12],
}

pub(crate) struct SubstitutionTable {
    pub(crate) levels: Vec<LevelTable>,
}

impl SubstitutionTable {
    #[inline]
    pub(crate) fn entry(&self, level: usize, direction: Angle) -> &LevelEntry {
        &self.levels[level].entries[direction.value() as usize]
    }

    #[inline]
    pub(crate) fn bounds_aabb(
        &self,
        level: usize,
        direction: Angle,
        anchor1: HexVec,
        is_mystic: bool,
    ) -> Aabb {
        let entry = self.entry(level, direction);
        let bounds = if is_mystic {
            entry.mystic_bounds
        } else {
            entry.spectre_bounds
        };
        bounds.translated(anchor1).to_aabb()
    }

    /// Resolve `(anchor1, direction)` in O(1) from any anchor coordinate and outgoing edge direction.
    #[inline]
    pub(crate) fn resolve_anchor(
        &self,
        level: usize,
        anchor: Anchor,
        coordinate: HexVec,
        edge_direction: Angle,
    ) -> (HexVec, Angle) {
        let slot = anchor.slot();
        let rel_dir = self.levels[level].entries[0].edge_from[slot];
        let direction = edge_direction - rel_dir;
        let anchor1 = coordinate - self.entry(level, direction).anchors[slot];
        (anchor1, direction)
    }

    /// Compute the neighbor `(anchor1, direction)` connected from `from_anchor` to `to_anchor` in O(1).
    #[inline]
    pub(crate) fn connected(
        &self,
        level: usize,
        anchor1: HexVec,
        direction: Angle,
        from_anchor: Anchor,
        to_anchor: Anchor,
    ) -> (HexVec, Angle) {
        let curr = self.entry(level, direction);
        let from_slot = from_anchor.slot();
        let to_slot = to_anchor.slot();
        let target_edge = if level % 2 == 1 {
            curr.edge_into[from_slot].opposite()
        } else {
            let rotation = curr.edge_from[to_slot] - curr.edge_into[to_slot].opposite();
            curr.edge_from[from_slot] + rotation
        };
        let join_point = anchor1 + curr.anchors[from_slot];
        self.resolve_anchor(level, to_anchor, join_point, target_edge)
    }

    /// Compute the parent `(anchor1, direction)` that contains the given child at `child_index` in O(1).
    #[inline]
    pub(crate) fn wrap_child(
        &self,
        child_level: usize,
        child_anchor1: HexVec,
        child_direction: Angle,
        child_index: usize,
    ) -> (HexVec, Angle) {
        let parent_level = child_level + 1;
        let rel_dir = self.levels[parent_level].entries[0].children[child_index].direction;
        let parent_direction = child_direction - rel_dir;
        let child_offset = self.entry(parent_level, parent_direction).children[child_index].offset;
        (child_anchor1 - child_offset, parent_direction)
    }
}

fn connect_in_level(
    level_table: &LevelTable,
    level: usize,
    curr: ChildPlacement,
    from_anchor: Anchor,
    to_anchor: Anchor,
) -> ChildPlacement {
    let curr_entry = &level_table.entries[curr.direction.value() as usize];
    let from_slot = from_anchor.slot();
    let to_slot = to_anchor.slot();
    let target_edge = if level % 2 == 1 {
        curr_entry.edge_into[from_slot].opposite()
    } else {
        let rotation = curr_entry.edge_from[to_slot] - curr_entry.edge_into[to_slot].opposite();
        curr_entry.edge_from[from_slot] + rotation
    };
    let rel_dir = level_table.entries[0].edge_from[to_slot];
    let next_dir = target_edge - rel_dir;
    let join_point = curr.offset + curr_entry.anchors[from_slot];
    let next_offset = join_point - level_table.entries[next_dir.value() as usize].anchors[to_slot];
    ChildPlacement {
        offset: next_offset,
        direction: next_dir,
    }
}

fn build_table() -> SubstitutionTable {
    let mut levels = Vec::with_capacity(MAX_CLUSTER_LEVEL + 1);

    let level0_entries: [LevelEntry; 12] = std::array::from_fn(|direction| {
        let angle = Angle::new(direction as i32);
        let tile = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, angle);
        let anchors = Anchor::ALL.map(|a| tile.coordinate(a));
        let edge_from = Anchor::ALL.map(|a| tile.edge_direction_from(a));
        let edge_into = Anchor::ALL.map(|a| tile.edge_direction_into(a));
        let spectre_bounds = ExactBounds::from_points(tile.vertices());
        let mystic = tile.into_mystic();
        let mystic_bounds = ExactBounds::from_points(mystic.lower().vertices())
            .union(ExactBounds::from_points(mystic.upper().vertices()));
        LevelEntry {
            anchors,
            edge_from,
            edge_into,
            spectre_bounds,
            mystic_bounds,
            children: [ChildPlacement::ZERO; 8],
        }
    });
    levels.push(LevelTable {
        entries: level0_entries,
    });

    for level in 1..=MAX_CLUSTER_LEVEL {
        let previous = &levels[level - 1];
        let g_anchor3_rel = previous.entries[0].edge_from[Anchor::Anchor3.slot()];
        let entries: [LevelEntry; 12] = std::array::from_fn(|direction| {
            // Anchor1 of level L is child g (index 6)'s Anchor3 at HexVec::ZERO with outgoing edge `direction`.
            let g_dir = Angle::new(direction as i32) - g_anchor3_rel;
            let g_offset = HexVec::ZERO
                - previous.entries[g_dir.value() as usize].anchors[Anchor::Anchor3.slot()];
            let mut children = [ChildPlacement::ZERO; 8];
            children[6] = ChildPlacement {
                offset: g_offset,
                direction: g_dir,
            };
            for step in 0..7 {
                let curr_idx = (6 + step) % 8;
                let (from_anchor, to_anchor) = EDGE_CHAIN[curr_idx];
                children[(curr_idx + 1) % 8] = connect_in_level(
                    previous,
                    level - 1,
                    children[curr_idx],
                    from_anchor,
                    to_anchor,
                );
            }

            let extract = |child_idx: usize, anchor: Anchor| {
                let child = children[child_idx];
                let entry = &previous.entries[child.direction.value() as usize];
                let slot = anchor.slot();
                (
                    child.offset + entry.anchors[slot],
                    entry.edge_from[slot],
                    entry.edge_into[slot],
                )
            };
            let (a1, f1, i1) = extract(6, Anchor::Anchor3);
            let (a2, f2, i2) = extract(3, Anchor::Anchor2);
            let (a3, f3, i3) = extract(1, Anchor::Anchor3);
            let (a4, f4, i4) = extract(0, Anchor::Anchor2);

            let child_bounds = std::array::from_fn::<_, 8, _>(|index| {
                let child = children[index];
                let entry = &previous.entries[child.direction.value() as usize];
                let bounds = if index == 7 {
                    entry.mystic_bounds
                } else {
                    entry.spectre_bounds
                };
                bounds.translated(child.offset)
            });
            let spectre_bounds = child_bounds
                .iter()
                .copied()
                .reduce(ExactBounds::union)
                .unwrap();
            // Mystic substitutions omit child E (index 4); all other children are shared.
            let mystic_bounds = child_bounds
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != 4)
                .map(|(_, bounds)| *bounds)
                .reduce(ExactBounds::union)
                .unwrap();

            LevelEntry {
                anchors: [a1, a2, a3, a4],
                edge_from: [f1, f2, f3, f4],
                edge_into: [i1, i2, i3, i4],
                spectre_bounds,
                mystic_bounds,
                children,
            }
        });
        levels.push(LevelTable { entries });
    }

    SubstitutionTable { levels }
}

#[inline]
pub(crate) fn table() -> &'static SubstitutionTable {
    static TABLE: OnceLock<SubstitutionTable> = OnceLock::new();
    TABLE.get_or_init(build_table)
}
