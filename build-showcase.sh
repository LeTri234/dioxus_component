#!/bin/bash
# Build script for Dioxus Components Showcase

set -e  # Exit on error

echo "🚀 Building Dioxus Components Showcase..."
echo ""

# Clean previous build
if [ -d "dist" ]; then
    echo "🧹 Cleaning previous dist folder..."
    rm -rf dist
fi

# Build the showcase example in release mode
echo "📦 Building showcase in release mode..."
dx build --example showcase --release

# Create dist folder and copy files
echo "📂 Copying built files to dist/..."
mkdir -p dist
cp -r target/dx/showcase/release/web/public/* dist/

# Display results
echo ""
echo "✅ Build complete!"
echo "📊 Distribution size: $(du -sh dist/ | cut -f1)"
echo "📁 Files generated:"
find dist -type f | wc -l | xargs echo "   Total files:"
echo ""
echo "📍 Built files are in: $(pwd)/dist"
echo ""
echo "To serve locally, run:"
echo "  cd dist && python3 -m http.server 8000"
echo ""
echo "Or use any static file server:"
echo "  npx serve dist"
echo "  php -S localhost:8000 -t dist"
echo ""
