WebAssembly.instantiateStreaming(fetch('rust-number-guesser.wasm'), importObject)
.then(obj => {
  // Call main function:
  obj.instance.exports.main();
  
  // WIP
});
