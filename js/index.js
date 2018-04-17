var memory;

function print_js(s, l) {
	// Create a buffer starting at the reference to the exported string
	const stringBuffer = new Uint8Array(memory.buffer, s, l);
	
	// Create a string from this buffer
	let str = '';
	for(let i = 0; i < stringBuffer.length; i++) {
		str += String.fromCharCode(stringBuffer[i]);
	}
	
	// Print string
	console.log(str);
}

WebAssembly.instantiateStreaming(fetch('wasm/wasm.wasm'))
.then(wasm_module => {
	memory = wasm_module.instance.exports.memory;
	
	wasm_module.instance.exports.run();
});