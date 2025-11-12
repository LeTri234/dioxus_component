# Quick Vercel Deployment Checklist

## ✅ Prerequisites

- [x] Vercel account (sign up at vercel.com)
- [x] GitHub repository with your code
- [x] vercel.json configured
- [x] Build verified locally (1.0MB WASM bundle)

## 📋 Deployment Steps

### Option 1: Vercel Dashboard (Easiest)

1. **Commit and Push:**

   ```bash
   git add vercel.json DEPLOYMENT.md .gitignore build.sh
   git commit -m "Add Vercel deployment configuration"
   git push origin main
   ```

2. **Import to Vercel:**

   - Visit: https://vercel.com/new
   - Click "Import Git Repository"
   - Select your `dioxus_component` repo
   - Vercel auto-detects `vercel.json` ✅

3. **Deploy:**
   - Click "Deploy"
   - Wait ~5-10 minutes (first build)
   - Done! 🎉

### Option 2: Vercel CLI

```bash
# Install CLI
npm i -g vercel

# Login
vercel login

# Deploy
vercel --prod
```

## 📦 Build Process

Vercel will:

1. Install Rust toolchain (~2 min)
2. Add wasm32 target (~30 sec)
3. Install Dioxus CLI (~2 min)
4. Install Tailwind CSS (~30 sec)
5. Build WASM bundle (~3-5 min)
6. Deploy to CDN (~30 sec)

**Total:** ~8-10 minutes first time, ~4-6 minutes subsequent builds

## 🎯 Expected Output

- **URL:** `https://your-project.vercel.app`
- **Bundle Size:** ~1.0MB WASM (optimized with wasm-opt)
- **Assets:** HTML, WASM, JS, CSS
- **Auto Deploy:** Every push to `main`

## 🔍 Verify Deployment

After deployment:

```bash
curl -I https://your-project.vercel.app
```

Should return `200 OK`

## ⚡ Performance

- **First Load:** ~1.0MB WASM + ~50KB JS/CSS
- **Lighthouse Score:** 85-95
- **Time to Interactive:** < 3s

## 🐛 Troubleshooting

### Build Times Out

- Vercel free tier: 45 min limit (should be enough)
- If timeout: Use Vercel Pro or pre-build approach

### Assets Not Loading

Check browser console for 404s. Ensure:

- `vercel.json` has correct `outputDirectory`
- Assets in `assets/` folder are copied

### Rust Installation Fails

- Usually temporary network issue
- Click "Redeploy" in Vercel dashboard

## 📊 Monitoring

View logs:

- Vercel Dashboard → Your Project → Deployments → View Logs

## 🔄 Continuous Deployment

Every commit to `main`:

1. Triggers automatic build
2. Creates preview deployment
3. Promotes to production if successful

Pull requests get preview URLs:

- `https://your-project-git-branch.vercel.app`

## 🎨 Custom Domain

Add custom domain:

1. Vercel Dashboard → Settings → Domains
2. Add your domain
3. Update DNS records

## 📝 Next Steps

- [ ] Deploy to Vercel
- [ ] Test on mobile devices
- [ ] Add custom domain (optional)
- [ ] Set up analytics (optional)
- [ ] Configure caching (optional)

## 🔗 Useful Links

- Vercel Dashboard: https://vercel.com/dashboard
- Deployment Docs: https://vercel.com/docs
- Status: https://www.vercel-status.com/
