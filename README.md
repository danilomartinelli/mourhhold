# Mourhhold

An Top-Down 2D RPG built with the [Bevy](https://bevyengine.org/) game engine
in Rust. Set in the world of Ravenhollow — see [`docs/lore/`](docs/lore/) for
the cosmology, glossary, base systems, and the story acts.

## Status

Early scaffolding. The project currently boots a windowed program; gameplay
systems are being built out.

## Workspace layout

| Crate            | Path                                             | Purpose                                                                   |
| ---------------- | ------------------------------------------------ | ------------------------------------------------------------------------- |
| `mourhhold`      | [`crates/mourhhold`](crates/mourhhold)           | Game binary — window, plugins, app wiring                                 |
| `mourhhold_core` | [`crates/mourhhold_core`](crates/mourhhold_core) | Renderer-agnostic components, states, and grid math (headlessly testable) |

Game design and lore live in [`docs/lore/`](docs/lore/); runtime assets go in
[`assets/`](assets/).

## Prerequisites

- Rust (pinned via [`rust-toolchain.toml`](rust-toolchain.toml) — currently 1.96)
- On Linux, Bevy's system deps: `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`
- Optional dev tooling: `just`, `pre-commit`

Install the Cargo dev tools and git hooks in one step:

```bash
just setup-tools
pre-commit install --install-hooks
pre-commit install --hook-type commit-msg
```

## Running

```bash
just run            # fast-iteration build (Bevy dynamic linking)
just run-release    # optimized build
# or directly:
cargo run --features dev
```

## Common tasks

```bash
just fmt        # format
just lint       # clippy with -D warnings
just test       # nextest (falls back to cargo test)
just coverage   # llvm-cov HTML + lcov
just deny       # cargo-deny: licenses, advisories, sources
just machete    # find unused dependencies
just docs       # build rustdoc
just ci         # fmt-check + lint + test + deny (full local gate)
```

## Conventions

- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/) —
  enforced by a `commit-msg` pre-commit hook and consumed by
  [release-please](https://github.com/googleapis/release-please) for automated
  versioning and changelogs.
- **Lints:** clippy `pedantic` + `nursery` at warn; CI promotes warnings to
  errors. Bevy-noisy lints are allowed in the workspace `Cargo.toml`.
- **Formatting:** `rustfmt` (config in [`rustfmt.toml`](rustfmt.toml)).

## CI/CD

- [`ci.yml`](.github/workflows/ci.yml) — fmt, clippy, nextest, cargo-deny, machete
- [`release-please.yml`](.github/workflows/release-please.yml) — release PRs from Conventional Commits
- [`docs.yml`](.github/workflows/docs.yml) — rustdoc published to GitHub Pages

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE)
at your option.
