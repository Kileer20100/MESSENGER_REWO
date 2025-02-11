async function initWasm() {
    const wasmModule = await WebAssembly.instantiateStreaming(fetch('my_wasm_module.wasm'));
    const wasmExports = wasmModule.instance.exports;

    // Вызов экспортируемой функции
    const result = wasmExports.my_function();
    console.log(result);
}

initWasm();
