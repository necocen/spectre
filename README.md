# Infinite Spectres

![Demo](./img/demo.gif "An animation demonstrating scrolling and zooming over a plane tiled with the Spectre monotile.")

This is a program that infinitely tiles using the Spectre (more precisely, Tile(1,1)). It is written in Rust with [mikage](https://github.com/necocen/mikage) (a lightweight wgpu+winit framework) and also runs in a web browser.
Spectre is an aperiodic monotile discovered in Reference [1]. For more details, please refer to the paper or the authors' website ( https://cs.uwaterloo.ca/~csk/spectre/ ).

Live demo here: https://spectre.necocen.info/

## How to build
### What You'll Need

- Rust 1.95 or newer
- [Trunk](https://trunkrs.dev/) for web builds

### Build Commands

Running it locally:
```bash
cargo run --release
```

Building for the web:
```bash
trunk build
```

Serving locally for development:
```bash
trunk serve
```

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
rustup target add wasm32-unknown-unknown
cargo check --locked --target wasm32-unknown-unknown
```

The tests cover tile geometry, conservative cluster bounds, lazy loading against
fully expanded clusters, and viewport coverage after panning, zooming, and resizing.
Regression cases include tiles missing near a cluster boundary and an empty region
inside the root's bounding box. CI runs these checks on pull requests and before
deployment. Cargo.lock is tracked so local builds and CI use the same dependencies.

To measure initial generation, cached panning, panning beyond the generated margin,
and zooming in and out:

```bash
cargo bench --locked --bench spectre_cluster_bench -- controller
```

## References

1. Smith, D., Myers, J. S, Kaplan, C. S, & Goodman-Strauss, C. (2024). [A chiral aperiodic monotile](https://doi.org/10.5070/C64264241). Combinatorial Theory, 4(2).
