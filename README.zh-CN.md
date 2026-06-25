# Rui

[English](./README.md) | [简体中文](./README.zh-CN.md)

`rui` 提供用于构建出色桌面应用程序的 UI 组件，使用 [GPUI](https://gpui.rs)。

## ✨ 特性

- **SwiftUI 风格语法**: 使用 `Row!{}`、`Col!{}`、`Section!{}` 等宏进行声明式组件组合。
- **丰富组件**: 按钮、输入框、对话框、提示、头像等多种组件。
- **主题支持**: 可定制主题，内置颜色调色板。
- **易于使用**: 受现代 UI 框架启发的简洁 API 设计。
- **跨平台**: 通过 GPUI 支持 macOS、Windows 和 Linux。

## 🚀 快速开始

### 安装

在 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
rui = { git = "https://github.com/sandhope/rui.git" }
```

### 基础示例

```rust
use gpui::*;
use rui::prelude::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| {
                Root! {
                    Section! {
                        "Hello Rui";
                        Text::new("欢迎使用 Rui!")
                        Button::new("点击我")
                            .on_click(|_, _, _| println!("按钮被点击了!"))
                    }
                }
            })
        })
        .unwrap();
    });
}
```

## 📦 组件

### 布局组件

- **`Row`** / **`Col`**: 水平和垂直布局容器（基于宏）。
- **`Root`**: 窗口内容的根容器宏。
- **`Section`**: 带可选标题的分区域布局宏。

### 显示组件

- **`Text`**: 文本显示组件。
- **`Label`**: 组合文本和图标的标签。
- **`Icon`**: SVG 图标组件。
- **`Avatar`**: 可自定义样式的用户头像。
- **`Card`**: 带边框的卡片容器。
- **`Divider`**: 水平或垂直分隔线。
- **`Headline`**: 章节标题文本。
- **`Indicator`**: 状态指示器。
- **`Scrollbar`**: 可自定义的滚动条。

### 交互组件

- **`Button`** / **`ButtonGroup`**: 可点击按钮和按钮组。
- **`Switch`**: 切换开关。
- **`Checkbox`** / **`CheckboxGroup`**: 复选框选择。
- **`Radio`** / **`RadioGroup`**: 单选按钮选择。
- **`Link`**: 超链接组件。
- **`Modal`** / **`AlertModal`**: 模态对话框和警告框。
- **`Toast`**: 临时通知消息。
- **`Tooltip`**: 悬停提示。

### 开发中

- `Input`: 文本输入框
- `Slider`: 范围选择器
- `Menu`: 下拉菜单
- `Table`: 数据表格
- `Theme`: 主题系统
- `Notification`: 通知中心

> 更多组件正在积极开发中！

## 🎨 SwiftUI 风格宏

Rui 引入了类似 SwiftUI 的声明式语法，让组件组合更简洁：

```rust
// 传统 GPUI 风格
div()
    .flex()
    .child(Text::new("Hello"))
    .child(Button::new("Click"))

// Rui 的 SwiftUI 风格
Row! {
    Text::new("Hello")
    Button::new("Click")
}
```

## 🛠️ 开发

### 运行 Story 画廊

`story` crate 展示了所有可用组件：

```bash
cargo run
```

### 运行示例

```bash
# Tiles 示例
cargo run --example tiles
```

更多示例可在 `examples` 目录中找到。

详见 [DEVELOPMENT](DEVELOPMENT)。

## 📖 文档

详细 API 文档请查看源代码 `crates/rui/src/components/`。

## 🤝 贡献

欢迎贡献！提交 PR 前请先阅读我们的贡献指南。

## 📄 许可证

GPL-3.0-or-later
