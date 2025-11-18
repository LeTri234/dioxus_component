# Deploying the Showcase

This guide explains how to deploy the Dioxus Components Showcase to various hosting platforms.

## Building for Production

First, build the showcase:

```bash
./build-showcase.sh
```

Or manually:

```bash
dx build --example showcase --release
mkdir -p dist
cp -r target/dx/showcase/release/web/public/* dist/
```

This creates a `dist/` folder with all production-ready files (~8MB).

## Deployment Options

### 1. Vercel (Recommended)

**Quick Deploy:**

```bash
npx vercel dist
```

**With Vercel CLI:**

```bash
# Install
npm i -g vercel

# Login
vercel login

# Deploy
cd dist
vercel --prod
```

**Configuration:**
Create `vercel.json` in the dist folder:

```json
{
  "routes": [
    { "handle": "filesystem" },
    { "src": "/(.*)", "dest": "/index.html" }
  ]
}
```

### 2. Netlify

**Drag & Drop:**

1. Go to https://app.netlify.com/drop
2. Drag the `dist` folder

**Netlify CLI:**

```bash
# Install
npm i -g netlify-cli

# Deploy
netlify deploy --dir=dist --prod
```

**Configuration:**
Create `netlify.toml` in project root:

```toml
[build]
  publish = "dist"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

### 3. GitHub Pages

**Option A: gh-pages branch**

```bash
# Build
./build-showcase.sh

# Deploy
git subtree push --prefix dist origin gh-pages
```

**Option B: GitHub Actions**
Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy Showcase

on:
  push:
    branches: [main]

jobs:
  build-and-deploy:
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

      - name: Build
        run: ./build-showcase.sh

      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

### 4. Cloudflare Pages

**Wrangler CLI:**

```bash
# Install
npm i -g wrangler

# Deploy
wrangler pages deploy dist
```

**Via Dashboard:**

1. Go to Cloudflare Pages dashboard
2. Create new project
3. Connect Git or upload `dist` folder
4. Set build command: `./build-showcase.sh`
5. Set publish directory: `dist`

### 5. Firebase Hosting

```bash
# Install
npm i -g firebase-tools

# Login
firebase login

# Initialize
firebase init hosting

# Deploy
firebase deploy
```

**Configuration (firebase.json):**

```json
{
  "hosting": {
    "public": "dist",
    "ignore": ["firebase.json", "**/.*"],
    "rewrites": [
      {
        "source": "**",
        "destination": "/index.html"
      }
    ]
  }
}
```

### 6. Custom Server (Nginx)

**Nginx Configuration:**

```nginx
server {
    listen 80;
    server_name your-domain.com;
    root /path/to/dist;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location /assets/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

**Deploy:**

```bash
# Copy files
scp -r dist/* user@server:/var/www/showcase/

# Restart Nginx
ssh user@server 'sudo systemctl restart nginx'
```

### 7. Docker

Create `Dockerfile` in dist folder:

```dockerfile
FROM nginx:alpine
COPY . /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

**Build and run:**

```bash
cd dist
docker build -t dioxus-showcase .
docker run -d -p 8080:80 dioxus-showcase
```

## Environment Configuration

### Base Path

If deploying to a subdirectory (e.g., `yourdomain.com/showcase/`), you may need to configure the base path.

Update `Dioxus.toml`:

```toml
[web.app]
base_path = "/showcase"
```

Then rebuild.

### Custom Domain

Most platforms support custom domains:

**Vercel/Netlify:**

- Add domain in dashboard
- Update DNS records

**GitHub Pages:**

- Add `CNAME` file to dist:
  ```bash
  echo "showcase.yourdomain.com" > dist/CNAME
  ```

## Performance Optimization

The built files are already optimized with:

- ✅ WASM optimization (release mode)
- ✅ Tree shaking
- ✅ Asset hashing for cache busting
- ✅ Minified JavaScript
- ✅ Stripped debug symbols

**Additional optimizations:**

1. **Enable gzip/brotli compression** on your server
2. **Set proper cache headers** for assets
3. **Use a CDN** for global distribution
4. **Enable HTTP/2** or HTTP/3

## Monitoring

After deployment, verify:

1. **Homepage loads:** Visit the base URL
2. **Routes work:** Test navigation between pages
3. **Assets load:** Check browser DevTools Network tab
4. **No console errors:** Open browser console
5. **Mobile responsive:** Test on mobile devices

## Troubleshooting

### Blank page after deployment

- Check browser console for errors
- Verify all asset paths are correct
- Ensure WASM mime type is set: `application/wasm`

### Routes return 404

- Configure server to redirect all routes to `index.html`
- Check SPA redirect rules in your hosting configuration

### Assets fail to load

- Check `base_path` configuration in `Dioxus.toml`
- Verify asset paths in `index.html` are correct
- Ensure server serves files from correct directory

## Continuous Deployment

Set up automatic deployments:

1. **Connect Git repository** to hosting platform
2. **Configure build command:** `./build-showcase.sh`
3. **Set publish directory:** `dist`
4. **Enable auto-deploy** on push to main branch

Now every push to your repository automatically rebuilds and deploys the showcase! 🚀
