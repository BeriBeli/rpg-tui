# TODO

## P1 Core Features

- [x] Add save/load system (player stats, bag, equipment, map seed, current mode).
  - Done: `game/save.rs` + `k`/`l` hotkeys wired in `game/state.rs`.
  - Acceptance: save file round-trip test added (`game::save::tests::save_round_trip_restores_exact_state`).
- [x] Add one-time map content (chests, NPC interaction points, cleared flags).
  - Done: generated in `game/world.rs`, persisted in `WorldObjects`, integrated into exploration flow.
  - Acceptance: opened/consumed map content persists in run and save.
- [x] Add full-flow integration tests (town -> battle -> level-up -> boss).
  - Done: deterministic integration test in `tests/full_flow.rs`.
  - Acceptance: `cargo test` passes with fixed-seed full flow.

## P2 Structure & Maintainability

- [x] Split UI into submodules (`ui/map.rs`, `ui/sidebar.rs`, `ui/footer.rs`).
  - Done: `ui.rs` is now a thin composition layer.
- [x] Add difficulty profiles (easy/normal/hard) from config.
  - Done: `config/difficulty.toml` + runtime loader in `game/config.rs`.
  - Acceptance: profile switch changes encounter/event/battle scaling without code edits.
- [x] Add localization layer for all user-facing text.
  - Done: `rust-i18n` with locale files (`en`, `zh-CN`, `zh-TW`, `ja`, `ko`) updated for new features.
- [x] Add an in-game settings page to switch language at runtime.
  - Done: `Settings` page available from Exploration/Town via `o`.

## P3 Gameplay Polish

- [x] Add event presentation layer (recent event banner / clearer reward feedback).
  - Done: `recent_event` banner added to footer panels; reward/event messages are surfaced clearly.
- [x] Add richer town interactions (healer, inn, simple quest hooks).
  - Done: new town actions `5..7` with healer/inn/quest board flow.
- [x] Add more enemy skill patterns and battle variety.
  - Done: enemy-style-based special actions in `game/battle.rs`.
