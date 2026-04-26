# MID-UI Design System

**风格定位**: 深色优先，极简工业，开发者工具美学。参考 Linear、Cursor、Warp 的设计精度标准。

---

## 1. 色彩体系（Color Tokens）

所有颜色均以 Rust `(R, G, B)` 或 `(R, G, B, A)` 元组表示，与 `ThemeTokens` 字段一一对应。**禁止在组件代码中出现未经 Token 系统定义的裸色值。**

### 1.1 深色模式（Dark — 默认）

| Token             | Rust 值                  | Hex       | 语义                          |
|-------------------|--------------------------|-----------|-------------------------------|
| `bg_nav`          | `(17, 17, 20)`           | `#111114` | Activity Bar 背景             |
| `bg_stage`        | `(22, 22, 26)`           | `#16161a` | Main Stage 主背景             |
| `bg_card`         | `(28, 28, 34)`           | `#1c1c22` | 卡片、行、面板背景             |
| `bg_elevated`     | `(38, 38, 46)`           | `#26262e` | Hover 态、Active 选中背景     |
| `border_subtle`   | `(255, 255, 255, 15)`    | `—`       | 分隔线（约 6% 白）            |
| `border`          | `(255, 255, 255, 25)`    | `—`       | 组件轮廓（约 10% 白）         |
| `text_primary`    | `(228, 228, 231)`        | `#e4e4e7` | 主文本（zinc-200）            |
| `text_secondary`  | `(161, 161, 170)`        | `#a1a1aa` | 次级文本（zinc-400）          |
| `text_muted`      | `(113, 113, 122)`        | `#71717a` | 辅助/占位文本（zinc-500）     |
| `text_disabled`   | `(82, 82, 91)`           | `#52525b` | 禁用态文本（zinc-600）        |

### 1.2 浅色模式（Light）

| Token             | Rust 值                  | Hex       | 语义                          |
|-------------------|--------------------------|-----------|-------------------------------|
| `bg_nav`          | `(228, 228, 231)`        | `#e4e4e7` | Activity Bar 背景             |
| `bg_stage`        | `(250, 250, 252)`        | `#fafafc` | Main Stage 主背景             |
| `bg_card`         | `(255, 255, 255)`        | `#ffffff` | 卡片背景                      |
| `bg_elevated`     | `(240, 240, 243)`        | `#f0f0f3` | Hover 态背景                  |
| `border_subtle`   | `(0, 0, 0, 10)`          | `—`       | 分隔线（约 4% 黑）            |
| `border`          | `(0, 0, 0, 20)`          | `—`       | 组件轮廓（约 8% 黑）          |
| `text_primary`    | `(24, 24, 27)`           | `#18181b` | 主文本（zinc-900）            |
| `text_secondary`  | `(63, 63, 70)`           | `#3f3f46` | 次级文本（zinc-700）          |
| `text_muted`      | `(113, 113, 122)`        | `#71717a` | 辅助文本（zinc-500）          |
| `text_disabled`   | `(161, 161, 170)`        | `#a1a1aa` | 禁用态文本（zinc-400）        |

### 1.3 强调色（Accent Colors — 3 选 1）

| 名称  | Rust 值           | Hex       | 使用场景                  |
|-------|-------------------|-----------|---------------------------|
| Blue  | `(94, 106, 210)`  | `#5e6ad2` | 默认，指示器/按钮/链接    |
| Green | `(34, 197, 94)`   | `#22c55e` | 次选，状态/成功/高亮      |
| Red   | `(239, 68, 68)`   | `#ef4444` | 危险/警告/错误            |

### 1.4 语义固定色

| 名称         | Rust 值           | Hex       | 用途               |
|--------------|-------------------|-----------|--------------------|
| `DANGER_RED` | `(239, 68, 68)`   | `#ef4444` | 更新红点、错误状态 |

---

## 2. 排版体系（Typography）

字体优先级：`Inter` → `-apple-system` → `system-ui`。Freya 不支持字体族指定时使用系统默认。

| 角色              | font_size | font_weight      | color          | 用途                        |
|-------------------|-----------|------------------|----------------|-----------------------------|
| Display Title     | `20.0`    | `BOLD`           | `text_primary` | About 页应用名              |
| Page Title        | `16.0`    | `BOLD`           | `text_primary` | 页面 H1 标题                |
| Section Label     | `11.0`    | `BOLD`（全大写） | `text_muted`   | 分组标题（APPEARANCE）      |
| Body              | `13.0`    | `NORMAL`         | `text_primary` | 正文内容                    |
| Caption           | `12.0`    | `NORMAL`         | `text_muted`   | 描述性说明文字              |
| Nav Label         | `10.0`    | `NORMAL`/`BOLD`  | accent/muted   | 导航项标签（active=BOLD）   |
| Meta              | `11.0`    | `NORMAL`         | `text_muted`   | 版本号、版权等              |

**规则**：
- 字号只允许使用 `10, 11, 12, 13, 14, 16, 20` 这七档，禁止使用 `9, 15, 17` 等非标准值。
- Section Label 必须配合 `.text(str.to_uppercase())` 产生全大写效果。

---

## 3. 间距体系（Spacing — 4pt Grid）

**合法间距值**：`4.0, 8.0, 12.0, 16.0, 20.0, 24.0, 32.0, 48.0, 64.0`

禁止使用 `5.0, 7.0, 10.0, 15.0` 等非 4pt 倍数值。

| 场景                       | 值      |
|----------------------------|---------|
| 组件内 padding（小）        | `8.0`   |
| 组件内 padding（标准）      | `12.0`  |
| 页面内容 padding           | `24.0`  |
| 页面顶部 padding           | `48.0`  |
| 相邻元素 spacing（紧凑）    | `4.0`   |
| 相邻元素 spacing（标准）    | `8.0`   |
| 相邻元素 spacing（宽松）    | `12.0`  |
| 卡片行间距                  | `8.0`   |
| 段落间距                    | `16.0`  |

---

## 4. 圆角体系（Border Radius）

| 常量          | 值      | 用途                        |
|---------------|---------|-----------------------------|
| `RADIUS_CARD` | `12.0`  | 卡片、Drop Zone、大容器     |
| `RADIUS_CTRL` | `6.0`   | 按钮、输入框、Chip、徽章     |
| 圆形          | `99.0`  | Nav 指示器、圆点、头像      |

---

## 5. 组件规范（Component Specifications）

### 5.1 Activity Bar（导航侧栏）

```text
+-----+
| Logo| ← 32×32 logo in 36×36 accent-bg block，corner_radius=8，margin=10 上下
+-----+
|  ▌  | ← 3px 宽指示器，height=fill，accent 色（inactive 时透明）
| [I] | ← 16×16 icon，accent（active）/ text_muted（inactive）
| Nav | ← 10px label，BOLD+accent（active）/ NORMAL+text_muted（inactive）
+-----+
```

| 属性            | 值                        |
|-----------------|---------------------------|
| 侧栏宽度        | `56px`                    |
| Nav item 高度   | `40px`                    |
| Nav item 方向   | `Horizontal`              |
| Nav item 背景   | active: `with_alpha(accent, 20)` / inactive: 透明 |
| 左侧指示器      | `width=3px`, `height=fill`, `corner_radius=99`, active=accent / inactive=透明 |
| 图标尺寸        | `16×16`                   |
| Icon 容器       | `20×20`（居中图标）        |
| Label 字号      | `10.0`                    |
| Icon↔Label 间距 | `3.0`                     |
| Logo 块尺寸     | `36×36`, `corner_radius=8`, `bg=with_alpha(accent, 220)` |
| Logo SVG 尺寸   | `24×24`, `color=(255,255,255)` |
| 更新红点        | `6×6`, `DANGER_RED`, `corner_radius=3`, absolute 偏移 `(+10, -10)` |

### 5.2 Main Stage（主内容区）

| 属性           | 值                    |
|----------------|-----------------------|
| 背景           | `bg_stage`            |
| 内容 padding   | `Gaps::new_all(24.0)` |
| 页面标题字号   | `16.0 BOLD`           |
| 页面副标题字号 | `11.0 NORMAL text_muted`（全大写） |
| 标题↔副标题间距 | `margin_top=4.0`      |
| 副标题↔内容间距 | `margin_top=20.0`     |
| 滚动支持       | 必须用 `ScrollView` 包裹 |

### 5.3 Settings Page（设置页）

```text
+----------------------------------------------+
| APPEARANCE                                   | ← 11px BOLD 全大写, text_muted, margin_bottom=8
|  +------------------------------------------+|
|  | Theme        [🌙Dark][☀️Light][🖥️Auto]  || ← 行高=44, padding=12 上下
|  |------------------------------------------|
|  | Accent Color               [●] [●] [●]  ||
|  +------------------------------------------+|
| SYSTEM                                       |
|  +------------------------------------------+|
|  | VERSION: 0.1.0                           ||
|  | TOTAL DROPPED FILES: 0                   ||
|  +------------------------------------------+|
+----------------------------------------------+
```

| 属性                | 值                         |
|---------------------|----------------------------|
| 分组标题字号        | `11.0 BOLD` 全大写         |
| 分组标题颜色        | `text_muted`               |
| 分组标题下边距      | `8.0`                      |
| 行容器高度          | `44.0px`                   |
| 行容器背景          | `bg_card`                  |
| 行容器圆角          | `RADIUS_CARD`              |
| 行容器 padding      | `Gaps::new_symmetric(0, 12.0)` |
| 行之间间距          | `1.0`（border_subtle）     |
| Theme chip 圆角     | `99.0`（胶囊形）           |
| Theme chip padding  | `Gaps::new_symmetric(4, 10)` |
| Active chip 背景    | `with_alpha(accent, 220)`  |
| Active chip 文字    | `(255, 255, 255)`          |
| Inactive chip 背景  | 透明                       |
| Inactive chip 文字  | `text_muted`               |
| Accent 圆点直径     | `16px`, `corner_radius=8`  |

### 5.4 About Page（关于页）

```text
[●●●]   ← 52×52 accent-bg logo block，corner_radius=12，padding=40
APP NAME  ← 20px BOLD text_primary，margin_top=16
v0.1.0 · Beta  ← 12px text_muted，margin_top=4
A minimalist...  ← 13px text_muted，margin_top=12
[Check for Updates]  ← accent-bg button，margin_top=20
[GitHub] [Docs] [MIT·Apache-2.0]  ← link chips，margin_top=24
Licensed under MIT OR Apache-2.0  ← 11px text_muted，margin_top=32
```

| 规则                              |
|-----------------------------------|
| **垂直水平居中布局**：外层 rect 使用 `main_align(Center)` + `cross_align(Center)` |
| 内容块 `cross_align(Center)`，`padding(40.0)`，`spacing(16.0)` |
| Logo 放在内容块顶部，居中显示，尺寸 56×56，`corner_radius=14` |
| 无底部分隔线                       |
| 链接 Chip：`corner_radius=RADIUS_CTRL`, `padding=(4, 8)` |
| 更新按钮：`accent-bg`, 圆角=`RADIUS_CTRL`, 白色文字 |

### 5.5 Drop Zone（拖拽区）

```text
+------------------------------------------+
|                  [↑]                     | ← idle: 220px 高，border=1px text_muted@60%
|           Drag files here                | ← hover: border=1.5px accent, bg=accent@10%
|        or click to browse                |         文字变为 "Drop to process" accent色
+------------------------------------------+  active: 120px 高（收缩）"Drop more files"
```

| 状态   | 高度    | 边框宽 | 边框色                        | 背景              |
|--------|---------|--------|-------------------------------|-------------------|
| Idle   | `220px` | `1.0`  | `with_alpha(text_muted, 60)`  | `bg_card`         |
| Hover  | `220px` | `1.5`  | `with_alpha(accent, 255)`     | `with_alpha(accent, 25)` |
| Active | `120px` | `1.0`  | `border`                      | `bg_card`         |

**Idle 状态文本层级**：
- 主文本 `"Drag files here"`：13px NORMAL，`text_muted`
- 副文本 `"or click to browse"`：11px，`with_alpha(text_muted, 150)`，仅在 Idle 态显示

**点击行为（Click-to-browse）**：
绑定 `on_press`，触发 `rfd::AsyncFileDialog::new().pick_files().await`，将用户选中的文件路径写入 `AppState.dropped_files`。此行为是对 Freya 拖拽事件时序 bug 的降级 fallback，必须实现。

**鼠标样式**：`on_pointer_enter` 设 `CursorIcon::Pointer`，`on_pointer_leave` 恢复 `CursorIcon::Default`。

### 5.6 状态胶囊（Status Tag）

```text
[ PROCESSING ]  ← 11px BOLD 全大写，border-radius=99，padding=(4, 12)
                    bg=accent@10%，text=accent
```

### 5.7 ColorPickerPanel（颜色拾取面板）

内嵌 HSV 拾色器，作为 Settings 页 Accent Color 行的弹出内容。**无 swatch 包装层**，直接在 `Popup` 中渲染，单步触达，避免"套娃"问题。

```text
+-----------------------------+
| [  SV 渐变面板 (240×160)  ] | ← Saturation-Value 渐变矩形
|  白→透明 叠 黑→透明 叠底色  |   RGBA 绝对定位叠层
+-----------------------------+
| [      色相条 (240×16)     ] | ← Hue 彩虹渐变条
+-----------------------------+
| Hex: #5E6AD2               | ← 12px Mono，text_secondary
+-----------------------------+
```

| 属性            | 值                    |
|-----------------|-----------------------|
| SV 面板宽       | `240px`               |
| SV 面板高       | `160px`               |
| 色相条高        | `16px`                |
| 面板圆角        | `RADIUS_CARD`         |
| 内部 padding    | `12px`                |
| 元素间距        | `8px`                 |
| 背景            | `bg_card`             |

**布局注意**：SV 面板内的渐变叠层使用 `Position::new_absolute()` 实现三层叠加（底色 / 白色 S 渐变 / 黑色 V 渐变），这是 widget 内部实现，不违反页面布局层禁止绝对定位的规则。

**触发方式**：Settings 页 Accent Color 行右侧放置 40×24 色块（`corner_radius=RADIUS_CTRL`），点击色块弹出 `Popup`，`Popup` 内嵌 `ColorPickerPanel`，`Popup.show` 默认传 `false` 不自动展开。

---

## 6. 布局层级（Elevation）

Freya 无内置 shadow，用背景色区分层级：

| 层级 | 场景              | Token        |
|------|-------------------|--------------|
| L0   | 窗口底层          | `bg_stage`   |
| L1   | 侧栏              | `bg_nav`     |
| L2   | 卡片、行          | `bg_card`    |
| L3   | Hover、选中       | `bg_elevated`|

---

## 7. Do / Don't

### ✅ Do
- 所有间距使用 4pt 倍数（4/8/12/16/24/32/48/64）
- 颜色必须从 `ThemeTokens` 取，或使用 `with_alpha()` 派生
- 每个可点击区域绑定 `on_press`
- 主题/强调色变化必须在 Activity Bar 指示器、主按钮、Drop Zone Hover 三处同步体现
- Section Label 必须全大写（`.to_uppercase()`）且使用 `text_muted`
- 所有页面内容包裹在 `ScrollView` 中支持溢出滚动

### ❌ Don't
- 禁止在组件内硬编码任何裸色值（如 `(40, 40, 40)`）
- 禁止使用非 4pt 间距（5, 7, 10, 15 等）
- 禁止在 Activity Bar 底部底部 nav 项目之间使用 `SpaceBetween` 以外的方式分隔主次导航
- 禁止用 `ImageViewer` 渲染 SVG（应使用 `svg(include_bytes!(...))`）
- 禁止在 Activity Bar 以外的地方使用绝对定位（`Position::new_absolute()`）作为**页面布局手段**；widget 内部叠层（如 `ColorPickerPanel` 的 SV 渐变叠层）除外

---

## 8. Freya-Rust 实现约定

- Token 获取：`let tokens = theme_tokens(app_state.read().theme_mode);`
- Alpha 混合：`with_alpha(tokens.bg_card, 220)` 而非手动构造四元组
- 所有路由视图必须实现 `impl Component for XxxView { fn render(...) }`，禁止 `#[component]` 宏
- `use_state` / `use_consume` 只能在 `render()` 方法体顶层调用，不得在条件分支或嵌套函数中调用
- 文件拖拽事件：`on_global_file_hover` / `on_global_file_hover_cancelled` / `on_file_drop`
  - ⚠️ **已知 Freya Bug**：`on_global_file_hover_cancelled` 在**文件成功 drop 后不会触发**，只在拖拽取消（Escape/移出窗口）时触发。因此必须在 `on_file_drop` handler 中手动将 `is_file_hovering` 重置为 `false`，否则 DropZone 会永远停在 Hover 态。
  - `FileDrop` 事件（`on_file_drop`）在下次 `CursorMoved` 时才分发（winit 底层行为），属正常，不影响功能。
- 多个 closure 共享 state：`State<T>` 实现了 `Copy`，直接在各 closure 中分别捕获即可
