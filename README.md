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

## Deployment

Cloudflare Workers serves the Trunk build in `dist` as Static Assets. The Worker
name and custom domain (`spectre.necocen.info`) are configured in
`cloudflare.config.ts`; `wrangler.config.ts` sets the assets directory.
The serving configuration preserves the index.html fallback previously used by
Pages, and does not require a Worker script.

Deployment uses the [Cloudflare CLI](https://developers.cloudflare.com/cf/) (`cf`)
and requires Node.js 22.18 or newer. Install the pinned deployment tools with
`npm ci`. `cf` is in beta and uses Wrangler internally to build this static site.
The Miniflare `sharp` override pins the fix for
[GHSA-wq5f-xc86-pv6w](https://github.com/advisories/GHSA-wq5f-xc86-pv6w).

Build and preview the Workers deployment locally:

```bash
npm ci
npm run dev
```

Deploy manually:

```bash
npx cf auth login --scopes user:read account:read workers:write \
  workers_scripts:write workers_routes:write zone:read ssl_certs:write
npm run build
npm run deploy
```

`cf` has its own login; an existing Wrangler login is not reused. `npm run build`
builds the Rust/WASM assets and writes `.cloudflare/output/v0/`. The deploy script
uses `cf deploy --prebuilt` to deploy that build without rebuilding it. To validate
the output without credentials or uploads, run `npm run deploy -- --dry-run`.

Pushes to `main` run validation, build the web assets, and deploy through
`.github/workflows/deploy-to-cloudflare-workers.yaml`. The workflow can also be
run manually. Its GitHub secrets are `CLOUDFLARE_API_TOKEN` and
`CLOUDFLARE_ACCOUNT_ID`. The token needs Workers Scripts edit permissions for
the account and Workers Routes edit / Zone read permissions for the custom
domain's zone.

For the initial cutover from Pages, first validate the deployment at
`https://spectre.necocen.workers.dev`. Remove only the Pages CNAME for
`spectre.necocen.info` in Cloudflare DNS, then run `npm run deploy`
to create the Workers Custom Domain. After confirming the custom domain serves
the Worker, remove that domain from the Pages project's Custom domains list.
The old Pages deployment can remain available at its `pages.dev` address.
The initial DNS change needs the dashboard or credentials with DNS edit
permission; the deployment token permissions listed above do not include it.

See Cloudflare's [Pages-to-Workers migration guide](https://developers.cloudflare.com/workers/static-assets/migration-guides/migrate-from-pages/).

## References

1. Smith, D., Myers, J. S, Kaplan, C. S, & Goodman-Strauss, C. (2024). [A chiral aperiodic monotile](https://doi.org/10.5070/C64264241). Combinatorial Theory, 4(2).
