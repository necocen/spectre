# Infinite Spectres

![Demo](./img/demo.gif "An animation demonstrating scrolling and zooming over a plane tiled with the Spectre monotile.")

This is a program that infinitely tiles using the Spectre (more precisely, Tile(1,1)). It is written in Rust with Bevy and also runs in a web browser.
Spectre is an aperiodic monotile discovered in Reference [1]. For more details, please refer to the paper or the authors' website ( https://cs.uwaterloo.ca/~csk/spectre/ ).

Live demo here: https://spectre.necocen.info/

## How to build
### What You'll Need

- Rust (2021 edition or newer)
- wasm-bindgen for web-related functionality

### Build Commands

Running it locally:
```bash
cargo run --release
```

Making it web-ready:
```bash
wasm-pack build --target web --release
```

## References

1. Smith, D., Myers, J. S, Kaplan, C. S, & Goodman-Strauss, C. (2024). [A chiral aperiodic monotile](https://doi.org/10.5070/C64264241). Combinatorial Theory, 4(2).
