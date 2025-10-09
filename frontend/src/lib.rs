use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

// Toast types
#[derive(Clone, PartialEq)]
enum ToastVariant {
    Default,
    Success,
    Destructive,
}

#[derive(Clone)]
struct Toast {
    title: String,
    description: String,
    variant: ToastVariant,
}

// WASM module state
static mut WASM_MODULE: Option<WasmModule> = None;

struct WasmModule {
    transpile_fn: js_sys::Function,
}

// Function to load and initialize the WASM module
async fn load_wasm_module() -> Result<WasmModule, JsValue> {
    let window = window().expect("no global `window` exists");
    let origin = window.location().origin()?;
    // Import the WASM module dynamically
    let module_url = format!("{}/wasm/breakfast_bg.wasm", origin);
    // Load the WASM file
    let promise = window.fetch_with_str(&module_url);
    let resp_value = JsFuture::from(promise).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;

    // Get the array buffer 
    let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
    // Instantiate the WASM module
    let wasm_module = js_sys::WebAssembly::Module::new(&array_buffer)?;
    let imports = js_sys::Object::new();
    let instance = js_sys::WebAssembly::Instance::new(&wasm_module, &imports)?;

    // Get the transpile function
    let exports = instance.exports();    
    let transpile_fn = js_sys::Reflect::get(&exports, &JsValue::from_str("transpile_brainfuck_to_c"))?
        .dyn_into::<js_sys::Function>()?;        

    Ok(WasmModule { transpile_fn })    
}

// Function to call the WASM transpile function
fn transpile_with_wasm(code: &str) -> Result<String, String> {
    unsafe {    
        if let Some(module) = &WASM_MODULE {        
            let this = JsValue::null();
            let arg = JsValue::from_str(code);
            match module.transpile_fn.call1(&this, &arg) {
                Ok(result) => {                
                    if let Some(result_str) = result.as_string() {                    
                        if result_str.starts_with("Error:") {                        
                            Err(result_str)                            
                        } else {                        
                            Ok(result_str)                            
                        }                       
                    } else {
                        Err("Invalid result from WASM".to_string())                        
                    }                    
                }                
                Err(e) => Err(format!("WASM call failed: {:?}", e)),            
            }           
        } else {        
            Err("WASM module not loaded".to_string())        
        }
    }
}

#[component]
fn App() -> View {
    let brainfuck_code = create_signal(String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
    ));
    let c_code = create_signal(String::new());
    let is_transpiling = create_signal(false);
    let wasm_loaded = create_signal(false);
    let error = create_signal(Option::<String>::None);
    let toasts = create_signal(Vec::<Toast>::new());

    // Load WASM module on mount
    create_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            match load_wasm_module().await {
                Ok(module) => {
                    unsafe {
                        WASM_MODULE = Some(module);
                    }
                    wasm_loaded.set(true);
                    show_toast(
                        toasts,
                        "Ready".to_string(),
                        "WebAssembly module loaded successfully".to_string(),
                        ToastVariant::Success,
                    );
                }
                Err(e) => {
                    error.set(Some("Failed to load WebAssembly module. Please refresh the page.".to_string()));
                    show_toast(
                        toasts,
                        "Error".to_string(),
                        "Failed to load WebAssembly module. Please refresh the page.".to_string(),
                        ToastVariant::Destructive,
                    );
                    web_sys::console::error_1(&JsValue::from_str(&format!("WASM load error: {:?}", e)));
                }
            }
        });
    });

    // Transpile function
    let handle_transpile = move |_| {
        let code = brainfuck_code.get_clone().to_string();
        
        if code.trim().is_empty() {
            show_toast(
                toasts,
                "Error".to_string(),
                "Please enter some Brainfuck code to transpile".to_string(),
                ToastVariant::Destructive,
            );
            return;
        }

        if !wasm_loaded.get() {
            show_toast(
                toasts,
                "Error".to_string(),
                "WebAssembly module not loaded yet".to_string(),
                ToastVariant::Destructive,
            );
            return;
        }
        
        is_transpiling.set(true);
        error.set(None);
        
        match transpile_with_wasm(&code) {
            Ok(result) => {
                c_code.set(result);
                show_toast(
                    toasts,
                    "Success".to_string(),
                    "Brainfuck transpiled to C successfully".to_string(),
                    ToastVariant::Success,
                );
            }
            Err(e) => {
                error.set(Some(e.clone()));
                show_toast(
                    toasts,
                    "Error".to_string(),
                    format!("Transpilation error: {}", e),
                    ToastVariant::Destructive,
                );
            }
        }
        
        is_transpiling.set(false);
    };

    // Copy to clipboard function
    let handle_copy = move |_| {
        let code = c_code.get_clone().to_string();
        if !code.is_empty() {
            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&code);
                show_toast(
                    toasts,
                    "Copied to clipboard".to_string(),
                    "The C code has been copied to your clipboard".to_string(),
                    ToastVariant::Success,
                );
            }
        }
    };

    // Clear function
    let handle_clear = move |_| {
        c_code.set(String::new());
    };

    view! {
        div(class="container") {
            h1(c8°lass="text-3xl font-bold mb-6 text-center") { "Brainfuck to C Transpiler" }
            
            (if !wasm_loaded.get() {
                view! {
                    div(class="alert alert-info") {
                        span(class="alert-description") { "Loading WebAssembly module..." }
                    }
                }
            } else {
                view! { }
            })
            
            div(class="grid gap-6 md:grid-cols-2") {
                // Brainfuck input card
                div(class="card") {
                    div(class="card-header") {
                        h2(class="card-title") { "Brainfuck Code" }
                        p(class="card-description") { "Enter your Brainfuck code here" }
                    }
                    div(class="card-content") {
                        textarea(
                            class="textarea font-mono",
                            placeholder="Enter Brainfuck code here... (e.g. ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.)",
                            bind:value=brainfuck_code
                        )
                    }
                    div(class="card-footer") {
                        button(
                            class="btn btn-primary w-full",
                            on:click=handle_transpile,
                            disabled=is_transpiling.get() || !wasm_loaded.get()
                        ) {
                            (if is_transpiling.get() { "Transpiling..." } else { "Transpile to C" })
                        }
                    }
                }
                
                // C output card
                div(class="card") {
                    div(class="card-header") {
                        h2(class="card-title") { "C Code" }
                        p(class="card-description") { "Transpiled C code will appear here" }
                    }
                    div(class="card-content") {
                        textarea(
                            class="textarea font-mono",
                            placeholder="Transpiled C code will appear here...",
                            readonly=true,
                            bind:value=c_code
                        )
                    }
                    div(class="card-footer flex justify-between") {
                        button(
                            class="btn btn-outline",
                            on:click=handle_copy,
                            disabled=c_code.get_clone().is_empty()
                        ) {
                            "Copy to Clipboard"
                        }
                        button(
                            class="btn btn-outline",
                            on:click=handle_clear,
                            disabled=c_code.get_clone().is_empty()
                        ) {
                            "Clear"
                        }
                    }
                }
            }
            
            (if let Some(err) = error.get_clone().as_ref() {
                view! {
                    div(class="alert alert-destructive mt-6") {
                        span(class="alert-description") { (err.as_str()) }
                    }
                }
            } else {
                view! { }
            })
            
            // Toast container
                div(class="toast-container") {
                    Indexed(
                        toasts.get().iter(),
                        |_, toast| {
                            let class_name = match toast.variant {
                                ToastVariant::Success => "toast toast-success",
                                ToastVariant::Destructive => "toast toast-destructive",
                                ToastVariant::Default => "toast",
                            };
                            view! {
                                div(class=class_name) {
                                    div(class="toast-title") { (toast.title.clone()) }
                                    div(class="toast-description") { (toast.description.clone()) }
                                }
                            }
                        }
                    )
                }
        }
    }
}

// Helper function to show toast
fn show_toast(toasts: Signal<Vec<Toast>>, title: String, description: String, variant: ToastVariant) {
    let toast = Toast {
        title: title.clone(),
        description: description.clone(),
        variant,
    };
    
    toasts.update(|t| t.push(toast.clone()));
    
    // Auto-remove toast after 3 seconds
    wasm_bindgen_futures::spawn_local(async move {
        gloo_timers::future::TimeoutFuture::new(3000).await;
        toasts.update(|t| t.retain(|existing| existing.title != toast.title || existing.description != toast.description));
    });
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    
    sycamore::render(|| {
        view! {
            App {}
        }
    });
}