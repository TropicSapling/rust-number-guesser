WebAssembly.instantiateStreaming(fetch('../wasm/wasm.wasm'), importObject)
.then(wasm_module => {
	alert("2 + 1 = " + wasm_module.instance.exports.add_one(2));
});