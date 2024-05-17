// Assume add.wasm file exists that contains a single function adding 2 provided arguments
const fs = require('node:fs');
const { WASI } = require("wasi");
const wasi = new WASI();
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

const wasmBuffer = fs.readFileSync('../pkg_node/jsfastlogging_bg.wasm');
WebAssembly.instantiate(wasmBuffer, importObject).then(wasmModule => {
    console.log("Hello2");
    const { Logging } = wasmModule.instance.exports;
    const logging = Logging.init();
    logging.debug("Debug Message");
});
