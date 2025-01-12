use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use spectre::{
    geometry::{Aabb, Anchor, SuperSpectre},
    utils::{Angle, HexVec},
};

fn create_super_spectre(level: usize) -> SuperSpectre {
    SuperSpectre::new_with_anchor(level, HexVec::ZERO, Anchor::Anchor1, Angle::ZERO)
}

fn create_test_aabbs() -> Vec<(String, Aabb)> {
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
        let super_spectre = create_super_spectre(*level);
        let aabbs = create_test_aabbs();

        for (aabb_name, aabb) in aabbs {
            group.bench_with_input(
                BenchmarkId::new(format!("level_{}", level), aabb_name),
                &(level, aabb),
                |b, (_level, aabb)| {
                    b.iter(|| {
                        let spectres: Vec<_> =
                            black_box(super_spectre.spectres_in(*aabb)).collect();
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
    let super_spectre = create_super_spectre(5);
    let sizes = [10.0, 50.0, 100.0, 500.0, 1000.0];

    for size in sizes.iter() {
        let aabb = Aabb::new(-size, -size, *size, *size);
        group.bench_with_input(BenchmarkId::new("size", size), size, |b, _| {
            b.iter(|| {
                let spectres: Vec<_> = black_box(super_spectre.spectres_in(aabb)).collect();
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
    let super_spectre = create_super_spectre(5);
    let positions = [
        ("center", (0.0, 0.0)),
        ("top", (0.0, 5.0)),
        ("bottom", (0.0, -5.0)),
        ("right", (5.0, 0.0)),
        ("left", (-5.0, 0.0)),
        ("far", (50.0, 50.0)),
    ];

    for (name, (x, y)) in positions.iter() {
        let aabb = Aabb::new(x - 100.0, y - 100.0, x + 100.0, y + 100.0);
        group.bench_with_input(BenchmarkId::new("position", name), name, |b, _| {
            b.iter(|| {
                let spectres: Vec<_> = black_box(super_spectre.spectres_in(aabb)).collect();
                black_box(spectres)
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_spectres_in,
    bench_spectres_in_with_size,
    bench_spectres_in_position
);
criterion_main!(benches);
