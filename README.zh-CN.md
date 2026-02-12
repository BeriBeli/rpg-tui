# rpg-tui

一个基于 Rust 与 `ratatui 0.30` 实现的“勇者斗恶龙风格”终端 RPG 原型。

## 当前功能

- 地图探索（`@` 玩家、`H` 城镇、`X` Boss 巢穴）
- 随机遇敌与回合制战斗
- 战斗指令：攻击、技能、防御、使用道具、逃跑
- 城镇商店：购买道具、升级武器与护甲
- 背包系统：Potion / Ether
- 装备系统：武器与护甲提供属性加成
- 成长系统：经验、升级、属性成长、金币奖励
- 地图事件系统：金币宝藏、补给、营火、陷阱
- 游戏状态：探索 / 城镇 / 战斗 / 胜利 / 失败

## 项目结构

```text
src/
  app.rs                # 终端初始化与事件循环
  ui.rs                 # ratatui 渲染逻辑
  main.rs               # 程序入口
  game/
    mod.rs
    model.rs            # 核心数据模型
    state.rs            # 状态机与流程编排
    balance.rs          # 数值与概率参数集中管理
    world.rs            # 地图生成
    encounter.rs        # 敌人生成与成长缩放
    battle.rs           # 战斗回合结算
    combat.rs           # 底层伤害公式
    progression.rs      # 奖励与升级逻辑
    town.rs             # 城镇商店逻辑
    event.rs            # 地图事件抽取与效果应用
```

## 操作说明

- 全局：`q` 退出，`r` 在结算界面重开
- 探索：`WASD` / 方向键移动，在 `H` 上按 `t` 打开城镇菜单
- 城镇：
  - `1` 购买 Potion
  - `2` 购买 Ether
  - `3` 升级武器
  - `4` 升级护甲
  - `5` 离开城镇
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

## 下一步待办（TODO）

1. 增加存档/读档（玩家状态、背包、装备、地图种子）。
2. 增加一次性地图对象（宝箱、NPC 任务、已清理标记）。
3. 继续拆分 UI 模块（如 `ui/map.rs`、`ui/sidebar.rs`、`ui/footer.rs`）。
4. 支持数值难度配置（easy/normal/hard）并改为配置文件驱动。
5. 增加完整流程集成测试（城镇 -> 战斗 -> 升级 -> Boss）。
6. 增加文本本地化层，统一管理 UI 文案。

详细清单见：`TODO.zh-CN.md`
