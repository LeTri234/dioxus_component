# Troubleshooting Guide

Common issues and solutions when using `dioxus_components`.

## Issue: Component Styles Not Appearing

### Symptoms

- Components render but have no styling
- Classes like `size-12`, `px-4`, `rounded-md` are missing from generated CSS
- Spinner is tiny, buttons have no padding, etc.

### Root Cause

Tailwind doesn't know about utility classes used in the `dioxus_components` library because it only scans **your** project files, not installed dependencies.

### Solution

Configure Tailwind to include the library's classes. Choose the method that works best for your setup:

#### Fix 1: Use Safelist (Easiest)

```bash
curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json
```

Update `tailwind.config.js`:

```javascript
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  safelist: require("./safelist.json"), // Add this line
  // ... rest of config
};
```

Then rebuild CSS:

```bash
rm assets/tailwind.css
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css
```

#### Fix 2: Tailwind v4 @source

Update `tailwind.css`:

```css
@import "tailwindcss";
@source "../src";
@source "../../.cargo/registry/src/*/dioxus_components-*/src";
```

#### Fix 3: Scan Cargo Registry

Update `tailwind.config.js`:

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
  // ... rest of config
};
```

### Verification

After applying the fix, check if it worked:

1. Rebuild CSS
2. Open `assets/tailwind.css`
3. Search for `size-12` (Ctrl+F)
4. If found, it's working! If not found, check config paths

## Issue: Animations Not Working

### Symptoms

- Accordion doesn't slide
- Checkbox doesn't fade
- Tooltip appears instantly without animation
- Spinner doesn't rotate

### Root Cause

The `components.css` file with custom animations is missing or not imported.

### Solution

Download the CSS file:

```bash
curl -o src/components.css https://raw.githubusercontent.com/LeTri234/dioxus_component/main/src/components.css
```

Import in your `tailwind.css` (at the top):

```css
@import "./components.css";

@tailwind base;
@tailwind components;
@tailwind utilities;
```

Rebuild CSS:

```bash
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css
```

### Verification

Check `assets/tailwind.css` for:

```css
@keyframes slideDown {
  from {
    height: 0;
  }
  to {
    height: var(--radix-accordion-content-height);
  }
}
```

If found, animations are included.

## Issue: `require('./safelist.json')` Error

### Symptoms

```
Error: Cannot find module './safelist.json'
```

### Solution

1. Make sure `safelist.json` is in the same directory as `tailwind.config.js`
2. Check file name is exactly `safelist.json` (case-sensitive)
3. Try absolute path:

```javascript
safelist: require(path.join(__dirname, "safelist.json"));
```

## Issue: Classes Still Missing After Safelist

### Symptoms

Safelist is configured but some classes still missing.

### Possible Causes

1. **Tailwind cache**: Delete `node_modules/.cache` and rebuild
2. **Old CSS file**: Delete `assets/tailwind.css` before rebuilding
3. **Config not loaded**: Check syntax errors in `tailwind.config.js`

### Solution

```bash
# Clear everything
rm -rf node_modules/.cache
rm assets/tailwind.css

# Rebuild
npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css
```

## Issue: Spinner Shows But Doesn't Rotate

### Symptoms

Spinner SVG appears but doesn't spin.

### Root Cause

Missing `@keyframes spin` animation or `animate-spin` class.

### Solution

Ensure `components.css` is imported:

```css
@import "./components.css";
```

Check if `spin` keyframe exists in generated CSS:

```bash
grep "@keyframes spin" assets/tailwind.css
```

If not found, re-download `components.css` and rebuild.

## Issue: Path Issues in @source or content

### Symptoms

```
Error: Cannot resolve path
```

### Solution for @source (Tailwind v4)

Try different path patterns:

```css
/* Relative from project root */
@source "../../.cargo/registry/src/*/dioxus_components-*/src";

/* Absolute path */
@source "/home/username/.cargo/registry/src/*/dioxus_components-*/src";
```

### Solution for content (Tailwind v3)

Use Node.js path resolution:

```javascript
const path = require("path");
const os = require("os");
const glob = require("glob");

// Find the actual path
const cargoPaths = glob.sync(
  path.join(
    os.homedir(),
    ".cargo/registry/src/*/dioxus_components-*/src/**/*.rs"
  )
);

module.exports = {
  content: ["./src/**/*.{rs,html}", ...cargoPaths],
  // ...
};
```

## Issue: Different Cargo Registry Location

### Symptoms

Can't find `.cargo/registry/src` in home directory.

### Solution

Find your Cargo home:

```bash
echo $CARGO_HOME
# or
cargo --version -v | grep home
```

Then adjust paths in config:

```javascript
path.join(
  process.env.CARGO_HOME || path.join(os.homedir(), ".cargo"),
  "registry/src/*/dioxus_components-*/src/**/*.rs"
);
```

## Issue: Works Locally But Not in CI/CD

### Symptoms

Styles work on your machine but fail in GitHub Actions, etc.

### Solution

1. **Commit safelist.json**: Make sure it's in version control
2. **Document setup**: Add setup steps to CI config
3. **Pre-build CSS**: Commit `assets/tailwind.css` to avoid build issues

Example GitHub Actions:

```yaml
- name: Install Tailwind
  run: npm install -g @tailwindcss/cli

- name: Download safelist
  run: curl -o safelist.json https://raw.githubusercontent.com/LeTri234/dioxus_component/main/safelist.json

- name: Build CSS
  run: npx @tailwindcss/cli -i ./src/tailwind.css -o ./assets/tailwind.css
```

## Still Having Issues?

1. Check the [Quick Start Guide](./QUICKSTART.md)
2. Review the [Example Project Setup](./EXAMPLE_PROJECT_SETUP.md)
3. Read the [Complete CSS Setup Guide](./CSS_SETUP.md)
4. Open an issue: [GitHub Issues](https://github.com/LeTri234/dioxus_component/issues)

## Debugging Checklist

- [ ] `safelist.json` exists and is in correct location
- [ ] `tailwind.config.js` has safelist or content config
- [ ] `src/components.css` exists and has animations
- [ ] `tailwind.css` imports `components.css`
- [ ] Generated `assets/tailwind.css` contains `size-12` class
- [ ] Generated `assets/tailwind.css` contains `@keyframes` animations
- [ ] CSS is properly linked in your Dioxus app
- [ ] Browser DevTools shows CSS is loaded
- [ ] No console errors
