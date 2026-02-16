#!/usr/bin/env bash
set -euo pipefail

# Build the WASM module and output directly into site/wasm/
echo "Building WASM module..."
wasm-pack build --target web --out-dir site/wasm --release

# Remove files wasm-pack generates that we don't need for the site
rm -f site/wasm/.gitignore site/wasm/package.json site/wasm/README.md

echo ""
echo "Build complete! Files in site/:"
ls -lh site/wasm/
echo ""
echo "To serve locally:  python3 -m http.server 8000 -d site"
echo "To deploy to CF:   wrangler pages deploy site/"
