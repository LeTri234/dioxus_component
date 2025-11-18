# Quick Setup - TL;DR

Can't get component styles working? Here's the quick fix.

## The Problem

You installed `dioxus_components` but classes like `size-12`, `px-4`, `rounded-md` etc. are missing from your generated CSS.

**Why?** Tailwind doesn't know about classes inside the library's Rust files.

## The Solution (Choose One)

### Option 1: Safelist (Easiest)

```bash
# Download the safelist
curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json
```

Add to `tailwind.config.js`:

```javascript
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  safelist: require("./safelist.json"), // ← Add this line
  theme: { extend: {} },
  plugins: [],
};
```

### Option 2: Tailwind v4 @source

Update `tailwind.css`:

```css
@import "tailwindcss";
@source "../src";
@source "../../.cargo/registry/src/*/dioxus_components-*/src"; // ← Add this
```

### Option 3: Scan Cargo Registry

Update `tailwind.config.js`:

```javascript
const path = require("path");
const os = require("os");

module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    // Add this line:
    path.join(
      os.homedir(),
      ".cargo/registry/src/*/dioxus_components-*/src/**/*.rs"
    ),
  ],
  theme: { extend: {} },
  plugins: [],
};
```

## Don't Forget Animations

Download `components.css`:

```bash
curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css
```

Import in your `tailwind.css`:

```css
@import "./components.css";
```

## Rebuild CSS

```bash
# Delete old CSS
rm assets/tailwind.css

# Rebuild
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css
```

## Test It

```rust
use dioxus_components::*;

#[component]
fn App() -> Element {
    rsx! {
        Spinner { size: SpinnerSize::XLarge }
    }
}
```

If the spinner appears with correct size, it's working!

## Still Not Working?

1. Check `safelist.json` is in the same folder as `tailwind.config.js`
2. Verify `require('./safelist.json')` path is correct
3. Make sure you rebuilt the CSS after config changes
4. Open `assets/tailwind.css` and search for `size-12` - if not found, Tailwind isn't scanning correctly

## Full Documentation

- [Complete Setup Guide](./CSS_SETUP.md)
- [Example Project Setup](./EXAMPLE_PROJECT_SETUP.md)
- [Component Documentation](./COMPONENTS.md)
