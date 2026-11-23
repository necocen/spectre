use std::cell::OnceCell;

use lyon_tessellation::{
    FillOptions, FillTessellator, VertexBuffers, geom::Point, geometry_builder::simple_builder,
    path::Path,
};
use mikage::{InstanceVertex, glam::Vec2};

use crate::{
    tiles::{
        Anchor, Skeleton, Spectre, SpectreCluster, SpectreIter,
        bounds::{self, LEAF_LEVEL, SubstitutionTable},
    },
    utils::{Aabb, Angle, HexVec},
};

/// Maximum number of verified covered regions retained to skip redundant clipping checks.
const MAX_VERIFIED_REGIONS: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpectreInstance {
    pub position: [f32; 3],
    pub angle: f32,
}

impl InstanceVertex for SpectreInstance {
    fn vertex_attributes() -> Vec<mikage::wgpu::VertexAttribute> {
        vec![mikage::wgpu::VertexAttribute {
            format: mikage::wgpu::VertexFormat::Float32x4,
            offset: 0,
            shader_location: 2,
        }]
    }
}

/// Spectreタイルのメッシュを生成する
pub fn create_spectre_mesh() -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
    let mut path_builder = Path::builder();
    let points = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO).vertices();
    let points_vec2: Vec<Vec2> = points.iter().map(|p| p.to_vec2()).collect();
    path_builder.begin(Point::new(points_vec2[0].x, points_vec2[0].y));
    for point in points_vec2.iter().skip(1) {
        path_builder.line_to(Point::new(point.x, point.y));
    }
    path_builder.close();
    let path = path_builder.build();

    let mut buffers: VertexBuffers<Point<f32>, u16> = VertexBuffers::new();
    {
        let mut vertex_builder = simple_builder(&mut buffers).with_inverted_winding(); // +Z法線に合わせて反時計回り
        let mut tessellator = FillTessellator::new();
        let result =
            tessellator.tessellate_path(&path, &FillOptions::default(), &mut vertex_builder);
        assert!(result.is_ok());
    }

    let positions: Vec<[f32; 3]> = buffers.vertices.iter().map(|p| [p.x, p.y, 0.0]).collect();
    let normals: Vec<[f32; 3]> = buffers.vertices.iter().map(|_| [0.0, 0.0, 1.0]).collect();
    let indices: Vec<u32> = buffers.indices.iter().map(|&i| i as u32).collect();

    (positions, normals, indices)
}

#[inline]
fn instance_from_anchor1(anchor1: HexVec, rotation: Angle) -> SpectreInstance {
    let pos = anchor1.to_vec2();
    SpectreInstance {
        position: [pos.x, pos.y, 0.0],
        angle: rotation.to_radians(),
    }
}

#[inline]
fn to_instance(spectre: &Spectre) -> SpectreInstance {
    instance_from_anchor1(spectre.anchor1(), spectre.rotation())
}

#[derive(Default)]
struct Coverage {
    whole_tiles: usize,
    boundary_area: f64,
}

impl Coverage {
    #[inline]
    fn total_area(&self) -> f64 {
        self.whole_tiles as f64 * Spectre::area() + self.boundary_area
    }
}

/// Computes the covered area inside `bbox` without emitting instances.
/// Subtrees completely contained in `bbox` are accounted for in O(1) via `tile_count`.
fn measure_coverage_node(
    table: &SubstitutionTable,
    level: usize,
    anchor1: HexVec,
    direction: Angle,
    is_mystic: bool,
    bbox: &Aabb,
    coverage: &mut Coverage,
) {
    if level == LEAF_LEVEL {
        for leaf in table.leaves(direction, is_mystic) {
            let tile = Spectre::from_anchor1(anchor1 + leaf.offset, leaf.rotation);
            let tile_bbox = tile.bbox();
            if !tile_bbox.has_intersection(bbox) {
                continue;
            }
            if bbox.contains_aabb(&tile_bbox) {
                coverage.whole_tiles += 1;
            } else {
                coverage.boundary_area += tile.area_in(bbox);
            }
        }
        return;
    }

    let entry = table.entry(level, direction);
    if level == 0 {
        let mut measure_tile = |tile_anchor1: HexVec, tile_rot: Angle| {
            let tile = Spectre::from_anchor1(tile_anchor1, tile_rot);
            let tile_bbox = tile.bbox();
            if !tile_bbox.has_intersection(bbox) {
                return;
            }
            if bbox.contains_aabb(&tile_bbox) {
                coverage.whole_tiles += 1;
            } else {
                coverage.boundary_area += tile.area_in(bbox);
            }
        };
        measure_tile(anchor1, direction);
        if is_mystic {
            measure_tile(
                anchor1 + entry.mystic_upper.offset,
                entry.mystic_upper.direction,
            );
        }
        return;
    }

    let child_level = level - 1;
    for (index, child) in entry.children.iter().enumerate() {
        if is_mystic && index == 4 {
            continue;
        }
        let child_anchor1 = anchor1 + child.offset;
        let child_is_mystic = index == 7;
        let child_bbox =
            table.bounds_aabb(child_level, child.direction, child_anchor1, child_is_mystic);
        if !child_bbox.has_intersection(bbox) {
            continue;
        }
        if bbox.contains_aabb(&child_bbox) {
            coverage.whole_tiles += table.tile_count(child_level, child_is_mystic);
            continue;
        }
        measure_coverage_node(
            table,
            child_level,
            child_anchor1,
            child.direction,
            child_is_mystic,
            bbox,
            coverage,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn collect_instances_node(
    table: &SubstitutionTable,
    level: usize,
    anchor1: HexVec,
    direction: Angle,
    is_mystic: bool,
    fully_inside: bool,
    bbox: &Aabb,
    instances: &mut Vec<SpectreInstance>,
) {
    if level == LEAF_LEVEL {
        let leaves = table.leaves(direction, is_mystic);
        if fully_inside {
            instances.extend(
                leaves
                    .iter()
                    .map(|leaf| instance_from_anchor1(anchor1 + leaf.offset, leaf.rotation)),
            );
        } else {
            for leaf in leaves {
                let tile = Spectre::from_anchor1(anchor1 + leaf.offset, leaf.rotation);
                if tile.bbox().has_intersection(bbox) {
                    instances.push(to_instance(&tile));
                }
            }
        }
        return;
    }

    let entry = table.entry(level, direction);
    if level == 0 {
        let mut emit_tile = |tile_anchor1: HexVec, tile_rot: Angle| {
            if fully_inside {
                instances.push(instance_from_anchor1(tile_anchor1, tile_rot));
                return;
            }
            let tile = Spectre::from_anchor1(tile_anchor1, tile_rot);
            if tile.bbox().has_intersection(bbox) {
                instances.push(to_instance(&tile));
            }
        };
        emit_tile(anchor1, direction);
        if is_mystic {
            emit_tile(
                anchor1 + entry.mystic_upper.offset,
                entry.mystic_upper.direction,
            );
        }
        return;
    }

    let child_level = level - 1;
    for (index, child) in entry.children.iter().enumerate() {
        if is_mystic && index == 4 {
            continue;
        }
        let child_anchor1 = anchor1 + child.offset;
        let child_is_mystic = index == 7;
        let child_inside = if fully_inside {
            true
        } else {
            let child_bbox =
                table.bounds_aabb(child_level, child.direction, child_anchor1, child_is_mystic);
            if !child_bbox.has_intersection(bbox) {
                continue;
            }
            bbox.contains_aabb(&child_bbox)
        };
        collect_instances_node(
            table,
            child_level,
            child_anchor1,
            child.direction,
            child_is_mystic,
            child_inside,
            bbox,
            instances,
        );
    }
}

/// Maintains a generated region around the viewport and reusable instance data.
/// GPU uploads are needed only when `update_view` returns a new slice.
pub struct TilesController {
    root: Skeleton,
    instances: Vec<SpectreInstance>,
    generated_bbox: Option<Aabb>,
    verified_bboxes: Vec<Aabb>,
    materialized: OnceCell<SpectreCluster>,
    materialized_bbox: Aabb,
}

impl Default for TilesController {
    fn default() -> Self {
        Self::new()
    }
}

impl TilesController {
    pub fn new() -> Self {
        let root = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5, None);
        Self {
            root,
            instances: Vec::new(),
            generated_bbox: None,
            verified_bboxes: Vec::new(),
            materialized: OnceCell::new(),
            materialized_bbox: Aabb::NULL,
        }
    }

    /// Extend the root without moving existing tiles. Returns false at the limit.
    pub fn expand(&mut self) -> bool {
        if self.level() >= crate::tiles::MAX_CLUSTER_LEVEL {
            return false;
        }
        let child_index = if self.root.level().is_multiple_of(2) {
            0
        } else {
            5
        };
        self.root = self.root.wrap_as_child(child_index);
        self.generated_bbox = None;
        self.materialized.take();
        true
    }

    pub fn level(&self) -> usize {
        self.root.level()
    }

    pub fn update(&mut self, bbox: &Aabb) {
        self.materialized_bbox = *bbox;
        if let Some(cluster) = self.materialized.get_mut() {
            cluster.update(bbox);
        }
        self.generated_bbox = None;
    }

    pub fn spectres_in(&self, bbox: &Aabb) -> SpectreIter<'_> {
        let cluster = self
            .materialized
            .get_or_init(|| self.root.to_spectre_cluster(&self.materialized_bbox));
        cluster.spectres_in(*bbox)
    }

    pub fn cluster_bbox(&self) -> Aabb {
        self.root.estimated_bbox()
    }

    pub fn instances(&self) -> &[SpectreInstance] {
        &self.instances
    }

    fn record_verified_bbox(&mut self, bbox: Aabb) {
        self.verified_bboxes
            .retain(|existing| !bbox.contains_aabb(existing));
        if self.verified_bboxes.len() >= MAX_VERIFIED_REGIONS {
            self.verified_bboxes.remove(0);
        }
        self.verified_bboxes.push(bbox);
    }

    /// Update for the actual viewport, without a caller-supplied margin.
    /// Expands to complete the generated region before returning its final data.
    /// An empty viewport or a view inside the cached region needs no upload.
    pub fn update_view(&mut self, viewport: &Aabb) -> Option<&[SpectreInstance]> {
        if viewport.is_empty() {
            return None;
        }
        let center = (viewport.min + viewport.max) * 0.5;
        let half_size = ((viewport.max - viewport.min) * 0.75).max(Vec2::splat(15.0));
        let bbox = Aabb::from_min_max(center - half_size, center + half_size);
        // Also shrink the cache after a substantial zoom-in, so a tiny view
        // does not keep submitting all instances from a previous zoom-out.
        if self.generated_bbox.is_some_and(|generated| {
            generated.contains_aabb(viewport) && generated.area() <= bbox.area() * 2.0
        }) {
            return None;
        }

        let already_verified = self
            .verified_bboxes
            .iter()
            .any(|verified| verified.contains_aabb(&bbox));
        let table = bounds::table();

        // Phase 1: Expand root until `bbox` is completely covered, using an O(1)-subtree
        // coverage pass that avoids building instances for intermediate expansion levels.
        if !already_verified {
            let requested_area = bbox.area();
            let tolerance = requested_area * 1e-8 + 1e-7;
            loop {
                while !self.cluster_bbox().contains_aabb(&bbox) && self.expand() {}
                if self.level() >= crate::tiles::MAX_CLUSTER_LEVEL {
                    break;
                }
                let root_bbox = self.cluster_bbox();
                let mut coverage = Coverage::default();
                if root_bbox.has_intersection(&bbox) {
                    if bbox.contains_aabb(&root_bbox) {
                        coverage.whole_tiles = table.tile_count(self.root.level(), false);
                    } else {
                        measure_coverage_node(
                            table,
                            self.root.level(),
                            self.root.anchor1(),
                            self.root.direction(),
                            false,
                            &bbox,
                            &mut coverage,
                        );
                    }
                }
                if coverage.total_area() + tolerance >= requested_area {
                    self.record_verified_bbox(bbox);
                    break;
                }
                if !self.expand() {
                    break;
                }
            }
        }

        // Phase 2: Emit GPU instances once at the final root level.
        self.instances.clear();
        let root_bbox = self.cluster_bbox();
        if root_bbox.has_intersection(&bbox) {
            let root_inside = bbox.contains_aabb(&root_bbox);
            collect_instances_node(
                table,
                self.root.level(),
                self.root.anchor1(),
                self.root.direction(),
                false,
                root_inside,
                &bbox,
                &mut self.instances,
            );
        }

        self.generated_bbox = Some(bbox);
        self.materialized_bbox = bbox;
        self.materialized.take();
        Some(&self.instances)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mikage::glam::DVec2;

    fn viewport(zoom: f32, position: Vec2) -> Aabb {
        let half_size = Vec2::new(16.0 / 9.0 / zoom, 1.0 / zoom);
        Aabb::from_min_max(position - half_size, position + half_size)
    }

    #[test]
    fn empty_view_inside_cluster_bounds_is_filled_before_returning() {
        let root = SpectreCluster::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5);
        let bbox = Aabb::new(-5.045517, -528.0256, 24.954483, -498.02557);
        assert!(root.bbox().contains_aabb(&bbox));
        assert_eq!(root.spectres_in(bbox).count(), 0);
        let mut controller = TilesController {
            root: root.to_skeleton(),
            instances: Vec::new(),
            generated_bbox: None,
            verified_bboxes: Vec::new(),
            materialized: OnceCell::new(),
            materialized_bbox: Aabb::NULL,
        };
        assert!(!controller.update_view(&bbox).unwrap().is_empty());
        assert!(controller.level() > 5);
        assert!(controller.update_view(&bbox).is_none());
    }

    #[test]
    fn expansion_stops_at_the_declared_maximum_level() {
        let mut controller = TilesController::new();
        while controller.level() < crate::tiles::MAX_CLUSTER_LEVEL {
            assert!(controller.expand());
        }
        assert!(!controller.expand());
        assert_eq!(controller.level(), crate::tiles::MAX_CLUSTER_LEVEL);
    }

    #[test]
    fn blocked_expansion_does_not_retry_the_same_view() {
        let mut controller = TilesController::new();
        while controller.expand() {}
        let outside = controller.cluster_bbox().max + Vec2::splat(10_000_000.0);
        let bbox = Aabb::from_min_max(outside, outside + Vec2::splat(128.0));
        assert!(controller.update_view(&bbox).unwrap().is_empty());
        for _ in 0..3 {
            assert!(controller.update_view(&bbox).is_none());
        }
        assert_eq!(controller.level(), crate::tiles::MAX_CLUSTER_LEVEL);
    }

    #[test]
    fn small_pans_reuse_instances_and_cover_every_visible_tile() {
        let mut controller = TilesController::new();
        let initial = controller
            .update_view(&viewport(0.028, Vec2::ZERO))
            .unwrap()
            .to_vec();
        assert!(!initial.is_empty());
        let key = |instance: SpectreInstance| {
            [
                instance.position[0].to_bits(),
                instance.position[1].to_bits(),
                instance.position[2].to_bits(),
                instance.angle.to_bits(),
            ]
        };
        let initial_keys: std::collections::HashSet<_> = initial.iter().copied().map(key).collect();
        for frame in 1..=120 {
            let view = viewport(0.028, Vec2::new(frame as f32 * 0.05, 0.0));
            assert!(controller.update_view(&view).is_none());
            assert_eq!(controller.instances(), initial);
            for tile in controller.spectres_in(&view) {
                assert!(
                    initial_keys.contains(&key(to_instance(tile))),
                    "cached instances missed a visible tile"
                );
            }
        }
    }

    #[test]
    fn pan_zoom_and_resize_regenerate_only_when_the_cached_region_is_left() {
        let mut controller = TilesController::new();
        let view = viewport(0.028, Vec2::ZERO);
        assert!(controller.update_view(&view).is_some());
        assert!(controller.update_view(&view).is_none());
        assert!(
            controller
                .update_view(&viewport(0.035, Vec2::ZERO))
                .is_none()
        );
        assert!(
            controller
                .update_view(&viewport(0.014, Vec2::ZERO))
                .is_some()
        );
        let shifted = viewport(0.014, Vec2::new(300.0, 0.0));
        assert!(controller.update_view(&shifted).is_some());
        let wider = Aabb::from_min_max(
            shifted.min - Vec2::new(300.0, 0.0),
            shifted.max + Vec2::new(300.0, 0.0),
        );
        assert!(controller.update_view(&wider).is_some());
        assert!(controller.update_view(&wider).is_none());
    }

    #[test]
    fn explicit_expansion_invalidates_the_cached_region() {
        let mut controller = TilesController::new();
        let view = viewport(0.028, Vec2::ZERO);
        controller.update_view(&view);
        assert!(controller.expand());
        assert!(controller.update_view(&view).is_some());
    }

    #[test]
    fn invalid_view_does_not_discard_generated_instances() {
        let mut controller = TilesController::new();
        controller.update_view(&viewport(0.028, Vec2::ZERO));
        let initial = controller.instances().to_vec();
        assert!(controller.update_view(&Aabb::NULL).is_none());
        assert_eq!(controller.instances(), initial);
    }

    #[test]
    fn instance_buffer_capacity_is_reused_when_regenerating() {
        let mut controller = TilesController::new();
        let view = Aabb::new(-10.0, -10.0, 10.0, 10.0);
        controller.update_view(&view);
        let capacity = controller.instances.capacity();
        let pointer = controller.instances.as_ptr();
        controller.update_view(&Aabb::new(90.0, -10.0, 110.0, 10.0));
        assert_eq!(controller.instances.capacity(), capacity);
        assert_eq!(controller.instances.as_ptr(), pointer);
    }

    #[test]
    fn minimum_zoom_is_completely_filled_in_one_update() {
        let mut controller = TilesController::new();
        let view = viewport(0.003, Vec2::ZERO);
        assert!(!controller.update_view(&view).unwrap().is_empty());
        let bbox = controller.generated_bbox.unwrap();
        let covered: f64 = controller
            .spectres_in(&bbox)
            .map(|tile| tile.area_in(&bbox))
            .sum();
        assert!((covered - bbox.area()).abs() < bbox.area() * 1e-8);
        assert!(controller.update_view(&view).is_none());
    }

    #[test]
    fn substantial_zoom_in_reduces_the_number_of_submitted_instances() {
        let mut controller = TilesController::new();
        let wide_count = controller
            .update_view(&viewport(0.003, Vec2::ZERO))
            .unwrap()
            .len();
        let close_count = controller
            .update_view(&viewport(0.12, Vec2::ZERO))
            .unwrap()
            .len();
        assert!(close_count < wide_count / 100);
        assert!(
            controller
                .update_view(&viewport(0.12, Vec2::ZERO))
                .is_none()
        );
    }

    #[test]
    fn tessellated_mesh_has_correct_area_winding_and_indices() {
        let (positions, normals, indices) = create_spectre_mesh();
        assert_eq!(positions.len(), normals.len());
        let (triangles, remainder) = indices.as_chunks::<3>();
        assert!(remainder.is_empty());
        let mut area = 0.0;
        for triangle in triangles {
            let vertices = triangle.map(|index| {
                let p = positions[index as usize];
                DVec2::new(f64::from(p[0]), f64::from(p[1]))
            });
            let signed_area = (vertices[1] - vertices[0]).perp_dot(vertices[2] - vertices[0]) * 0.5;
            assert!(
                signed_area >= 0.0,
                "area={signed_area}, vertices={vertices:?}"
            );
            area += signed_area;
        }
        assert!((area - Spectre::area()).abs() < 1e-5);
        assert!(normals.iter().all(|normal| *normal == [0.0, 0.0, 1.0]));
    }
}
