// WebAssembly module and initialization state
let wasmModule: any = null;
let initialized = false;
let initPromise: Promise<void> | null = null;

// Function to initialize the WebAssembly module
export async function initializeWasm(): Promise<void> {
  // If already initialized, return immediately
  if (initialized) {
    return;
  }

  // If initialization is in progress, return the promise
  if (initPromise) {
    return initPromise;
  }

  // Start initialization process
  initPromise = (async () => {
    try {
      // Skip initialization during server-side rendering
      if (typeof window === 'undefined') {
        console.log('Skipping WebAssembly initialization on server');
        return;
      }

      console.log('Initializing WebAssembly module...');

      // Dynamically import the WebAssembly module
      const init = await import('@/lib/wasm/breakfast.js');
      
      // Load the WebAssembly file from the public directory
      const wasmUrl = `${window.location.origin}/wasm/breakfast_bg.wasm`;
      
      // Initialize the WebAssembly module with the URL
      await init.default(wasmUrl);
      wasmModule = init;
      
      console.log('WebAssembly module initialized successfully');
      initialized = true;
    } catch (error) {
      console.error('Failed to initialize WebAssembly module:', error);
      initPromise = null; // Allow retry on failure
      throw error;
    }
  })();

  return initPromise;
}

// Export the function that calls into the WebAssembly module
export function transpile_brainfuck_to_c(code: string): string {
  if (!initialized || !wasmModule) {
    throw new Error('WebAssembly module not initialized yet');
  }
  
  try {
    return wasmModule.transpile_brainfuck_to_c(code);
  } catch (error) {
    console.error('Error calling WebAssembly function:', error);
    throw new Error(`Transpilation failed: ${error.message || 'Unknown error'}`);
  }
} 