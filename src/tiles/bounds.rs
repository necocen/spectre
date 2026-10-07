use std::sync::OnceLock;

use glam::DVec2;

use super::{Anchor, MAX_CLUSTER_LEVEL, Skeleton, Spectre};
use crate::utils::{Aabb, Angle, HexVec};

/// Extrema stay in the exact coordinate system until translated to world space.
/// `magnitude` bounds the sum of the absolute terms in each coordinate, including
/// cancellation, so the resulting AABB also contains f32-rounded tile vertices.
#[derive(Clone, Copy)]
pub(super) struct ExactBounds {
    min: HexVec,
    max: HexVec,
    magnitude: DVec2,
}

impl ExactBounds {
    fn term_magnitude(p: HexVec) -> DVec2 {
        let coordinate = |value: crate::utils::HexValue| {
            (f64::from(value.rational).abs() + f64::from(value.irrational).abs() * 3.0_f64.sqrt())
                * 0.5
        };
        DVec2::new(coordinate(p.x), coordinate(p.y))
    }

    pub(super) fn from_points(points: impl IntoIterator<Item = HexVec>) -> Self {
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

    pub(super) fn translated(self, offset: HexVec) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
            magnitude: self.magnitude + Self::term_magnitude(offset),
        }
    }

    pub(super) fn to_aabb(self) -> Aabb {
        let error = self.magnitude * (4.0 * f64::from(f32::EPSILON));
        Aabb::new(
            ((self.min.x.to_f64() - error.x) as f32).next_down(),
            ((self.min.y.to_f64() - error.y) as f32).next_down(),
            ((self.max.x.to_f64() + error.x) as f32).next_up(),
            ((self.max.y.to_f64() + error.y) as f32).next_up(),
        )
    }
}

struct LevelBounds {
    spectre: [ExactBounds; 12],
    mystic: [ExactBounds; 12],
}

fn build_bounds() -> Vec<LevelBounds> {
    let tiles: [Spectre; 12] = std::array::from_fn(|direction| {
        Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::new(direction as i32))
    });
    let mut levels = Vec::with_capacity(MAX_CLUSTER_LEVEL + 1);
    levels.push(LevelBounds {
        spectre: tiles.map(|tile| ExactBounds::from_points(tile.vertices())),
        mystic: tiles.map(|tile| {
            let pair = tile.into_mystic();
            ExactBounds::from_points(pair.lower().vertices())
                .union(ExactBounds::from_points(pair.upper().vertices()))
        }),
    });
    for level in 1..=MAX_CLUSTER_LEVEL {
        let previous = &levels[level - 1];
        let bounds: [(ExactBounds, ExactBounds); 12] = std::array::from_fn(|direction| {
            let skeleton = Skeleton::with_anchor(
                Anchor::Anchor1,
                HexVec::ZERO,
                Angle::new(direction as i32),
                level,
                None,
            );
            let children = skeleton.split_into_skeletons().map(|child| {
                let direction = child.edge_direction_from(Anchor::Anchor1).value() as usize;
                (direction, child.coordinate(Anchor::Anchor1))
            });
            let child_bounds = std::array::from_fn::<_, 8, _>(|index| {
                let (direction, offset) = children[index];
                let bounds = if index == 7 {
                    previous.mystic[direction]
                } else {
                    previous.spectre[direction]
                };
                bounds.translated(offset)
            });
            let spectre = child_bounds
                .iter()
                .copied()
                .reduce(ExactBounds::union)
                .unwrap();
            // Mystic substitutions omit child E; all other children are shared.
            let mystic = child_bounds
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != 4)
                .map(|(_, bounds)| *bounds)
                .reduce(ExactBounds::union)
                .unwrap();
            (spectre, mystic)
        });
        levels.push(LevelBounds {
            spectre: bounds.map(|(spectre, _)| spectre),
            mystic: bounds.map(|(_, mystic)| mystic),
        });
    }
    levels
}

pub(super) fn spectre_bounds(skeleton: &Skeleton) -> Aabb {
    static BOUNDS: OnceLock<Vec<LevelBounds>> = OnceLock::new();
    let levels = BOUNDS.get_or_init(build_bounds);
    let direction = skeleton.edge_direction_from(Anchor::Anchor1).value() as usize;
    levels[skeleton.level()].spectre[direction]
        .translated(skeleton.coordinate(Anchor::Anchor1))
        .to_aabb()
}
