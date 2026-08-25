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

## Domain model: Species / Variety / Form

Verified against rustemon's actual struct fields (docs.rs, 2026-08-25) rather
than guessed. PokeAPI nests three levels deep and poke-core mirrors that
instead of flattening it: a Species has Varieties (distinct `pokemon`
resources with their own stats/typing — base/Mega/Gigantamax/regional forms),
and each Variety has Forms (cosmetic/battle sub-variants that don't get a
full `pokemon` record of their own).

Important discovery: `PokemonForm` carries `is_battle_only: bool` and
`is_mega: bool` directly from the API. Use these to *derive* the "not
storable in HOME" default automatically instead of hand-curating every
Mega/battle-only exception in `form_order_overrides.toml` — that file should
shrink to genuine judgment calls, not every known exception.

```rust
// pokedex/species.rs
pub struct Species {
    pub id: u32,                    // == national dex number
    pub name: String,
    pub order: u32,
    pub generation: String,
    pub has_gender_differences: bool,
    pub gender_rate: i8,             // -1 = genderless, else eighths-female
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub is_baby: bool,
    pub varieties: Vec<Variety>,
}

pub struct Variety {
    pub pokemon_id: u32,
    pub name: String,
    pub order: u32,
    pub is_default: bool,
    pub forms: Vec<Form>,
}

pub struct Form {
    pub form_id: u32,
    pub name: String,
    pub form_name: String,
    pub order: u32,
    pub is_default: bool,
    pub is_battle_only: bool,        // -> default storable = false when true
    pub is_mega: bool,
}
```

Deliberately excluded: base stats, abilities, moves, held items, sprites,
flavor text, encounter data. None of that serves ordering/identity/box
placement, which is all this domain model exists for. Battle stats/moves
belong to the `team` module instead, sourced from Showdown text — resist
mirroring the whole API response just because the data is sitting there.

## File responsibilities (poke-core)

Every module file under `poke-core/src/` currently exists as an empty stub.
Intended contents:

- `pokedex/species.rs` — `Species`/`Variety`/`Form` above, plus (once
  `sync.rs` exists) `TryFrom<rustemon::model::pokemon::PokemonSpecies>` —
  the only place raw rustemon types get converted to domain types.
- `pokedex/identity.rs` — `DexIdentity { species_id, form_id: Option<u32>,
  gender: Option<Gender> }`, `enum Gender { Male, Female }`. A small, cheap,
  Copy-able key type distinct from the heavier data structs in species.rs —
  everything downstream passes this around instead of a full `Species`.
- `pokedex/order.rs` — `OrderKey`, `OverrideRule` (mirrors
  form_order_overrides.toml), and the resolver: override lookup ->
  is_battle_only/order -> species order -> dex number.
- `pokedex/projection.rs` — `enum DexKind { Standard, Form, FormGender }`
  and the grouping function collapsing the master order into each variant.
- `pokedex/layout.rs` — `DexLayout { start_box, capacity }`, `PlacedEntry`,
  `LayoutSnapshot`, and the sync-diff move-report function.
- `pokedex/sync.rs` — the only file allowed to call rustemon/tokio directly;
  fetches, converts via species.rs's TryFrom impls, hands results to
  storage::repo to cache.
- `collection/entry.rs` — `OwnedPokemon { identity, shiny: bool,
  tags: Vec<Tag>, box_num: Option<u32>, slot_num: Option<u32> }`,
  `enum Tag { Competitive, Shiny, Archive }`.
- `collection/mod.rs` — re-exports + a thin `Collection(Vec<OwnedPokemon>)`
  wrapper with lookup-by-identity helpers.
- `team/model.rs` — `Team`, `TeamMember` (item/ability/level/nature/EVs/IVs/
  moves), `StatBlock`, `Nature` — one-to-one with Showdown's fields.
- `team/showdown.rs` — `parse(text: &str) -> Result<Team, _>` and
  `serialize(team: &Team) -> String`.
- `team/archive.rs` — `ArchivedTeam { team: Team, game: String,
  saved_at: String, notes: String }`.
- `storage/paths.rs` — `directories::ProjectDirs`-based resolution of
  cache/data/config directories.
- `storage/repo.rs` — generic `load<T: DeserializeOwned>` /
  `save<T: Serialize>` helpers used by every module that persists something.
- `error.rs` — thiserror enums: `SyncError`, `StorageError`, and a top-level
  `CoreError` composing them with `#[from]`.

## Current repo state (as of 2026-08-25, checked against actual files)

Phase 0 is mostly done, with one open gap:

- Workspace conversion, `poke-tui -> poke-core` path dependency, module file
  convention (`pokedex/mod.rs` + one file per submodule): all done and
  correct.
- `poke-core/src/pokedex/species.rs` defines `Species`, `Variety`, `Form`
  exactly matching the domain model above (all fields, correct types,
  `#[derive(Debug)]`).
- `poke-core/src/pokedex/identity.rs` defines `Gender { Male, Female }` and
  `DexIdentity { species_id: u32, form_id: Option<u32>, gender: Option<Gender> }`,
  both `#[derive(Debug, Clone, Copy)]` — matches plan.
- `poke-core/src/pokedex/mod.rs` re-exports both:
  `pub use species::{Species, Variety, Form};` /
  `pub use identity::{Gender, DexIdentity};`
- `order.rs`, `projection.rs`, `layout.rs`, `sync.rs` are still empty stubs —
  expected, those are Phase 2/1/3 scope, not Phase 0.
- **Gap**: `poke-tui/src/main.rs` was simplified to a bare
  `println!("Damn")` and no longer references `poke_core` at all. There is
  no test file anywhere in the repo. Nothing currently proves the new
  Species/Variety/Form model compiles or works across the crate boundary
  with real (hardcoded) data — the original point of Phase 0's "hardcoded
  test data" step. Recommended fix: a `#[test]` in `species.rs` building one
  small hardcoded `Species` (with a nested `Variety` and `Form`) and
  asserting on it — a better permanent artifact than reviving the old
  `println!` loop, and gets a first `cargo test` in place.
- `poke-tui/Cargo.toml` still correctly depends on `poke-core` via path.

Next up once the gap above is closed (or knowingly carried forward): Phase 1
(PokeAPI sync via `rustemon` + `tokio`) — not started.


## Rust experience level

User has finished the Rust book and small exercises; this is their first
real multi-module project. Explain new concepts (workspaces, path deps,
module visibility/re-exports, async, trait impls) as they come up rather
than assuming familiarity, but don't over-explain things already covered
by the book (ownership basics, structs/enums, `Result`/`?`).
