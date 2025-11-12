# Deploying Dioxus Demo to Vercel

This guide shows how to deploy the demo example to Vercel.

## Prerequisites

1. **Vercel Account**: Sign up at [vercel.com](https://vercel.com)
2. **Vercel CLI** (optional): `npm i -g vercel`

## Method 1: Deploy via Vercel Dashboard (Recommended)

### Step 1: Push to GitHub

```bash
git add .
git commit -m "Add Vercel configuration"
git push origin main
```

### Step 2: Import to Vercel

1. Go to [vercel.com/new](https://vercel.com/new)
2. Import your `dioxus_component` repository
3. Vercel will automatically detect the `vercel.json` configuration
4. Click **Deploy**

### Step 3: Wait for Build

The build will:

- Install Rust and cargo
- Add wasm32-unknown-unknown target
- Install Dioxus CLI
- Install Tailwind CSS
- Build the demo example for web
- Deploy to Vercel's CDN

Build time: ~5-10 minutes (first deployment)

## Method 2: Deploy via CLI

### Install Vercel CLI

```bash
npm i -g vercel
```

### Login

```bash
vercel login
```

### Deploy

```bash
# Production deployment
vercel --prod

# Preview deployment
vercel
```

## Configuration Files

### `vercel.json`

Configures the build process:

- **buildCommand**: Installs dependencies and builds the WASM bundle
- **outputDirectory**: `dist/` (where Dioxus outputs files)
- **installCommand**: Installs Rust toolchain

### `build.sh`

Alternative build script if you need more control.

## Build Output

After deployment, Vercel will serve:

- `dist/index.html` - Main HTML file
- `dist/assets/*.wasm` - WebAssembly bundle
- `dist/assets/*.js` - JavaScript glue code
- `dist/assets/*.css` - Tailwind styles

## Environment Variables (Optional)

If you need environment variables:

1. Go to your project on Vercel
2. Settings → Environment Variables
3. Add variables (available during build)

## Custom Domain

1. Go to your project on Vercel
2. Settings → Domains
3. Add your custom domain

## Troubleshooting

### Build Fails - Rust Installation

If Rust installation times out, the build might exceed Vercel's time limit. Consider:

- Using a Docker-based build
- Pre-building the WASM and committing it
- Using Vercel Pro for longer build times

### Build Fails - Memory

Dioxus builds can be memory-intensive. If you hit limits:

- Add to `Cargo.toml`:
  ```toml
  [profile.release]
  opt-level = "z"
  lto = true
  codegen-units = 1
  ```

### Assets Not Loading

Check `Dioxus.toml` has correct asset paths:

```toml
[web.resource]
style = ["assets/main.css"]
```

## Local Testing

Test the production build locally:

```bash
# Build for web
dx build --example demo --release --features web

# Serve locally
cd dist
python -m http.server 8000
```

Then open http://localhost:8000

## Automatic Deployments

Every push to `main` branch will trigger a new deployment automatically.

## Preview Deployments

Every pull request gets a unique preview URL for testing before merging.

## Deployment URL

After deployment, your demo will be available at:

```
https://your-project-name.vercel.app
```

## Performance

Expected metrics:

- **First Load**: ~50-100KB (WASM + JS)
- **Lighthouse Score**: 90+ (with optimizations)
- **Time to Interactive**: < 2s

## Next Steps

1. Add `vercel.json` to git
2. Push to GitHub
3. Import to Vercel
4. Deploy!

## Alternative: Pre-build Approach

If build times are too long on Vercel:

1. Build locally:

   ```bash
   dx build --example demo --release --features web
   ```

2. Commit `dist/` folder:

   ```bash
   git add dist/
   git commit -m "Add pre-built demo"
   ```

3. Update `vercel.json`:
   ```json
   {
     "buildCommand": null,
     "outputDirectory": "dist"
   }
   ```

This skips the build on Vercel and just deploys the pre-built files.
