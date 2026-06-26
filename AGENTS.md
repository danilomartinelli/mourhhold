# AGENTS.md

This file provides guidance to LLMs when working with code in this repository.

## What this is

Mourhhold is a Top-Down 2D RPG built with the **Bevy 0.19** engine in Rust. Game
design and lore live in `docs/lore/` (written in Portuguese) and are the source
of truth for world/systems decisions — read them before designing gameplay.

## Commands

The task runner is `just` (see `justfile`). Common recipes:

```bash
just run            # run the game with fast-iteration dynamic linking (--features dev)
just run-release    # optimized build
just test           # nextest if installed, else cargo test
just lint           # clippy with -D warnings (the CI gate)
just fmt            # format; `just fmt-check` to verify only
just deny           # cargo-deny: licenses / advisories / sources
just machete        # find unused dependencies
just coverage       # llvm-cov HTML + lcov
just ci             # fmt-check + lint + test + deny — run this before pushing
just setup-tools    # install nextest, llvm-cov, cargo-deny, cargo-machete
```

Run a single test: `cargo test -p mourhhold_core world_round_trips_through_grid`
(or `cargo nextest run -E 'test(world_round_trips)'`).

Toolchain is pinned in `rust-toolchain.toml` (Rust 1.96, edition 2024). On Linux,
Bevy needs `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`.

## Architecture

Two-crate Cargo workspace (`crates/*`), split by a hard rule — **rendering stays
out of `core`**:

- **`mourhhold_core`** (`crates/mourhhold_core`) — the renderer-agnostic gameplay
  layer: components, `GameState`, and grid math (`GridPosition` with
  `to_world`/`from_world`, a **pure top-down** mapping at `TILE_SIZE` = 48 px:
  grid X → world X, grid Y → world Y negated so larger rows are *lower* on screen;
  `GridPosition::depth()` gives the painter's-order `z` so lower rows draw in
  front). It is unit-tested headlessly — keep it that way so logic tests don't
  need a GPU/window. Game logic that doesn't draw belongs here.
- **`mourhhold`** (`crates/mourhhold`) — the binary: wires Bevy `DefaultPlugins`
    - `CorePlugin` plus the gameplay plugins (`assets` loads the atlases and runs
  the `Loading`→`Playing` transition, `world` builds the winter-forest tilemap and
  camera, `character` spawns the layered-paperdoll player). Owns the
  window/camera/sprites; Bevy-facing setup lives here.

New gameplay features are typically a Bevy `Plugin` registered into `App`. Mirror
`CorePlugin` (`build(&self, app)`); favor adding systems/components over editing
`main.rs` directly.

### Bevy feature configuration (important, non-obvious)

The binary uses Bevy's **default features** (`bevy = { workspace = true }`).
Do **not** switch the binary to `default-features = false` with a hand-picked
feature list: doing so previously caused a first-frame panic
(`Resource does not exist`) because a render system needed a resource provided by
an omitted feature. For a game, default features are the safe choice.

`mourhhold_core` keeps `default-features = false` to stay lean/headless, but it
must declare `bevy` **directly** (not `workspace = true`) — Cargo forbids a
workspace member from overriding the inherited dependency's `default-features`.
Wayland is added for Linux via a target-specific dependency (X11 is already in
defaults).

## Assets

Assets live at the **workspace root** `assets/` (tiles, items, icons, ui, audio),
not per-crate. Bevy resolves its asset folder from `BEVY_ASSET_ROOT`, falling
back to the running crate's `CARGO_MANIFEST_DIR` — which in this workspace would
wrongly point at `crates/mourhhold/assets`. `.cargo/config.toml` pins
`BEVY_ASSET_ROOT` to the workspace root to fix this; keep it, and load assets by
paths relative to `assets/` (e.g. `asset_server.load("tiles/terrain/...png")`).

## Conventions

- **Commits: Conventional Commits**, enforced by a `commit-msg` pre-commit hook
  and consumed by release-please for versioning/changelogs. Use `feat:`/`fix:`/etc.
- **Lints are strict**: clippy `pedantic` + `nursery` at warn, promoted to errors
  in CI. `cast_precision_loss` and `cast_possible_truncation` are allowed
  workspace-wide (grid/screen math casts i32↔f32 constantly); the other allows
  are Bevy-idiom noise (`type_complexity`, `too_many_arguments`,
  `needless_pass_by_value`). `unsafe_code` is denied. Add a workspace-level allow
  with a justification rather than scattering `#[allow]` for recurring lint noise.
- Lint/format settings live in the workspace `Cargo.toml` `[workspace.lints]`,
  `rustfmt.toml`, and `clippy.toml`. Each crate opts in via `[lints] workspace = true`.
- New permissive-license deps may trip `cargo-deny`; add the SPDX id to
  `deny.toml`'s allow-list (with the reason it's safe).

## CI

`.github/workflows/`: `ci.yml` (fmt, clippy, nextest, cargo-deny, machete),
`release-please.yml`, and `docs.yml` (rustdoc → GitHub Pages). `just ci` mirrors
the core gate locally.

## Intentional non-goals

No Dockerfile/container/Kubernetes/Skaffold — this is a desktop GPU game and is
not containerized. Configured standards and deviations are tracked in
`.project-standards.yaml`.
