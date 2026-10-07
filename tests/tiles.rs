use spectre::{
    tiles::{Anchor, Skeleton, Spectre, SpectreCluster},
    utils::{Aabb, Angle, HexValue, HexVec},
};

const ANCHORS: [Anchor; 4] = [
    Anchor::Anchor1,
    Anchor::Anchor2,
    Anchor::Anchor3,
    Anchor::Anchor4,
];

type TileKey = (i32, i32, i32, i32, u8);

fn tile_key(tile: &Spectre) -> TileKey {
    let p = tile.coordinate(Anchor::Anchor1);
    (
        p.x.rational,
        p.x.irrational,
        p.y.rational,
        p.y.irrational,
        tile.rotation().value(),
    )
}

fn query_keys(cluster: &SpectreCluster, bbox: Aabb) -> Vec<TileKey> {
    let mut keys: Vec<_> = cluster.spectres_in(bbox).map(tile_key).collect();
    keys.sort_unstable();
    keys
}

// Enumerate the entire eager tree first, then filter individual tiles. This
// reference does not use the query-dependent pruning exercised by the tests.
fn reference_tiles(cluster: &SpectreCluster) -> Vec<Spectre> {
    cluster.spectres_in(cluster.bbox()).copied().collect()
}

fn reference_keys(tiles: &[Spectre], bbox: Aabb) -> Vec<TileKey> {
    let mut keys: Vec<_> = tiles
        .iter()
        .filter(|tile| tile.bbox().has_intersection(&bbox))
        .map(tile_key)
        .collect();
    keys.sort_unstable();
    keys
}

fn cluster(level: usize, direction: i32) -> SpectreCluster {
    SpectreCluster::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::new(direction), level)
}

#[test]
fn skeleton_bounds_contain_all_tiles_for_every_anchor_and_direction() {
    for level in 1..=4 {
        for direction in 0..12 {
            for anchor in ANCHORS {
                let skeleton =
                    Skeleton::with_anchor(anchor, HexVec::ZERO, Angle::new(direction), level, None);
                let eager =
                    SpectreCluster::with_anchor(anchor, HexVec::ZERO, Angle::new(direction), level);
                let bounds = skeleton.estimated_bbox();
                let actual = eager.bbox();
                assert!(
                    bounds.contains(actual.min) && bounds.contains(actual.max),
                    "level={level}, direction={direction}, anchor={anchor:?}: {bounds:?} does not contain {actual:?}"
                );
            }
        }
    }
}

#[test]
fn partial_generation_keeps_tiles_at_the_underestimated_boundary() {
    let eager = cluster(5, 0);
    let reference = reference_tiles(&eager);
    let bbox = Aabb::new(129.399, 25.854553, 159.399, 55.854553);
    let expected = reference_keys(&reference, bbox);
    assert_eq!(expected.len(), 138);

    let skeleton = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5, None);
    let mut partial = skeleton.to_spectre_cluster(&Aabb::NULL);
    partial.update(&bbox);
    assert_eq!(query_keys(&partial, bbox), expected);
    partial.update(&bbox);
    assert_eq!(query_keys(&partial, bbox), expected);
}

#[test]
fn pruning_and_reloading_preserves_exact_tile_positions_and_directions() {
    let bbox = Aabb::new(129.399, 25.854553, 159.399, 55.854553);
    let expected = reference_keys(&reference_tiles(&cluster(5, 0)), bbox);
    let skeleton = Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 5, None);
    let mut partial = skeleton.to_spectre_cluster(&bbox);
    for query in [bbox, Aabb::new(-2000.0, -2000.0, -1900.0, -1900.0), bbox] {
        partial.update(&query);
    }
    assert_eq!(query_keys(&partial, bbox), expected);
}

#[test]
fn moving_queries_match_eager_generation_for_all_directions() {
    for direction in 0..12 {
        let eager = cluster(4, direction);
        let reference = reference_tiles(&eager);
        let bounds = eager.bbox();
        let skeleton = Skeleton::with_anchor(
            Anchor::Anchor1,
            HexVec::ZERO,
            Angle::new(direction),
            4,
            None,
        );
        let mut partial = skeleton.to_spectre_cluster(&Aabb::NULL);
        for ix in 0..6 {
            for iy in 0..6 {
                let x = bounds.min.x + (bounds.max.x - bounds.min.x) * ix as f32 / 5.0;
                let y = bounds.min.y + (bounds.max.y - bounds.min.y) * iy as f32 / 5.0;
                let query = Aabb::new(x - 15.0, y - 15.0, x + 15.0, y + 15.0);
                partial.update(&query);
                assert_eq!(
                    query_keys(&partial, query),
                    reference_keys(&reference, query),
                    "direction={direction}, query={query:?}"
                );
            }
        }
    }
}

#[test]
fn tile_counts_and_uniqueness_follow_substitution_rules() {
    for (level, expected_count) in [(1, 9), (2, 71), (3, 559), (4, 4401), (5, 34649)] {
        let eager = cluster(level, 0);
        let keys = query_keys(&eager, eager.bbox());
        assert_eq!(keys.len(), expected_count, "level={level}");
        assert!(
            keys.windows(2).all(|pair| pair[0] != pair[1]),
            "level={level}"
        );
        assert!(query_keys(&eager, Aabb::NULL).is_empty());
    }
}

#[test]
fn skeleton_geometry_matches_eager_clusters_after_translation() {
    let translation = HexVec::new(HexValue::new(37, -11), HexValue::new(-23, 7));
    for level in 1..=3 {
        for direction in 0..12 {
            for anchor in ANCHORS {
                let skeleton =
                    Skeleton::with_anchor(anchor, translation, Angle::new(direction), level, None);
                let eager =
                    SpectreCluster::with_anchor(anchor, translation, Angle::new(direction), level);
                for test_anchor in ANCHORS {
                    assert_eq!(
                        skeleton.coordinate(test_anchor),
                        eager.coordinate(test_anchor)
                    );
                    assert_eq!(
                        skeleton.edge_direction_from(test_anchor),
                        eager.edge_direction_from(test_anchor)
                    );
                    assert_eq!(
                        skeleton.edge_direction_into(test_anchor),
                        eager.edge_direction_into(test_anchor)
                    );
                }
            }
        }
    }
}

#[test]
fn canonical_vertices_and_all_rotations_keep_the_tile_shape() {
    let expected = [
        (0, 0, 0, 0),
        (2, 0, 0, 0),
        (4, 0, 0, 0),
        (5, 0, 0, 1),
        (5, 1, -1, 1),
        (5, 2, 0, 1),
        (4, 2, 0, 2),
        (2, 2, 0, 2),
        (2, 2, 2, 2),
        (2, 1, 3, 2),
        (1, 1, 3, 1),
        (-1, 1, 3, 1),
        (-1, 1, 1, 1),
        (-1, 0, 0, 1),
    ]
    .map(|(xr, xi, yr, yi)| HexVec::new(HexValue::new(xr, xi), HexValue::new(yr, yi)));
    for direction in 0..12 {
        let angle = Angle::new(direction);
        let tile = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, angle);
        assert_eq!(
            tile.vertices(),
            expected.map(|p| p.rotate(HexVec::ZERO, angle))
        );
        for vertex in tile.vertices() {
            assert!(tile.bbox().contains(vertex.to_vec2()));
        }
    }
}

#[test]
fn clipped_area_handles_contained_disjoint_and_partial_rectangles() {
    let tile = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO);
    assert_eq!(tile.area_in(&tile.bbox()), Spectre::area());
    assert_eq!(tile.area_in(&Aabb::NULL), 0.0);
    assert_eq!(tile.area_in(&Aabb::new(-10.0, -10.0, -9.0, -9.0)), 0.0);
    let inside = Aabb::new(0.1, 0.1, 0.2, 0.2);
    assert!((tile.area_in(&inside) - inside.area()).abs() < 1e-12);
    let strip = Aabb::new(0.0, 0.0, 1.0, 0.125);
    assert!((tile.area_in(&strip) - 0.125).abs() < 1e-12);
}

#[test]
fn clipped_area_is_conserved_by_rectangular_partitions_for_all_directions() {
    for direction in 0..12 {
        let tile = Spectre::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::new(direction));
        let bounds = tile.bbox();
        let mut total = 0.0;
        for x in 0..8 {
            for y in 0..8 {
                let at = |ix, iy| {
                    bounds.min
                        + (bounds.max - bounds.min)
                            * glam::Vec2::new(ix as f32 / 8.0, iy as f32 / 8.0)
                };
                let query = Aabb::from_min_max(at(x, y), at(x + 1, y + 1));
                let area = tile.area_in(&query);
                assert!(area >= 0.0 && area <= query.area() + 1e-9);
                total += area;
            }
        }
        assert!(
            (total - Spectre::area()).abs() < 1e-9,
            "direction={direction}, area={total}"
        );
    }
}

#[test]
fn translated_bounds_contain_f32_vertices_when_coordinates_are_large() {
    for direction in 0..12 {
        let offset = HexVec::new(
            HexValue::new(2_000_000, -999_983),
            HexValue::new(-2_000_001, 999_979),
        );
        let skeleton =
            Skeleton::with_anchor(Anchor::Anchor1, offset, Angle::new(direction), 3, None);
        let eager = SpectreCluster::with_anchor(Anchor::Anchor1, offset, Angle::new(direction), 3);
        let bounds = skeleton.estimated_bbox();
        assert!(bounds.contains_aabb(&eager.bbox()));
        for tile in eager.spectres_in(eager.bbox()) {
            for vertex in tile.vertices() {
                assert!(bounds.contains(vertex.to_vec2()));
            }
        }
    }
}

#[test]
fn wrapping_a_cluster_preserves_existing_tile_positions() {
    let root = cluster(4, 0);
    let bounds = root.bbox();
    let original = query_keys(&root, bounds);
    let expanded = SpectreCluster::with_child_a(root);
    assert_eq!(query_keys(&expanded, bounds), original);
    let expanded = SpectreCluster::with_child_f(expanded);
    assert_eq!(query_keys(&expanded, bounds), original);
}

#[test]
#[should_panic(expected = "unsupported cluster level")]
fn skeleton_rejects_zero_level_instead_of_underflowing() {
    Skeleton::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, 0, None);
}
