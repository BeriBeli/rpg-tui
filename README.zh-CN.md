# rpg-tui

一个基于 Rust 与 `ratatui 0.30` 实现的“勇者斗恶龙风格”终端 RPG 原型。

## 当前功能

- 地图探索（`@` 玩家、`H` 城镇、`X` Boss 巢穴）
- 一次性地图对象：
  - `C` 宝箱（仅可开启一次）
  - `N` NPC 交互点（仅可交互一次）
  - 已清理地块状态可在本局与存档中保留
- 回合制战斗（攻击、火焰斩、防御、道具、逃跑）
- 敌人技能模式（重击、灼烧法力、连扑、吸取、龙系技能）
- 城镇扩展服务：
  - 商店（Potion / Ether）
  - 武器/护甲升级
  - 治疗师、旅店、任务板
- 简易任务钩子：接取、进度追踪、回城领奖
- 成长系统：经验、升级、属性成长、金币奖励
- 存档/读档：
  - 玩家属性、背包、装备、地图种子、世界对象状态、当前模式、日志
- 难度配置（`easy` / `normal` / `hard`）
- 多语言本地化（`en` / `zh-CN` / `zh-TW` / `ja` / `ko`）与运行时切换

## 项目结构

```text
src/
  lib.rs                # 共享模块入口 + i18n 初始化
  main.rs               # 二进制入口
  app.rs                # 终端初始化与事件循环
  ui.rs                 # UI 组合层
  ui/
    map.rs              # 地图面板
    sidebar.rs          # 角色/日志/操作面板
    footer.rs           # 模式详情面板
  game/
    mod.rs
    model.rs            # 核心数据模型
    state.rs            # 状态机与流程编排
    balance.rs          # 数值参数
    config.rs           # 难度配置读取
    save.rs             # 存档序列化
    world.rs            # 地图与一次性对象生成
    encounter.rs        # 敌人生成与缩放
    battle.rs           # 战斗回合结算
    combat.rs           # 底层伤害公式
    progression.rs      # 奖励与升级逻辑
    town.rs             # 城镇服务与任务逻辑
    event.rs            # 地图事件抽取与效果
config/
  difficulty.toml       # easy/normal/hard 难度配置
tests/
  full_flow.rs          # 固定种子全流程集成测试
```

## 操作说明

- 全局：
  - `q` 退出
  - `k` 存档
  - `l` 读档
  - `r` 结算界面重开
- 探索：
  - `WASD` / 方向键移动
  - 在 `H` 上按 `t` 打开城镇菜单
  - `o` 打开设置
- 城镇：
  - `1` 购买 Potion
  - `2` 购买 Ether
  - `3` 升级武器
  - `4` 升级护甲
  - `5` 治疗师
  - `6` 旅店
  - `7` 任务板
  - `8` 离开城镇
- 设置：
  - `Up/Down` 或 `1..5` 选择语言
  - `Enter` 应用
  - `b` / `Esc` 返回
- 战斗：
  - `1` 普攻
  - `2` 火焰斩
  - `3` 防御
  - `4` 使用 Potion
  - `5` 使用 Ether
  - `6` 逃跑

## 运行与测试

```bash
cargo run
cargo test
```

## 运行时配置

- 语言：

```bash
RPG_LANG=zh-CN   # en | zh-CN | zh-TW | ja | ko
cargo run
```

- 难度：

```bash
RPG_DIFFICULTY=hard
cargo run
```

- 自定义难度配置路径：

```bash
RPG_DIFFICULTY_CONFIG=./config/difficulty.toml
cargo run
```

- 自定义存档路径：

```bash
RPG_SAVE_PATH=./savegame.json
cargo run
```

## TODO 状态

详见 `TODO.zh-CN.md`（已更新：当前 P1/P2/P3 条目均已完成）。
