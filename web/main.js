let wasm = null;
let canvas = null;
let context = null;

let env = {
    js_console: (type, ptr, len) => {
        const view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
        const utf8dec = new TextDecoder("utf-8");
        const str = utf8dec.decode(view);
        if (type == 1) {
            console.error(str);
        } else {
            console.log(str);
        }
    }
};
Object.assign(env, canvas_env);

window.addEventListener("load", () => {
    canvas = document.getElementById("screen");
    context = canvas.getContext("2d");
    fetch('./main.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {env: env}))
    .then(results => {
        wasm = results.instance;
        wasm.exports.init();
        wasm.exports.draw();
    });
});
