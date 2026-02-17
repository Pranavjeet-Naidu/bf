// ============================================
// WASM Module
// ============================================

let wasmModule = null;

async function initWasm() {
    const badge = document.getElementById('wasm-badge');
    const statusText = document.getElementById('wasm-status-text');
    const transpileBtn = document.getElementById('transpile-btn');

    try {
        const init = await import('./wasm/bf.js');
        await init.default();
        wasmModule = init;

        badge.classList.add('ready');
        statusText.textContent = 'WASM Ready';
        transpileBtn.disabled = false;
    } catch (err) {
        console.error('Failed to load WASM:', err);
        badge.classList.add('error');
        statusText.textContent = 'WASM Failed';
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
    const clearBtn = document.getElementById('clear-btn');
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
        clearBtn.disabled = false;
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
    const copyIcon = copyBtn.querySelector('.copy-icon');
    const checkIcon = copyBtn.querySelector('.check-icon');
    const copyText = copyBtn.querySelector('.copy-text');

    if (!output.value) return;

    try {
        await navigator.clipboard.writeText(output.value);

        // Visual feedback
        copyIcon.style.display = 'none';
        checkIcon.style.display = 'block';
        copyText.textContent = 'Copied!';

        setTimeout(() => {
            copyIcon.style.display = '';
            checkIcon.style.display = 'none';
            copyText.textContent = 'Copy';
        }, 2000);
    } catch (err) {
        // Fallback for older browsers / non-https
        output.select();
        document.execCommand('copy');
    }
}

// ============================================
// Helpers
// ============================================

function showError(message) {
    const errorBar = document.getElementById('error-bar');
    const errorText = document.getElementById('error-text');
    errorText.textContent = message;
    errorBar.hidden = false;
}

function clearOutput() {
    const output = document.getElementById('c-output');
    const copyBtn = document.getElementById('copy-btn');
    const clearBtn = document.getElementById('clear-btn');
    const errorBar = document.getElementById('error-bar');

    output.value = '';
    copyBtn.disabled = true;
    clearBtn.disabled = true;
    errorBar.hidden = true;
}

function loadExample() {
    const input = document.getElementById('bf-input');
    input.value = '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>[<]<-]>>. >---. +++++++.. +++. >>. <-. <. +++. ------. --------. >>+. >++.';
    input.focus();
}

// ============================================
// Event listeners
// ============================================

document.addEventListener('DOMContentLoaded', () => {
    // Init WASM
    initWasm();

    // Theme toggle
    document.getElementById('theme-toggle').addEventListener('click', toggleTheme);

    // Transpile
    document.getElementById('transpile-btn').addEventListener('click', transpile);

    // Copy
    document.getElementById('copy-btn').addEventListener('click', copyOutput);

    // Clear
    document.getElementById('clear-btn').addEventListener('click', clearOutput);

    // Example
    document.getElementById('load-example').addEventListener('click', loadExample);

    // Keyboard shortcut: Ctrl+Enter / Cmd+Enter to transpile
    document.getElementById('bf-input').addEventListener('keydown', (e) => {
        if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
            e.preventDefault();
            transpile();
        }
    });

    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
        // Only auto-switch if user hasn't manually set a preference
        if (!localStorage.getItem('bf-theme')) {
            setTheme(e.matches ? 'dark' : 'light');
        }
    });
});
