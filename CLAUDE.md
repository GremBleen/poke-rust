# poke-rust — project context

A Rust learning project: a TUI companion for planning a Pokémon HOME living dex
(box/slot placement, updatable per new game release) plus a Showdown-format
team builder and archive. The user is learning Rust as they build this —
explain *why*, not just *what*, and default to explaining/guiding rather than
writing implementation code, unless they explicitly ask you to write it.

Full design reference (rationale, tables, diagrams): the "Living Dex Blueprint"
artifact — https://claude.ai/code/artifact/32a649b5-da09-4d23-907d-15e867894094
This file is a condensed, code-facing summary of the same decisions; if they
ever conflict, treat the artifact as the source of truth and flag the drift
rather than silently picking one.

## Locked decisions

- **Interface**: TUI via `ratatui` + `crossterm`. Not CLI-only, not GUI.
- **PokeAPI client**: `rustemon` crate (NOT "rustmon" — that name doesn't
  exist on crates.io). Async — requires `tokio`. Has built-in response caching.
- **Storage**: structured files (JSON/TOML via `serde`), not a database.
  Curated config (ordering overrides, dex layouts) is git-tracked under
  `data/`. Personal save data (collection, teams) lives in the OS user-data
  dir via the `directories` crate — never in the repo.
- **HOME integration scope**: planning companion only. No public HOME API
  exists, so nothing here reads/writes real HOME data — the app computes
  where things belong and the user mirrors it by hand.
- **Living dex ordering**: one unified master order at max granularity
  (species + form + gender-variant). Primary sort key is PokeAPI's
  `pokemon.order` field (family/stage-aware), overridden by a curated
  `data/form_order_overrides.toml` only for HOME-specific exceptions
  (non-storable forms, cosmetic grouping, etc). This override file is the
  direct replacement for the AustinJohnPlays spreadsheet.
- **Dex variants**: three living-dex types, all *projections* of the one
  master order — not separate ordering systems:
  - Standard — group by `species_id` only.
  - Living Form Dex — group by `(species_id, form)`.
  - Form + Gender Dex — group by `(species_id, form, gender)`.
  A group's position = the minimum order-key among its members. Which
  physical individual fills a collapsed slot is the user's call in real
  HOME, not something the app enforces.
- **Box placement**: literal box/slot numbers, not relative references.
  `box = start_box + index / capacity`, `slot = index % capacity + 1`.
  Capacity defaults to 30 (HOME's real per-box size) but MUST stay
  configurable, not a constant — plan/box-count limits change over time
  (Nintendo announced an Oct 2026 Premium capacity increase, 6000 → 9000).
  Layout (`kind`, `start_box`, `capacity`) is defined per named dex instance
  in `data/dex_layouts.toml`, since more than one instance may run at once
  (e.g. a main Form dex plus a separate shiny showcase).
- **Reflow strategy**: "recompute + move report." Keep one gapless,
  purely-sorted master order — no reserved gaps, no append-only. Every sync
  persists a `LayoutSnapshot` per dex instance; the next sync diffs old vs
  new and produces an explicit move checklist (species, old box/slot → new
  box/slot). Chosen over reserved-gap and append-only because reflows are
  rare (new species mostly append at the end) and this keeps the data model
  simple.
- **Teams**: mirror Pokémon Showdown's plain-text export format field for
  field (species/item/ability/level/EVs/IVs/nature/moves) so teams round-trip
  via copy-paste with the real Showdown site. Archive = same `Team` type +
  metadata (source game, date, notes), not a separate subsystem.
- **Errors**: `thiserror` for typed domain errors inside `poke-core`,
  `anyhow` at the `poke-tui` binary boundary.
- **Open / not yet resolved**: exactly how PokeAPI exposes gender
  differences (flag only vs. distinct `pokemon` records) needs confirming
  against live API responses — don't assume until verified. Naming
  convention for running multiple simultaneous dex instances is unsettled,
  intended to be settled when it's actually built (collection/layout phase).

## Workspace layout

Two-crate Cargo workspace (virtual manifest at root — no `[package]` at
root, `resolver = "3"`):

```
poke-rust/
  Cargo.toml          # [workspace] members = ["poke-core", "poke-tui"]
  poke-core/           # domain logic, no UI/async deps where avoidable, unit-testable
    src/
      lib.rs
      pokedex/
        mod.rs
        species.rs      # Species, Form, Variety
        identity.rs      # DexIdentity { species, form, gender }  — not yet written
        order.rs         # sort-key resolver                       — not yet written
        projection.rs    # collapses master order into the 3 dex kinds — not yet written
        layout.rs         # box/slot math + snapshot diff / move report — not yet written
        sync.rs           # rustemon calls, disk cache                  — not yet written
      collection/         # OwnedPokemon, tags — not yet written
      team/                # Team, Showdown parser, archive — not yet written
      storage/              # paths (directories crate), repo load/save — not yet written
      error.rs
  poke-tui/             # binary — ratatui event loop
    src/
      main.rs
      app.rs, event.rs, ui/  — not yet written
  data/
    form_order_overrides.toml   # not yet written
    dex_layouts.toml             # not yet written
```

Module file convention: `pokedex/mod.rs` + one file per submodule (not the
newer flat `pokedex.rs` style) — match this existing convention for
consistency rather than switching styles mid-project.

## Current repo state (as of 2026-08-25)

Phase 0 is complete and verified: `cargo run -p poke-tui` builds and prints
the sample species list.

- Root `Cargo.toml` is a converted virtual workspace manifest
  (`[workspace] members = ["poke-core", "poke-tui"]`, `resolver = "3"`).
- `poke-tui/Cargo.toml` has a path dependency on `poke-core`.
- `poke-core/src/lib.rs` -> `pub mod pokedex;`
- `poke-core/src/pokedex/mod.rs` -> `pub mod species; pub use species::{Species, Form, sample_species};`
- `poke-core/src/pokedex/species.rs` -> defines `Species { id: u32, name: String }`,
  `Form { slug: String, species_id: u32 }`, and `sample_species() -> Vec<Species>`
  with 3 hardcoded entries. Has one stray `use std::vec;` — harmless but
  redundant (the `vec!` macro is already in the prelude), worth deleting
  when noticed; likely produces an unused-import warning.
- `poke-tui/src/main.rs` calls `sample_species()` and prints each entry.
- Old root `src/main.rs` deletion and the new `poke-core`/`poke-tui`
  directories are not yet committed (`git status` still shows them as
  unstaged/untracked as of last check) — worth committing this milestone
  before moving on.
- A stale-looking `.git/index.lock` has shown up during status checks from
  this bridge (sandbox lacks delete permission to clean it up itself) — if
  git commands start failing with "Unable to create .git/index.lock: File
  exists" and no other git process is actually running, delete it by hand.

Next up: Phase 1 (PokeAPI sync via `rustemon` + `tokio`) — not started.
## Rust experience level

User has finished the Rust book and small exercises; this is their first
real multi-module project. Explain new concepts (workspaces, path deps,
module visibility/re-exports, async, trait impls) as they come up rather
than assuming familiarity, but don't over-explain things already covered
by the book (ownership basics, structs/enums, `Result`/`?`).
