# rpg-tui

A terminal RPG prototype inspired by Dragon Quest, built with Rust and `ratatui 0.30`.

## Current Features

- Tile-based exploration (`@` player, `H` town, `X` boss lair)
- One-time map objects:
  - `C` chest (open once)
  - `N` NPC point (interact once)
  - cleared floor markers persisted in current run/save
- Turn-based battle with player actions:
  - attack, Fire Slash, defend, item use, run
- Enemy skill patterns by enemy style (heavy smash, mana burn, pounce, drain, dragon skills)
- Town services:
  - shop (Potion / Ether)
  - equipment upgrades (weapon / armor)
  - healer, inn, quest board
- Quest hook:
  - accept quest, track kill progress, claim reward
- Progression system:
  - EXP, level-up growth, gold rewards
- Save/load system:
  - player stats, bag, equipment, map seed, world object states, mode, logs
- Difficulty profiles from config (`easy` / `normal` / `hard`)
- Full localization (`en`, `zh-CN`, `zh-TW`, `ja`, `ko`) and runtime language switch

## Project Structure

```text
src/
  lib.rs                # crate entry for shared modules + i18n bootstrap
  main.rs               # binary entry
  app.rs                # terminal setup + event loop
  ui.rs                 # thin UI composition layer
  ui/
    map.rs              # map panel
    sidebar.rs          # hero/log/controls panels
    footer.rs           # mode detail panel
  game/
    mod.rs
    model.rs            # core data types
    state.rs            # state machine + flow orchestration
    balance.rs          # centralized gameplay parameters
    config.rs           # difficulty profile loading
    save.rs             # save/load serialization
    world.rs            # map + one-time object generation
    encounter.rs        # enemy generation/scaling
    battle.rs           # battle turn resolution
    combat.rs           # low-level damage calculation
    progression.rs      # rewards + level-up logic
    town.rs             # town services and quest actions
    event.rs            # world event rolling + effects
config/
  difficulty.toml       # easy/normal/hard profile values
tests/
  full_flow.rs          # deterministic full-flow integration test
```

## Controls

- Global:
  - `q` quit
  - `k` save game
  - `l` load game
  - `r` restart (result screens)
- Exploration:
  - `WASD` / arrow keys move
  - `t` open town menu when on `H`
  - `o` open settings
- Town:
  - `1` buy Potion
  - `2` buy Ether
  - `3` upgrade weapon
  - `4` upgrade armor
  - `5` healer
  - `6` inn
  - `7` quest board
  - `8` leave town
- Settings:
  - `Up/Down` or `1..5` choose language
  - `Enter` apply
  - `b` / `Esc` back
- Battle:
  - `1` attack
  - `2` Fire Slash
  - `3` defend
  - `4` use Potion
  - `5` use Ether
  - `6` run

## Run and Test

```bash
cargo run
cargo test
```

## Runtime Config

- Language:

```bash
RPG_LANG=en        # en | zh-CN | zh-TW | ja | ko
cargo run
```

- Difficulty profile:

```bash
RPG_DIFFICULTY=hard
cargo run
```

- Custom difficulty config path:

```bash
RPG_DIFFICULTY_CONFIG=./config/difficulty.toml
cargo run
```

- Save file path override:

```bash
RPG_SAVE_PATH=./savegame.json
cargo run
```

## TODO Status

See `TODO.md` (updated: all current P1/P2/P3 items completed).
