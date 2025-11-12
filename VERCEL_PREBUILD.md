# Vercel Deployment - Pre-built Approach

## Issue

The original build approach failed because `dioxus-cli v0.7.1` has a dependency conflict with `axum-server` that causes compilation errors on Vercel's build servers.

**Error:** `axum-server` trait bound issues with `Buf` implementation.

## Solution

Use the **pre-built approach**: Build the WASM locally and commit the `dist/` folder to git. Vercel will simply serve the static files.

## Steps to Deploy

### 1. Build Locally (Already Done)

```bash
dx build --example demo --release --features web
mkdir -p dist
cp -r target/dx/demo/release/web/public/* dist/
```

### 2. Commit and Push

```bash
git add dist/ vercel.json .gitignore
git commit -m "Add pre-built demo for Vercel deployment"
git push origin main
```

### 3. Deploy to Vercel

Go to [vercel.com/new](https://vercel.com/new) and import your repository. Vercel will detect the `vercel.json` config and deploy the `dist/` folder.

## Configuration

**vercel.json:**
```json
{
  "buildCommand": null,
  "outputDirectory": "dist",
  "framework": null,
  "installCommand": null
}
```

This tells Vercel to skip building and just deploy the pre-built files in `dist/`.

## Files in dist/

- `index.html` - Main HTML file
- `assets/demo_bg-*.wasm` - WebAssembly bundle (1.0MB optimized)
- `assets/demo-*.js` - JavaScript glue code
- `assets/*.css` - Styles (Tailwind + custom)
- `assets/*.ico`, `assets/*.svg` - Static assets

## Updating the Demo

When you make changes:

1. **Build again:**
   ```bash
   dx build --example demo --release --features web
   rm -rf dist/*
   cp -r target/dx/demo/release/web/public/* dist/
   ```

2. **Commit and push:**
   ```bash
   git add dist/
   git commit -m "Update demo"
   git push origin main
   ```

3. **Auto-deploy:** Vercel will automatically deploy the new version.

## Alternative: GitHub Actions

To automate building on every push, create `.github/workflows/build.yml`:

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Install Dioxus CLI
        run: cargo install dioxus-cli
      
      - name: Build demo
        run: |
          dx build --example demo --release --features web
          rm -rf dist/*
          cp -r target/dx/demo/release/web/public/* dist/
      
      - name: Commit and push
        run: |
          git config --global user.name "GitHub Actions"
          git config --global user.email "actions@github.com"
          git add dist/
          git diff --staged --quiet || git commit -m "Auto-build demo [skip ci]"
          git push
```

This will build on GitHub's servers instead of locally.

## Why This Approach?

✅ **Pros:**
- No build time on Vercel (instant deploys)
- No dependency conflicts
- Faster deployments
- Works around Dioxus CLI build issues

❌ **Cons:**
- Must commit built files (~1.5MB)
- Need to rebuild manually when changing code
- Git history includes build artifacts

## File Sizes

- Total: ~1.5MB
- WASM: ~1.0MB
- JS: ~50KB
- CSS: ~400KB
- Assets: ~50KB

After gzip compression (served by Vercel): ~350KB total

## Testing Locally

Before deploying, test the built files:

```bash
cd dist
python3 -m http.server 8000
```

Open http://localhost:8000 to verify everything works.

## Deployment URL

After deploying to Vercel, your demo will be available at:
```
https://dioxus-component-<random>.vercel.app
```

You can add a custom domain in Vercel settings.
