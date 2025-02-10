## rui

> _This project is in an early stage of development; APIs may change frequently._

`rui` provides UI components for building fantastic desktop applications using [GPUI](https://gpui.rs).

## Components

Here are some of the components currently available:

- **Layout Components:**
  - **`Col`**: A column layout for arranging child components vertically.
  - **`Row`**: A row layout for arranging child components horizontally.

- **Interactive Components:**
  - **`Button`**: A clickable button component.
  - **`Input`**: A component for user input. __todo__
  - **`Switch`**: A toggle switch component. __todo__
  - **`Radio`**: A radio button component for selecting one option from a group. __todo__
  - **`Checkbox`**: A checkbox component for selecting multiple options. __todo__
  - **`Link`**: A hyperlink component for navigation. __todo__
  - **`Slider`**: A component for selecting a value from a range. __todo__

- **Navigation Components:**
  - **`Menu`**: A component for creating navigation menus. __todo__

- **Display Components:**
  - **`Icon`**: A component for graphical representations.  __todo__
  - **`Text`**: A component for displaying text in various styles.  __todo__
  - **`Label`**: A component that combines `Text` and an `Icon` for enhanced labeling.  __todo__

> _More components are actively under development and will be added soon!_

## Features

- `Theme`: __todo__.
- `Notification`: __todo__.

## Utility Functions

- **Color Utilities:**
  - `rgb`: Function to create RGB color values.
  - `rgba`: Function to create RGBA color values.

## Extension Methods

- **CSS-style padding and margin methods**
  - `padding`: Sets the padding of the element, in pixels.
  - `margin`: Sets the margin of the element, in pixels.

### Examples

```rust
element.padding(10.0); // Sets padding for all sides to 10.0
element.padding((10.0, 20.0)); // Sets vertical padding to 10.0 and horizontal padding to 20.0
element.padding((10.0, 20.0, 30.0, 40.0)); // Sets padding for top, right, bottom, and left respectively

element.margin(10.0); // Sets margin for all sides to 10.0
element.margin((10.0, 20.0)); // Sets vertical margin to 10.0 and horizontal margin to 20.0
element.margin((10.0, 20.0, 30.0, 40.0)); // Sets margin for top, right, bottom, and left respectively
```

- **Methods for setting padding or margin in px units**
  - `padding_top`: Sets the top padding of the element, in pixels.
  - `padding_right`: Sets the right padding of the element, in pixels.
  - `padding_bottom`: Sets the bottom padding of the element, in pixels.
  - `padding_left`: Sets the left padding of the element, in pixels.
  - `padding_x`: Sets the horizontal padding of the element, in pixels.
  - `padding_y`: Sets the vertical padding of the element, in pixels.
  - `margin_top`: Sets the top margin of the element, in pixels.
  - `margin_right`: Sets the right margin of the element, in pixels.
  - `margin_bottom`: Sets the bottom margin of the element, in pixels.
  - `margin_left`: Sets the left margin of the element, in pixels.
  - `margin_x`: Sets the horizontal margin of the element, in pixels.
  - `margin_y`: Sets the vertical margin of the element, in pixels.

## Usage

As both `GPUI` and `rui` are still in development, you will need to include them as dependencies directly from Git repositories. Add the following to your `Cargo.toml`:

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
rui = { git = "https://github.com/sandhope/rui.git" }
```

## Development

```bash
cargo run
```

More examples can be found in `examples` directory.

Checkout [DEVELOPMENT](DEVELOPMENT) to see more details.

## License

license = "GPL-3.0-or-later"
