# CSS Setup Guide

This guide explains how to properly configure CSS for `dioxus_components` in your project.

## Understanding the Setup

When you use `dioxus_components`, you need two things:

1. **Custom animations** - Accordion slides, checkbox fades, spinner rotation, tooltip animations
2. **Tailwind utility classes** - Classes like `size-12`, `px-4`, `rounded-md`, etc.

The `components.css` file contains only the animations. For utility classes, you must configure Tailwind to scan the library's source code.

## Complete Setup Instructions

### Step 1: Install the Dependency

Add to your `Cargo.toml`:

````toml
```toml
[dependencies]
dioxus_components = "0.1.2"
````

dioxus = { version = "0.7.1", features = ["web"] }

````

### Step 2: Configure Tailwind (Choose One)

#### Option A: Tailwind v4 with @source (Recommended)

Update your `tailwind.css`:

```css
@import "tailwindcss";

/* Scan your project */
@source "../src";

/* Scan dioxus_components library */
@source "../../.cargo/registry/src/*/dioxus_components-*/src";
````

This tells Tailwind v4 to scan the installed library for classes.

#### Option B: Use Safelist (Tailwind v3)

Download the safelist file:

```bash
curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json
```

Update your `tailwind.config.js`:

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

This ensures all component classes are always included.

#### Option C: Content Scanning (Tailwind v3)

Update your `tailwind.config.js`:

```javascript
const path = require("path");
const os = require("os");

module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    // Scan dioxus_components in Cargo registry
    path.join(
      os.homedir(),
      ".cargo/registry/src/*/dioxus_components-*/src/**/*.rs"
    ),
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

### Step 3: Import Component Animations

Download the animations CSS:

```bash
curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css
```

Import in your `tailwind.css`:

```css
@import "./components.css";
```

**Alternative:** Embed in Rust code:

```rust
const DIOXUS_COMPONENTS_CSS: &str = include_str!("./components.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Style { {DIOXUS_COMPONENTS_CSS} }
        // Your app
    }
}
```

## What's Included

The `components.css` file includes all necessary animations and transitions:

### Accordion Component

- `slideDown` animation - Smooth content reveal
- `slideUp` animation - Smooth content hide
- `.animate-slideDown` and `.animate-slideUp` utility classes

### Checkbox Component

- `checkboxFadeIn` animation - Smooth checkmark appearance
- `checkboxFadeOut` animation - Smooth checkmark disappearance
- Automatic animations for `[data-state="checked"]`, `[data-state="unchecked"]`, and `[data-state="indeterminate"]` states

### Spinner Component

- `spin` animation - Continuous rotation
- `.animate-spin` utility class

### Tooltip Component

- `slideUpAndFade`, `slideDownAndFade`, `slideLeftAndFade`, `slideRightAndFade` animations
- Side-specific animations based on `[data-side]` attribute
- Smooth tooltip appearance from all directions

## Components Without Custom CSS

The following components use only Tailwind utility classes and don't require additional CSS:

- **Avatar** - Image display with fallback
- **Badge** - Text labels with variants
- **Button** - Interactive buttons with variants
- **Card** - Content containers
- **Dialog** - Modal dialogs (uses Portal)
- **Empty** - Empty state displays
- **Portal** - DOM manipulation only

## Customization

You can customize the animations by overriding the keyframes in your own CSS:

```css
/* Example: Faster accordion animations */
@keyframes slideDown {
  from {
    height: 0;
  }
  to {
    height: var(--radix-accordion-content-height);
  }
}

.animate-slideDown {
  animation: slideDown 150ms cubic-bezier(0.87, 0, 0.13, 1); /* Changed from 300ms */
}
```

## Troubleshooting

### CSS Not Loading

**Issue:** Animations not working or components look unstyled.

**Solution:**

1. Verify the import path is correct
2. Ensure the CSS file is loaded before your components render
3. Check browser console for CSS loading errors

### Animation Conflicts

**Issue:** Animations conflict with other libraries.

**Solution:**

1. Use more specific selectors
2. Adjust animation timing or easing functions
3. Namespace the animations if needed

### Build System Issues

**Issue:** Import not recognized by build system.

**Solution:**

- For Vite: Use `@import` with proper paths
- For Webpack: Configure CSS loaders correctly
- For Parcel: Should work out of the box with `@import`

## Production Optimization

For production builds:

1. **Minify CSS:** Most build tools will automatically minify the CSS
2. **PurgeCSS:** The animations will be preserved as they're used by components
3. **Critical CSS:** Consider inlining critical animations for faster initial render

## Migration from Individual CSS Files

If you were previously importing individual component CSS files:

**Before:**

```css
@import "dioxus_components/src/components/accordion/accordion.css";
@import "dioxus_components/src/components/checkbox/checkbox.css";
@import "dioxus_components/src/components/spinner/spinner.css";
@import "dioxus_components/src/components/tooltip/tooltip.css";
```

**After:**

```css
@import "dioxus_components/src/components.css";
```

This consolidated approach:

- ✅ Reduces HTTP requests
- ✅ Simplifies imports
- ✅ Better for bundling and optimization
- ✅ Single source of truth for all component styles

## Support

For issues or questions about CSS setup:

- [GitHub Issues](https://github.com/LeTri234/dioxus_component/issues)
- [Documentation](./COMPONENTS.md)
