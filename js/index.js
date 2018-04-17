WebAssembly.instantiateStreaming(fetch('../wasm/target/wasm32-unknown-unknown/release/wasm.wasm'), importObject)
.then(wasm_module => {
	alert("2 + 1 = " + wasm_module.instance.exports.add_one(2));
});