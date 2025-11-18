# Dioxus Components Library

A comprehensive collection of reusable Dioxus 0.7 components built with Tailwind CSS v4 and following Radix UI design patterns.

[![Crates.io](https://img.shields.io/crates/v/dioxus_components.svg)](https://crates.io/crates/dioxus_components)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

✨ **11 High-Quality Components**

- 🎯 **Accordion** - Collapsible sections with single/multiple modes
- 👤 **Avatar** - User avatars with image loading and fallback support
- 🏷️ **Badge** - Flexible badges with 4 style variants
- 🔘 **Button** - Versatile buttons with 6 variants and 6 sizes
- 🃏 **Card** - Flexible card container with header, content, and footer sections
- ✅ **Checkbox** - Three-state checkbox (checked/unchecked/indeterminate)
- 💬 **Dialog** - Accessible modal dialogs with overlay and keyboard handling
- 📭 **Empty** - Empty state component for "no content" scenarios
- 🌀 **Portal** - Render content outside the parent DOM hierarchy
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

> **⚠️ Important:** Tailwind must be configured to scan the library source code, otherwise utility classes (like `size-12`, `px-4`, etc.) won't be generated. See [Step 2](#2-configure-tailwind-css) below.

### 1. Add the Rust Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus_components = "0.1.2"
dioxus = { version = "0.7.1", features = ["web"] }
```

Or add with cargo:

```bash
cargo add dioxus_components
```

### 2. Configure Tailwind CSS

**⚠️ Critical Step:** The components use Tailwind utility classes that must be included in your CSS build.

#### Option A: Use Safelist (Recommended - Works Everywhere)

Download the safelist file:

```bash
curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json
```

Add to your `tailwind.config.js`:

```javascript
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  safelist: require("./safelist.json"),
  theme: { extend: {} },
  plugins: [],
};
```

#### Option B: Tailwind v4 with @source

Update your `tailwind.css`:

```css
@import "tailwindcss";
@source "../src";
@source "../../.cargo/registry/src/*/dioxus_components-*/src";
```

#### Option C: Scan Cargo Registry (Tailwind v3)

Update your `tailwind.config.js`:

```javascript
const path = require("path");
const os = require("os");

module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    path.join(
      os.homedir(),
      ".cargo/registry/src/*/dioxus_components-*/src/**/*.rs"
    ),
  ],
  theme: { extend: {} },
  plugins: [],
};
```

### 3. Import Component Animations

Download the animations CSS:

```bash
curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css
```

Import in your `tailwind.css`:

```css
@import "./components.css";
```

**What's Included:**

- ✅ Accordion slide animations
- ✅ Checkbox fade transitions
- ✅ Spinner rotation animation
- ✅ Tooltip slide/fade animations

**Note:** The CSS file only contains custom animations. Step 2 above is required to generate utility classes like `size-12`, `px-4`, etc.

**What's included:**

- ✅ Accordion slide animations
- ✅ Checkbox fade animations
- ✅ Spinner rotation animation
- ✅ Tooltip slide and fade animations
- ✅ All necessary keyframes and transitions

**Note:** Components like Avatar, Badge, Button, Card, Dialog, Empty, and Portal use only Tailwind utility classes and don't require additional CSS.

**📚 [See detailed CSS setup guide](./CSS_SETUP.md)** for more import options and troubleshooting.

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
5. **Card** - Flexible card container with header, content, and footer sections
6. **Checkbox** - Three-state checkbox (checked/unchecked/indeterminate) with full Radix UI parity
7. **Dialog** - Modal dialogs with overlay, focus management, and keyboard controls
8. **Empty** - Empty state component for "no content" scenarios
9. **Portal** - Render content to different DOM locations (modals, overlays)
10. **Spinner** - Loading indicators with multiple sizes
11. **Tooltip** - Hover-triggered tooltips with positioning

See `COMPONENTS.md` for detailed documentation and API references for all components.

## Running the Demo

Build the library:

```bash
cargo build --lib
```

Run the demo application:

```bash
dx serve --example demo
```

This will start a development server with hot reloading and open the demo in your browser.

## Deploy to Vercel

Deploy the demo to Vercel in 3 steps:

1. **Push to GitHub:**

   ```bash
   git push origin main
   ```

2. **Import to Vercel:**

   - Go to [vercel.com/new](https://vercel.com/new)
   - Import your repository
   - Vercel will auto-detect `vercel.json`

3. **Deploy:**
   - Click "Deploy" and wait ~5-10 minutes
   - Your demo will be live at `https://your-project.vercel.app`

See `DEPLOYMENT.md` for detailed deployment instructions and alternative methods.

## Project Structure

```text
src/
├── lib.rs                 # Library entry point
├── components/            # All reusable components
│   ├── accordion/
│   ├── avatar/
│   ├── badge/
│   ├── button/
│   ├── card/
│   ├── checkbox/
│   ├── dialog/
│   ├── empty/            # NEW: Empty component
│   ├── portal/
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
2. Published to crates.io ✅ **Now published!**
3. Used locally via path dependencies

### Library Exports

The library exports all components and utilities:

```rust
pub use dioxus_components::{
    Accordion, AccordionItem, AccordionTrigger, AccordionContent,
    Avatar, AvatarImage, AvatarFallback,
    Badge, BadgeVariant,
    Button, ButtonVariant, ButtonSize,
    Portal, // NEW: Portal component
    Spinner, SpinnerSize,
    Tooltip, TooltipTrigger, TooltipContent, TooltipProvider,
    cn, // utility function
};
```

## Published on Crates.io

This library is published and available on [crates.io](https://crates.io/crates/dioxus_components).

Install it in your project:

```bash
cargo add dioxus_components
```

Or manually add to `Cargo.toml`:

```toml
[dependencies]
dioxus_components = "0.1"
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

## Changelog

### v0.1.2 - Latest Release 🚀

- ✅ **Interactive Showcase App** - New multi-page demo with routing
- ✅ **Dark Theme** - Full dark mode implementation for showcase
- ✅ **Enhanced Documentation** - Updated examples with improved styling
- ✅ **Improved Code Blocks** - Better syntax highlighting in dark theme
- ✅ **11 Component Demos** - Separate pages for each component
- ✅ **Landing Page** - Overview and quick start guide
- ✅ **Installation Guide** - Step-by-step setup instructions

### v0.1.1

- ✅ Added **Checkbox** component with three states (checked/unchecked/indeterminate)
- ✅ Added **Dialog** component with modal support, focus trap, and keyboard handling
- ✅ Added **Card** component with flexible header, content, and footer sections
- ✅ Added **Empty** component for empty state scenarios
- ✅ 11 total components now available
- ✅ Enhanced documentation with comprehensive examples
- ✅ All components use `cn` utility for class merging
- ✅ **Consolidated CSS file** - Single import for all component styles

### v0.1.0 - Published 🎉

- ✅ Initial release on crates.io
- ✅ 7 core components (Accordion, Avatar, Badge, Button, Portal, Spinner, Tooltip)
- ✅ Tailwind CSS v4 integration
- ✅ Full WAI-ARIA compliance
- ✅ Dark mode support
- ✅ Comprehensive documentation

## Resources

- [Crates.io Package](https://crates.io/crates/dioxus_components)
- [Live Showcase](https://dioxus-components-showcase.vercel.app) - Interactive demo
- [Deployment Guide](./DEPLOYMENT.md) - Deploy the showcase to various platforms
- [Quick Start Guide](./QUICKSTART.md) - ⚡ Fix missing styles in 2 minutes
- [Troubleshooting Guide](./TROUBLESHOOTING.md) - 🔧 Common issues and solutions
- [Example Project Setup](./EXAMPLE_PROJECT_SETUP.md) - Step-by-step new project guide
- [Component Documentation](./COMPONENTS.md) - Complete API reference
- [CSS Setup Guide](./CSS_SETUP.md) - Detailed styling instructions
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Radix UI](https://www.radix-ui.com/)
