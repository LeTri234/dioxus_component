# Migration Guide: From Demo App to Library

This document explains how the project was converted from a demo application to a reusable library.

## What Changed

### Before (Demo App Only)

- Binary target only in `src/main.rs`
- Components only available locally
- Cannot be used in other projects
- Package name: `first_dioxus`

### After (Reusable Library)

- Library target in `src/lib.rs`
- Demo application in `examples/demo.rs`
- Components can be imported in other Dioxus projects
- Package name: `dioxus_components`
- Can be published to crates.io

## Project Structure Changes

```
BEFORE:
src/
├── main.rs (contains all app code)
├── components/
├── utils/

AFTER:
src/
├── lib.rs (library entry point - NEW)
├── main.rs (minimal placeholder)
├── components/ (same structure)
└── utils/ (same structure)

examples/
└── demo.rs (demo application - MOVED HERE)
```

## Cargo.toml Changes

### Before

```toml
[package]
name = "first_dioxus"

[dependencies]
# ... dependencies ...

[features]
default = ["web"]
web = ["dioxus/web"]
```

### After

```toml
[package]
name = "dioxus_components"
description = "..."
license = "MIT"
repository = "..."

[lib]
name = "dioxus_components"
path = "src/lib.rs"

[[bin]]
name = "demo"
path = "src/main.rs"
required-features = ["demo"]

[dependencies]
# ... same dependencies ...

[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
demo = []
```

## Using as a Library

### In Your Own Project

1. Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus_components = { path = "../path/to/dioxus_components" }
```

2. Import and use components:

```rust
use dioxus::prelude::*;
use dioxus_components::{Button, Spinner};

#[component]
fn MyApp() -> Element {
    rsx! {
        Button { "Click me" }
        Spinner { }
    }
}
```

## Running the Demo

The demo is now an example that can be run with:

```bash
# Traditional way
cargo run --example demo --target wasm32-unknown-unknown
dx serve --example demo

# Or with feature flag
cargo run --features demo --example demo
```

## Library Exports

The `lib.rs` file exports all public components and utilities:

```rust
pub use components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType,
    Avatar, AvatarFallback, AvatarImage,
    Badge, BadgeVariant,
    Button, ButtonSize, ButtonVariant,
    Spinner, SpinnerSize,
    Tooltip, TooltipArrow, TooltipContent, TooltipProvider, TooltipSide, TooltipTrigger,
};

pub use utils::cn;
```

## Building the Library

```bash
# Build library only
cargo build --lib

# Build library with optimizations
cargo build --lib --release

# Generate documentation
cargo doc --lib --open

# Run tests
cargo test --lib
```

## Publishing to crates.io

To publish this library to crates.io:

1. Update version in `Cargo.toml`
2. Update `README.md` with installation instructions
3. Run: `cargo publish`

Then users can add it to their projects with:

```toml
[dependencies]
dioxus_components = "0.1.0"
```

## Component Organization

All components maintain the same subfolder structure:

```
src/components/
├── accordion/
│   ├── mod.rs (exports)
│   ├── accordion.rs (component logic)
│   └── accordion.css (styles)
├── avatar/
│   ├── mod.rs
│   ├── avatar.rs
│   └── avatar.css
# ... other components ...
└── mod.rs (main components module)
```

Each component is:

- Self-contained in its subfolder
- Properly documented
- Independently importable
- Includes its own styles

## Breaking Changes

None! The component APIs remain the same. Only the project structure changed.

## Backward Compatibility

Existing code using components will work without changes:

```rust
// This still works exactly the same
use dioxus_components::Button;

Button { "Click me" }
```

## Documentation

- **`lib.rs`** - Library crate documentation with usage examples
- **`COMPONENTS.md`** - Detailed component API reference
- **`README.md`** - Quick start guide
- **Inline docs** - Doc comments in source files

Generate and view full docs:

```bash
cargo doc --lib --open
```

## Next Steps

1. Test the library in another Dioxus project
2. Consider publishing to crates.io
3. Add more components as needed
4. Gather user feedback
5. Version and release accordingly

## Questions?

Refer to:

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Crates.io Publishing Guide](https://doc.rust-lang.org/cargo/publishing/)
