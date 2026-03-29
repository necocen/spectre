# Infinite Spectres

![Demo](./img/demo.gif "An animation demonstrating scrolling and zooming over a plane tiled with the Spectre monotile.")

This is a program that infinitely tiles using the Spectre (more precisely, Tile(1,1)). It is written in Rust with [mikage](https://github.com/necocen/mikage) (a lightweight wgpu+winit framework) and also runs in a web browser.
Spectre is an aperiodic monotile discovered in Reference [1]. For more details, please refer to the paper or the authors' website ( https://cs.uwaterloo.ca/~csk/spectre/ ).

Live demo here: https://spectre.necocen.info/

## How to build
### What You'll Need

- Rust (2024 edition or newer)
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

## References

1. Smith, D., Myers, J. S, Kaplan, C. S, & Goodman-Strauss, C. (2024). [A chiral aperiodic monotile](https://doi.org/10.5070/C64264241). Combinatorial Theory, 4(2).
