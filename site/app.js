// ============================================
// WASM Module
// ============================================

let wasmModule = null;

async function initWasm() {
    const statusText = document.getElementById('wasm-status-text');
    const transpileBtn = document.getElementById('transpile-btn');
    const header = document.querySelector('.header');

    try {
        const init = await import('./wasm/bf.js');
        await init.default();
        wasmModule = init;

        header.classList.add('ready');
        statusText.textContent = 'SYSTEM READY';
        transpileBtn.disabled = false;
    } catch (err) {
        console.error('Failed to load WASM:', err);
        statusText.textContent = 'SYSTEM ERROR';
        statusText.style.color = 'var(--accent)';
        showError('Failed to load WebAssembly module. Please refresh the page.');
    }
}

// ============================================
// Theme
// ============================================

function getPreferredTheme() {
    const stored = localStorage.getItem('bf-theme');
    if (stored) return stored;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function setTheme(theme) {
    if (theme === 'dark') {
        document.documentElement.setAttribute('data-theme', 'dark');
    } else {
        document.documentElement.removeAttribute('data-theme');
    }
    localStorage.setItem('bf-theme', theme);
}

function toggleTheme() {
    const current = document.documentElement.hasAttribute('data-theme') ? 'dark' : 'light';
    setTheme(current === 'dark' ? 'light' : 'dark');
}

// Apply theme immediately (before page renders)
setTheme(getPreferredTheme());

// ============================================
// Transpile
// ============================================

function transpile() {
    const input = document.getElementById('bf-input');
    const output = document.getElementById('c-output');
    const copyBtn = document.getElementById('copy-btn');
    const errorBar = document.getElementById('error-bar');

    if (!wasmModule) {
        showError('WebAssembly module not loaded yet.');
        return;
    }

    const code = input.value.trim();
    if (!code) {
        showError('Please enter some Brainfuck code to transpile.');
        return;
    }

    // Hide previous error
    errorBar.hidden = true;

    try {
        const result = wasmModule.transpile_brainfuck_to_c(code);

        if (result.startsWith('Error:')) {
            showError(result);
            return;
        }

        output.value = result;
        copyBtn.disabled = false;
    } catch (err) {
        showError(`Transpilation failed: ${err.message || err}`);
    }
}

// ============================================
// Copy to clipboard
// ============================================

async function copyOutput() {
    const output = document.getElementById('c-output');
    const copyBtn = document.getElementById('copy-btn');

    if (!output.value) return;

    try {
        await navigator.clipboard.writeText(output.value);

        // Visual feedback (Text only)
        const originalText = copyBtn.textContent;
        copyBtn.textContent = 'COPIED!';

        setTimeout(() => {
            copyBtn.textContent = originalText;
        }, 2000);
    } catch (err) {
        // Fallback
        output.select();
        document.execCommand('copy');
        copyBtn.textContent = 'COPIED!';
        setTimeout(() => { copyBtn.textContent = 'Copy'; }, 2000);
    }
}

// ============================================
// Helpers
// ============================================

function showError(message) {
    const errorBar = document.getElementById('error-bar');
    errorBar.textContent = message;
    errorBar.hidden = false;

    // Auto-hide error after 5 seconds
    setTimeout(() => {
        errorBar.hidden = true;
    }, 5000);
}

function loadExample() {
    const input = document.getElementById('bf-input');
    // Just clear it and set value
    input.value = '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>[<]<-]>>. >---. +++++++.. +++. >>. <-. <. +++. ------. --------. >>+. >++.';
    // Trigger transpile automatically for instant gratification
    if (wasmModule) {
        transpile();
    }
}

// ============================================
// Event listeners
// ============================================

document.addEventListener('DOMContentLoaded', () => {
    // Init WASM
    initWasm();

    // Theme toggles
    const themeBtn = document.getElementById('theme-toggle');
    if (themeBtn) themeBtn.addEventListener('click', toggleTheme);

    const themeBtnMobile = document.getElementById('theme-toggle-mobile');
    if (themeBtnMobile) themeBtnMobile.addEventListener('click', toggleTheme);

    // Transpile
    const transpileBtn = document.getElementById('transpile-btn');
    if (transpileBtn) transpileBtn.addEventListener('click', transpile);

    // Copy
    const copyBtn = document.getElementById('copy-btn');
    if (copyBtn) copyBtn.addEventListener('click', copyOutput);

    // Example
    const exampleBtn = document.getElementById('load-example');
    if (exampleBtn) exampleBtn.addEventListener('click', loadExample);

    // Keyboard shortcut: Ctrl+Enter / Cmd+Enter
    document.getElementById('bf-input').addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            transpile();
        }
    });

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
        if (!localStorage.getItem('bf-theme')) {
            setTheme(e.matches ? 'dark' : 'light');
        }
    });
});
