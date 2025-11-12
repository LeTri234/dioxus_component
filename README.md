# Dioxus Components Library

A comprehensive collection of reusable Dioxus 0.7 components built with Tailwind CSS v4 and following Radix UI design patterns.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

✨ **6 High-Quality Components**

- 🎯 **Accordion** - Collapsible sections with single/multiple modes
- 👤 **Avatar** - User avatars with image loading and fallback support
- 🏷️ **Badge** - Flexible badges with 4 style variants
- 🔘 **Button** - Versatile buttons with 6 variants and 6 sizes
- ⏳ **Spinner** - Loading indicators with multiple sizes and colors
- 💬 **Tooltip** - Hover-triggered tooltips with flexible positioning

🎨 **Styling**

- Built with **Tailwind CSS v4** for consistent, utility-first styling
- **Dark mode** support on all components
- **Customizable** through Tailwind theme configuration
- **Responsive** design patterns built-in

♿ **Accessibility**

- **WAI-ARIA** compliant
- Keyboard navigation support
- Screen reader friendly
- Focus management

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus_components = { path = "./dioxus_components" }
dioxus = { version = "0.7.1", features = ["web"] }
```

## Quick Start

### Basic Component Usage

```rust
use dioxus::prelude::*;
use dioxus_components::{Button, ButtonVariant, Spinner, SpinnerSize};

#[component]
fn App() -> Element {
    rsx! {
        div {
            class: "flex gap-4",
            Button {
                variant: ButtonVariant::Default,
                "Click me"
            }
            Spinner {
                size: SpinnerSize::Large,
                color: "text-primary"
            }
        }
    }
}
```

## Components Included

1. **Accordion** - Collapsible sections with single/multiple modes
2. **Avatar** - User avatars with image loading and fallback
3. **Badge** - Small decorative labels with 4 variants
4. **Button** - Versatile buttons with 6 variants and 6 sizes
5. **Spinner** - Loading indicators with multiple sizes
6. **Tooltip** - Hover-triggered tooltips with positioning

See `COMPONENTS.md` for detailed documentation and API references for all components.

## Running the Demo

Build the library:

```bash
cargo build --lib
```

Run the demo application:

```bash
cargo run --example demo --target wasm32-unknown-unknown
dx serve --example demo
```

## Project Structure

```
src/
├── lib.rs                 # Library entry point
├── components/            # All reusable components
│   ├── accordion/
│   ├── avatar/
│   ├── badge/
│   ├── button/
│   ├── spinner/
│   └── tooltip/
└── utils/                 # Utility functions

examples/
└── demo.rs               # Demo application

COMPONENTS.md             # Full component documentation
```

## Building as a Library

This project is configured as a Rust library that can be:

1. Used as a dependency in other Dioxus projects
2. Published to crates.io
3. Used locally via path dependencies

### Library Exports

The library exports all components and utilities:

```rust
pub use dioxus_components::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent,
    Avatar, AvatarImage, AvatarFallback,
    Badge, BadgeVariant,
    Button, ButtonVariant, ButtonSize,
    Spinner, SpinnerSize,
    Tooltip, TooltipTrigger, TooltipContent, TooltipProvider,
    cn, // utility function
};
```

## Customization

### Theme Colors

Update colors in your `tailwind.css`:

```css
@theme {
  --color-primary: hsl(221.2 83.2% 53.3%);
  --color-secondary: hsl(210 40% 96.1%);
  --color-destructive: hsl(0 84.2% 60.2%);
}
```

### Custom Styling

All components support the `class` prop:

```rust
Button {
    class: "rounded-full px-6 py-3 text-lg",
    "Custom button"
}
```

## Development

Build the library:

```bash
cargo build --lib
```

Run tests:

```bash
cargo test --lib
```

Generate documentation:

```bash
cargo doc --lib --open
```

## License

MIT

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Radix UI](https://www.radix-ui.com/)

3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```
