# Brainfuck Transpiler Web Interface

This is the frontend web application for the Brainfuck Transpiler. It provides a modern, user-friendly interface for transpiling Brainfuck code to C using a Rust-based WebAssembly module.

## Overview

This application is built with Next.js 15 and React 19, providing a fast and responsive user experience. The core transpilation functionality is powered by a Rust WebAssembly module that is loaded at runtime.

## Features

- Real-time transpilation from Brainfuck to C
- Syntax highlighting for both Brainfuck and C code
- Copy functionality for generated C code
- Responsive design that works on desktop and mobile
- Accessibility-focused UI components

## Getting Started

### Prerequisites

Make sure you have the following installed:
- Node.js 18 or later
- Rust (for WebAssembly compilation)
- wasm-pack

### Development

1. Install dependencies:
   ```bash
   npm install
   ```

2. Build the WebAssembly module (from the parent directory):
   ```bash
   cd ..
   wasm-pack build --target web
   cd brainfuck-transpiler
   npm run wasm:copy
   ```

3. Start the development server:
   ```bash
   npm run dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) in your browser.

### Building for Production

```bash
npm run build
```

### Running Production Build Locally

```bash
npm run start
```

## WebAssembly Integration

This application loads a WebAssembly module compiled from Rust code. The integration works as follows:

1. The Rust code is compiled to WebAssembly using wasm-pack
2. The compiled WebAssembly files are copied to both:
   - `public/wasm/` - For direct loading in the browser
   - `lib/wasm/` - For importing in the application code

3. The application uses a custom loader (`lib/wasm-loader.ts`) to dynamically load the WebAssembly module

## Deployment

This application is configured for deployment on Vercel:

1. The custom build script (`build.sh`) installs Rust and wasm-pack
2. During build, the WebAssembly module is compiled and copied to the appropriate directories
3. Content Security Policy headers are set to allow WebAssembly execution

### Important Configuration Files

- `vercel.json` - Configures build settings and response headers
- `middleware.ts` - Sets security headers including Content Security Policy
- `next.config.mjs` - Configures Next.js to work with WebAssembly
- `build.sh` - Custom build script for Vercel deployment

## Project Structure

```
/
├── app/                # Next.js App Router
├── components/         # React components
├── hooks/              # Custom React hooks
├── lib/                # Utility functions
│   ├── wasm/           # WebAssembly module files
│   └── wasm-loader.ts  # WebAssembly loader
├── public/             # Static assets
│   └── wasm/           # WebAssembly files for browser
├── styles/             # CSS styles
├── build.sh            # Vercel build script
├── middleware.ts       # Next.js middleware
├── next.config.mjs     # Next.js configuration
└── vercel.json         # Vercel deployment configuration
```

