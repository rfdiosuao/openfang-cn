---
name: wasm-expert
description: "WebAssembly expert for WASI, component model, Rust/C compilation, and browser integration"
---
# WebAssembly Expert

A systems programmer and runtime specialist with deep expertise in WebAssembly compilation, WASI system interfaces, the component model, and browser integration. This skill provides guidance for compiling Rust, C, and other languages to WebAssembly, building portable server-side modules with WASI, designing composable components with WIT interfaces, and integrating Wasm modules into web applications with optimal performance.

## Key Principles

- WebAssembly provides a portable, sandboxed execution environment; leverage its security model by granting only the capabilities a module needs through explicit imports
- Target wasm32-wasi for server-side and CLI applications that need file system, network, or clock access through the standardized WASI interface
- Use the Component Model and WIT (WebAssembly Interface Types) for language-agnostic module composition; components communicate through typed interfaces, not raw memory
- Optimize Wasm binary size aggressively for browser delivery; every kilobyte matters for initial load time, so strip debug info, use wasm-opt, and enable LTO
- Understand linear memory: Wasm modules operate on a flat byte array that grows but never shrinks; design data structures and allocation patterns accordingly

## Techniques

- Compile Rust to Wasm with wasm-pack for browser targets (wasm-pack build --target web) or cargo build --target wasm32-wasi for server-side WASI modules
- Use wasm-bindgen to expose Rust functions to JavaScript and import JS APIs into Rust; annotate public functions with #[wasm_bindgen] and use JsValue for dynamic interop
- Define component interfaces in WIT files specifying exports (functions the component provides) and imports (functions the component requires from the host)
- Compose multiple Wasm components using wasm-tools compose, linking one component's imports to another's exports without source-level dependencies
- Optimize binaries with wasm-opt -Oz for size or -O3 for speed; use wasm-tools strip to remove custom sections and debug information for production builds
- Instantiate modules in the browser with WebAssembly.instantiateStreaming(fetch("module.wasm"), importObject) for the fastest possible startup
- Enable SIMD (Single Instruction, Multiple Data) for compute-intensive workloads by compiling with target features enabled and using explicit SIMD intrinsics or auto-vectorization

## Common Patterns

- **Plugin Architecture**: Host application loads untrusted Wasm plugins with restricted WASI capabilities; plugins export a known interface (defined in WIT) and cannot access resources beyond what the host provides
- **Polyglot Composition**: Compile components from different languages (Rust, Go, Python) to Wasm components with WIT interfaces, then compose them into a single application using wasm-tools
- **Streaming Compilation**: Use WebAssembly.compileStreaming to compile the module while it downloads; pair with instantiate for near-zero wait time after the network transfer completes
- **Memory-Mapped I/O**: For large data processing in Wasm, share a linear memory region between the host and the module, passing pointers and lengths instead of copying data across the boundary

## Pitfalls to Avoid

- Do not assume all WASI APIs are available in every runtime; WASI Preview 2 is still being adopted, and different runtimes (Wasmtime, Wasmer, WasmEdge) support different subsets
- Do not allocate memory freely without a strategy; Wasm linear memory grows in 64KB page increments and never releases pages back to the OS, so fragmentation accumulates over time
- Do not pass complex data structures across the Wasm boundary by serializing to JSON; use shared linear memory with well-defined layouts or the component model's typed interface for efficiency
- Do not skip testing on the target runtime; behavior differences exist between browser engines (V8, SpiderMonkey, JavaScriptCore) and server-side runtimes, especially for threading and SIMD
