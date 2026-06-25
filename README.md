# Rui

[English](./README.md) | [简体中文](./README.zh-CN.md)

`rui` provides UI components for building fantastic desktop applications using [GPUI](https://gpui.rs).

## ✨ Features

- **SwiftUI-style Syntax**: Declarative component composition with macros like `Row!{}`, `Col!{}`, and `Section!{}`.
- **Rich Components**: Buttons, inputs, dialogs, tooltips, avatars, and more.
- **Theme Support**: Customizable themes with built-in color palettes.
- **Easy to Use**: Simple API design inspired by modern UI frameworks.
- **Cross-platform**: Runs on macOS, Windows, and Linux via GPUI.

## 🚀 Quick Start

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
rui = { git = "https://github.com/sandhope/rui.git" }
```

### Basic Example

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
                        Text::new("Welcome to Rui!")
                        Button::new("Click me")
                            .on_click(|_, _, _| println!("Button clicked!"))
                    }
                }
            })
        })
        .unwrap();
    });
}
```

## 📦 Components

### Layout Components

- **`Row`** / **`Col`**: Horizontal and vertical layout containers (macro-based).
- **`Root`**: Root container macro for window content.
- **`Section`**: Sectioned layout with optional title (macro-based).

### Display Components

- **`Text`**: Text display component.
- **`Label`**: Combined text and icon label.
- **`Icon`**: SVG icon component.
- **`Avatar`**: User avatar with customizable styles.
- **`Card`**: Bordered card container.
- **`Divider`**: Horizontal or vertical separator.
- **`Headline`**: Heading text for section titles.
- **`Indicator`**: Status indicator element.
- **`Scrollbar`**: Customizable scrollbar.

### Interactive Components

- **`Button`** / **`ButtonGroup`**: Clickable buttons and button groups.
- **`Switch`**: Toggle switch.
- **`Checkbox`** / **`CheckboxGroup`**: Checkbox selection.
- **`Radio`** / **`RadioGroup`**: Radio button selection.
- **`Link`**: Hyperlink component.
- **`Modal`** / **`AlertModal`**: Modal dialogs and alerts.
- **`Toast`**: Temporary notification messages.
- **`Tooltip`**: Hover tooltip.

### Work in Progress

- `Input`: Text input field
- `Slider`: Range selector
- `Menu`: Dropdown menu
- `Table`: Data table
- `Theme`: Theme system
- `Notification`: Notification center

> More components are actively under development!

## 🎨 SwiftUI-style Macros

Rui introduces a declarative syntax similar to SwiftUI, making component composition cleaner:

```rust
// Traditional GPUI style
div()
    .flex()
    .child(Text::new("Hello"))
    .child(Button::new("Click"))

// Rui's SwiftUI-style
Row! {
    Text::new("Hello")
    Button::new("Click")
}
```

## 🛠️ Development

### Run Story Gallery

The `story` crate showcases all available components:

```bash
cargo run
```

### Run Examples

```bash
# Tiles example
cargo run --example tiles
```

More examples can be found in the `examples` directory.

See [DEVELOPMENT](DEVELOPMENT) for more details.

## 📖 Documentation

For detailed API documentation, check out the source code in `crates/rui/src/components/`.

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## 📄 License

GPL-3.0-or-later
