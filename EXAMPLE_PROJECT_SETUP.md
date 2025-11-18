# Example Project Setup

This guide shows how to set up a new Dioxus project with `dioxus_components`.

## Step-by-Step Guide

### 1. Create New Dioxus Project

```bash
cargo new my-dioxus-app
cd my-dioxus-app
```

### 2. Add Dependencies

Edit `Cargo.toml`:

````toml
```toml
[dependencies]
dioxus_components = "0.1.2"
dioxus = { version = "0.7.1", features = ["web"] }
````

````

### 3. Setup Tailwind CSS

Install Tailwind CLI:

```bash
npm install -D tailwindcss @tailwindcss/cli
# or
npm install -D @tailwindcss/cli@next  # for v4
````

Download the safelist file:

```bash
curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json
```

Create `tailwind.config.js`:

```javascript
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  safelist: require("./safelist.json"),
  theme: {
    extend: {},
  },
  plugins: [],
};
```

### 4. Download Component Styles

```bash
mkdir -p src
curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css
```

### 5. Create Tailwind CSS File

Create `src/tailwind.css`:

```css
@import "./components.css";

@tailwind base;
@tailwind components;
@tailwind utilities;
```

### 6. Build CSS

```bash
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css --watch
```

### 7. Create Your App

Edit `src/main.rs`:

```rust
use dioxus::prelude::*;
use dioxus_components::*;

const STYLE: &str = include_str!("../assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }

        div {
            class: "p-8 space-y-4",

            h1 {
                class: "text-3xl font-bold",
                "Dioxus Components Demo"
            }

            // Button example
            Button {
                onclick: move |_| {
                    println!("Clicked!");
                },
                "Click me"
            }

            // Spinner example
            Spinner {
                size: SpinnerSize::Large,
            }

            // Badge example
            div {
                class: "flex gap-2",
                Badge { "Default" }
                Badge {
                    variant: BadgeVariant::Secondary,
                    "Secondary"
                }
                Badge {
                    variant: BadgeVariant::Destructive,
                    "Destructive"
                }
            }
        }
    }
}
```

### 8. Run the App

```bash
# Terminal 1: Build CSS
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css --watch

# Terminal 2: Run app
dx serve --hot-reload
```

## Troubleshooting

### Classes Not Showing

If classes like `size-12` are missing from your CSS:

1. **Verify safelist**: Make sure `safelist.json` is downloaded and referenced in `tailwind.config.js`
2. **Check content paths**: Ensure `./src/**/*.{rs,html}` is in your `tailwind.config.js`
3. **Rebuild CSS**: Delete `assets/tailwind.css` and rebuild

### Animations Not Working

If animations aren't working:

1. **Check import**: Ensure `@import "./components.css";` is in your `tailwind.css`
2. **Verify file**: Confirm `src/components.css` exists and has content
3. **Build order**: Import must come before `@tailwind` directives

### Using Tailwind v4

For Tailwind v4, use `@source` instead of safelist:

```css
@import "./components.css";
@import "tailwindcss";

@source "../src";
@source "../../.cargo/registry/src/*/dioxus_components-*/src";
```

## Complete File Structure

```
my-dioxus-app/
├── Cargo.toml
├── tailwind.config.js
├── safelist.json
├── src/
│   ├── main.rs
│   ├── components.css
│   └── tailwind.css
└── assets/
    └── tailwind.css (generated)
```

## Next Steps

- Read the [Component Documentation](./COMPONENTS.md)
- Check out the [CSS Setup Guide](./CSS_SETUP.md)
- Visit the [GitHub Repository](https://github.com/LeTri234/dioxus_component)
