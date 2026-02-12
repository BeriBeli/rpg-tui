# rpg-tui

A terminal RPG prototype inspired by Dragon Quest, built with Rust and `ratatui 0.30`.

## Current Features

- World exploration on a tile map (`@` player, `H` town, `X` boss lair)
- Random encounters with turn-based battles
- Battle actions: attack, skill, defend, use items, run
- Town shop: buy items and upgrade weapon/armor tiers
- Inventory system: Potion / Ether
- Equipment system: weapon + armor stat bonuses
- Progression system: EXP, leveling, stat growth, gold rewards
- World events on floor tiles: hidden cache, stash, campfire, trap
- Game states: Exploration / Town / Battle / Victory / GameOver

## Project Structure

```text
src/
  app.rs                # terminal setup + event loop
  ui.rs                 # all ratatui rendering
  main.rs               # entry point
  game/
    mod.rs
    model.rs            # core data types
    state.rs            # state machine and flow orchestration
    balance.rs          # centralized gameplay parameters
    world.rs            # map generation
    encounter.rs        # enemy generation/scaling
    battle.rs           # battle turn resolution
    combat.rs           # low-level damage calculation
    progression.rs      # rewards + level-up logic
    town.rs             # town shop actions
    event.rs            # world event rolling + effects
```

## Controls

- Global: `q` quit, `r` restart (on result screens)
- Exploration: `WASD` / arrow keys to move, `t` open town menu when on `H`, `o` open settings
- Town:
  - `1` buy Potion
  - `2` buy Ether
  - `3` upgrade weapon
  - `4` upgrade armor
  - `5` leave town, `o` open settings
- Settings:
  - `Up/Down` or `1/2/3` select language (English/中文/日本語)
  - `Enter` apply
  - `b` / `Esc` return
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

## Language

Set `RPG_LANG` before running:

```bash
# English
set RPG_LANG=en
# Simplified Chinese
set RPG_LANG=zh-CN
# Traditional Chinese
set RPG_LANG=zh-TW
# Japanese
set RPG_LANG=ja
# Korean
set RPG_LANG=ko
cargo run
```

Or switch language at runtime from the in-game settings page (`o` in Exploration/Town).

## Next TODO

1. Add persistent save/load (player state, inventory, equipment, map seed).
2. Add one-time map objects (chests, NPC quests, cleared markers).
3. Split UI further into dedicated panels (`ui/map.rs`, `ui/sidebar.rs`, `ui/footer.rs`).
4. Add balancing profiles (easy/normal/hard) from config files.
5. Add integration tests for full game flows (town -> battle -> level-up -> boss).
6. Add localization layer for all user-facing text.

See detailed checklist: `TODO.md`
