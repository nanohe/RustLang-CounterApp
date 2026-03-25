# egui Counter App

A desktop GUI app for tracking multiple named counters, built with Rust and egui.

## How to run

```bash
cargo run
```

## How to use the app

The app window looks like this from top to bottom:

### 1. Add a counter

At the top there is a row with three controls:

| Control | What it does |
|---|---|
| Text box | Type a name for your counter (e.g. `Water`, `Pushups`) |
| Dropdown | Pick a category — sets a max limit |
| **Add Counter** button | Creates the counter and adds it to the list |

**Categories and their limits:**

| Category | Max count |
|---|---|
| Daily | 10 |
| Weekly | 50 |
| Unlimited | No limit |

---

### 2. Your counters list

Each counter appears as a row:

```
[Daily] Water: 3/10    +    -    Remove
```

- `+` — increment the count (stops at the category limit)
- `-` — decrement the count (stops at 0)
- `Remove` — delete this counter

---

### 3. Save / Load

- **Save** — writes all your counters to a file called `counters.json` in the folder where you ran `cargo run`
- **Load** — reads `counters.json` and restores the counters (replaces the current list)

A status message appears below the buttons confirming success or describing any error.

---

### 4. Debug info (optional)

Tick the **Show debug info** checkbox to see the raw internal state of each counter — useful for learning how the data is structured.

## Project structure

```
src/
├── main.rs      — entry point, launches the window
├── app.rs       — UI layout and user interactions
└── counter.rs   — Counter and Category types, save/load logic
```



### 这个 App 是一个桌面版多计数器追踪工具，用 Rust + egui 写的。
### 简单说就是：你可以创建多个带名字的计数器，手动点 + / - 来记录数量，然后保存到本地文件。
### 典型使用场景：

记录今天喝了几杯水（Daily，上限 10）
追踪本周做了几次俯卧撑（Weekly，上限 50）
统计某件事发生了多少次（Unlimited，不限量）

核心功能一句话总结：

给任意事项起个名字 → 选个类别（决定上限）→ 每次发生就点 + → 随时保存进度

本质上是一个轻量的习惯/频次打点工具，不需要联网，数据存本地 JSON 文件，适合想用桌面小工具做简单记录的场景。
