# Brainfuck Transpiler

A brainfuck to C transpiler in Rust 

## About Brainfuck

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. It is designed to be minimalistic and challenging, with only eight simple commands:

- `>`: Move the pointer to the right
- `<`: Move the pointer to the left
- `+`: Increment the memory cell at the pointer
- `-`: Decrement the memory cell at the pointer
- `.`: Output the character signified by the cell at the pointer
- `,`: Input a character and store it in the cell at the pointer
- `[`: Jump past the matching `]` if the cell at the pointer is 0
- `]`: Jump back to the matching `[` if the cell at the pointer is nonzero

Despite its simplicity, Brainfuck is Turing-complete, meaning it can compute anything a modern computer can (given enough time and memory).

## Technical Approach & Architecture

These are the things I had the most fun doing in this project 

### Rust to WebAssembly Compilation

Instead of implementing the Brainfuck parser and transpiler in JavaScript, I tried to leverage Rust's performance and safety features:

- **Memory Safety**: Rust's ownership system prevents memory-related bugs common in parser implementations
- **Performance**: The transpiler runs at near-native speed thanks to WebAssembly compilation
- **Code Reuse**: The same Rust code can be used for both web and command-line interfaces

### WebAssembly Integration with Next.js

While this is my second time working with WebAssembly , its still very fun because of these reasons : 

- **Dynamic Loading**: WebAssembly modules are loaded asynchronously at runtime
- **Dual-Path Strategy**: WASM files are stored in both `public/wasm/` for direct browser access and `lib/wasm/` for code imports
- **Custom Webpack Configuration**: The Next.js config is extended to properly handle WebAssembly modules
- **Type Safety**: TypeScript definitions for WebAssembly modules ensure type safety when calling Rust functions

### Security-First Deployment

Special attention is paid to security considerations, particularly for WebAssembly execution:

- **Content Security Policy**: Custom CSP headers with `wasm-unsafe-eval` directive to allow WebAssembly
- **Header Management**: Dual-layer security with both static configuration (vercel.json) and dynamic middleware
- **Path-Based Security Rules**: Different security policies for different URL paths

### CI/CD Pipeline Optimization

The project features a custom build pipeline for Vercel that solves the challenge of compiling Rust to WebAssembly in a serverless environment:

- **Shell Script Integration**: A compact build script that installs the Rust toolchain on demand
- **Build Caching**: Optimized build steps to leverage Vercel's caching capabilities
- **Error Handling**: Robust error handling in build scripts to diagnose deployment issues

### Transpilation Process

The transpilation from Brainfuck to C involves several steps:

1. **Parsing**: The Rust code parses the Brainfuck input character by character
2. **Optimization**: Several optimizations are applied, such as:
   - Loop detection and optimization
   - Contiguous operation grouping (e.g., `+++` becomes `ptr[0] += 3`)
   - Dead code elimination
3. **C Code Generation**: Structured C code is generated with:
   - A standard memory model (30,000 cells by default)
   - Proper handling of input/output operations
   - Error checking for common issues like pointer underflow/overflow
4. **WebAssembly Bridge**: The transpiled C code is returned to JavaScript through the WebAssembly bridge

This multi-step process happens near-instantly in the browser, allowing for real-time feedback as you type.


The separation between the UI layer and the core transpiler logic allows each to evolve independently, while the WebAssembly bridge provides a clean interface between them.

## Installation

### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Setup

1. Clone the repository
   ```bash
   git clone https://github.com/Pranavjeet-Naidu/bf.git
   cd bf
   ```

2. Install dependencies
   ```bash
   cd breakfast/brainfuck-transpiler
   npm install
   ```

3. Build the WebAssembly module
   ```bash
   cd ../
   wasm-pack build --target web
   cd brainfuck-transpiler
   npm run wasm:copy
   ```

4. Run the development server
   ```bash
   npm run dev
   ```

5. Open [http://localhost:3000](http://localhost:3000) in your browser

## Usage

1. Enter your Brainfuck code in the editor
2. The transpiler will automatically convert it to C code
3. Use the copy button to copy the generated C code
4. You can also view and edit the C code directly

### Example Brainfuck Code

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

This "Hello World" program will be transpiled to C code that you can compile and run.

## Development

### Project Structure

```
breakfast/
├── src/            # Rust source code
├── Cargo.toml      # Rust dependencies
├── brainfuck-transpiler/
│   ├── app/        # Next.js application
│   ├── components/ # React components
│   ├── lib/        # JavaScript/TypeScript utilities
│   ├── public/     # Static assets
│   └── styles/     # CSS styles
```


### Deployment Notes

- The project uses a custom build script (`build.sh`) to set up the Rust environment
- Content Security Policy headers are configured to allow WebAssembly execution
- WASM files are served with the appropriate MIME types

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
