use std::time::Duration;

use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use glam::Vec2;
use spectre::{
    TilesController,
    tiles::{Anchor, SpectreCluster},
    utils::{Aabb, Angle, HexVec},
};

fn viewport(zoom: f32, x: f32) -> Aabb {
    let center = Vec2::new(x, 0.0);
    let half_size = Vec2::new(16.0 / 9.0 / zoom, 1.0 / zoom);
    Aabb::from_min_max(center - half_size, center + half_size)
}

fn bench_controller(c: &mut Criterion) {
    let mut group = c.benchmark_group("controller");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));
    let wide = viewport(0.003, 0.0);
    group.bench_function("initial_minimum_zoom", |b| {
        b.iter_batched(
            TilesController::new,
            |mut controller| black_box(controller.update_view(black_box(&wide)).unwrap().len()),
            BatchSize::LargeInput,
        );
    });

    let pans: Vec<_> = (1..=120)
        .map(|frame| viewport(0.003, frame as f32 * 0.05))
        .collect();
    let mut controller = TilesController::new();
    controller.update_view(&wide);
    group.bench_function("cached_120_pans", |b| {
        b.iter(|| {
            let uploads = pans
                .iter()
                .filter(|view| controller.update_view(black_box(view)).is_some())
                .count();
            black_box(uploads)
        });
    });

    let distant = viewport(0.003, 500.0);
    group.bench_function("pan_beyond_margin_and_back", |b| {
        b.iter(|| {
            black_box(controller.update_view(black_box(&distant)).unwrap().len());
            black_box(controller.update_view(black_box(&wide)).unwrap().len())
        });
    });

    let close = viewport(0.12, 0.0);
    group.bench_function("zoom_in_and_out", |b| {
        b.iter(|| {
            black_box(controller.update_view(black_box(&close)).unwrap().len());
            black_box(controller.update_view(black_box(&wide)).unwrap().len())
        });
    });
    group.finish();
}

fn create_spectre_cluster(level: usize) -> SpectreCluster {
    SpectreCluster::with_anchor(Anchor::Anchor1, HexVec::ZERO, Angle::ZERO, level)
}

fn create_test_bboxes() -> Vec<(String, Aabb)> {
    vec![
        // Small AABB at center
        (
            "small_center".to_string(),
            Aabb::new(-10.0, -10.0, 10.0, 10.0),
        ),
        // Medium AABB at center
        (
            "medium_center".to_string(),
            Aabb::new(-100.0, -100.0, 100.0, 100.0),
        ),
        // Medium AABB at bottom right
        (
            "medium_bottom_right".to_string(),
            Aabb::new(100.0, 100.0, 200.0, 200.0),
        ),
        // Large AABB covering most of the space
        (
            "large_covering".to_string(),
            Aabb::new(-1000.0, -1000.0, 1000.0, 1000.0),
        ),
        // AABB outside the SuperSpectre
        (
            "outside".to_string(),
            Aabb::new(-1000.0, -1000.0, -900.0, -900.0),
        ),
        // AABB partially intersecting
        (
            "partial_intersect".to_string(),
            Aabb::new(-5.0, -5.0, 0.0, 0.0),
        ),
        // AABB exactly matching a Spectre
        ("exact_match".to_string(), Aabb::new(-2.0, -2.0, 2.0, 2.0)),
    ]
}

fn bench_spectres_in(c: &mut Criterion) {
    let mut group = c.benchmark_group("spectres_in");
    group.sample_size(50); // Increase sample size for more accurate results

    // Test different levels
    for level in [3, 4, 5, 6].iter() {
        let spectre_cluster = create_spectre_cluster(*level);
        let bboxes = create_test_bboxes();

        for (bbox_name, bbox) in bboxes {
            group.bench_with_input(
                BenchmarkId::new(format!("level_{}", level), bbox_name),
                &(level, bbox),
                |b, (_level, bbox)| {
                    b.iter(|| {
                        let spectres: Vec<_> =
                            black_box(spectre_cluster.spectres_in(*bbox)).collect();
                        black_box(spectres)
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_spectres_in_with_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("spectres_in_size");
    group.sample_size(100);

    // Test different AABB sizes at level 5
    let spectre_cluster = create_spectre_cluster(5);
    let sizes = [10.0, 50.0, 100.0, 500.0, 1000.0];

    for size in sizes.iter() {
        let bbox = Aabb::new(-size, -size, *size, *size);
        group.bench_with_input(BenchmarkId::new("size", size), size, |b, _| {
            b.iter(|| {
                let spectres: Vec<_> = black_box(spectre_cluster.spectres_in(bbox)).collect();
                black_box(spectres)
            })
        });
    }

    group.finish();
}

fn bench_spectres_in_position(c: &mut Criterion) {
    let mut group = c.benchmark_group("spectres_in_position");
    group.sample_size(100);

    // Test different AABB positions at level 5
    let spectre_cluster = create_spectre_cluster(5);
    let positions = [
        ("center", (0.0, 0.0)),
        ("top", (0.0, 5.0)),
        ("bottom", (0.0, -5.0)),
        ("right", (5.0, 0.0)),
        ("left", (-5.0, 0.0)),
        ("far", (50.0, 50.0)),
    ];

    for (name, (x, y)) in positions.iter() {
        let bbox = Aabb::new(x - 100.0, y - 100.0, x + 100.0, y + 100.0);
        group.bench_with_input(BenchmarkId::new("position", name), name, |b, _| {
            b.iter(|| {
                let spectres: Vec<_> = black_box(spectre_cluster.spectres_in(bbox)).collect();
                black_box(spectres)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_controller,
    bench_spectres_in,
    bench_spectres_in_with_size,
    bench_spectres_in_position
);
criterion_main!(benches);
