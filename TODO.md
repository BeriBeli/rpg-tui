# TODO

## P1 Core Features

- [ ] Add save/load system (player stats, bag, equipment, map seed, current mode).
  - Acceptance: restart game and restore exact previous state from file.
- [ ] Add one-time map content (chests, NPC interaction points, cleared flags).
  - Acceptance: opened chests/NPC state persist in current run (and save file when implemented).
- [ ] Add full-flow integration tests (town -> battle -> level-up -> boss).
  - Acceptance: deterministic seed-based tests pass in CI.

## P2 Structure & Maintainability

- [ ] Split UI into submodules (`ui/map.rs`, `ui/sidebar.rs`, `ui/footer.rs`).
  - Acceptance: `ui.rs` becomes a thin composition layer.
- [ ] Add difficulty profiles (easy/normal/hard) from config.
  - Acceptance: switching profile changes encounter/event/battle scaling without code edits.
- [x] Add localization layer for all user-facing text.
  - Done: implemented with `rust-i18n` and locale files (`en`, `zh-CN`, `zh-TW`, `ja`, `ko`).
  - Usage: set `RPG_LANG=en|zh-CN|zh-TW|ja|ko` before `cargo run`.
- [x] Add an in-game settings page to switch language at runtime.
  - Done: `Settings` page is available from Exploration/Town via `o`.
  - Usage: up/down (or `1/2/3`) + Enter to apply, `b`/`Esc` to return.

## P3 Gameplay Polish

- [ ] Add event presentation layer (recent event banner / clearer reward feedback).
- [ ] Add richer town interactions (healer, inn, simple quest hooks).
- [ ] Add more enemy skill patterns and battle variety.
