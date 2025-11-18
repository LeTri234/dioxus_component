# Dioxus Components Showcase - Distribution

This folder contains the built production-ready showcase application.

## 📦 Contents

- `index.html` - Main entry point
- `assets/` - All compiled assets including:
  - WebAssembly modules (`.wasm`)
  - JavaScript bundles (`.js`)
  - CSS stylesheets (`.css`)
  - Static assets (favicon, images)

## 🚀 Deployment

### Option 1: Static File Server

Deploy to any static hosting service:

**Vercel:**

```bash
vercel dist
```

**Netlify:**

```bash
netlify deploy --dir=dist --prod
```

**GitHub Pages:**

```bash
# Push to gh-pages branch
git subtree push --prefix dist origin gh-pages
```

### Option 2: Local Testing

Serve locally with any static file server:

**Python:**

```bash
cd dist
python3 -m http.server 8000
# Visit http://localhost:8000
```

**Node.js (serve):**

```bash
npx serve dist
```

**PHP:**

```bash
php -S localhost:8000 -t dist
```

## 📊 Build Info

- **Total Size:** ~8MB
- **Build Mode:** Release (optimized)
- **Target:** wasm32-unknown-unknown
- **Features:** Web

## 🔄 Rebuilding

To rebuild the showcase:

```bash
./build-showcase.sh
```

Or manually:

```bash
dx build --example showcase --release
cp -r target/dx/showcase/release/web/public/* dist/
```

## 📝 Notes

- All assets are hashed for cache busting (e.g., `showcase-dxh65a1b24d78602472.js`)
- The WASM file is optimized and stripped for production
- CSS includes Tailwind utilities and component styles
- Dark theme is enabled by default
