let wasm = null;

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

let import_env = (env, im_env, prefix) => {
    prefix = !prefix ? "" : prefix;
    for (var key in im_env) {
        if (im_env.hasOwnProperty(key)) {
            env[prefix + key] = im_env[key];
        }
    }
    return env;
};

let load_wasm = (path, env, onload) => {
    fetch(path)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {env: env}))
    .then(results => {
        onload(results.instance);
    });
};

window.addEventListener("load", () => {
    canvas_init();
    import_env(env, canvas_make_env(), "js_canvas_");

    load_wasm("./main.wasm", env, instance => {
        wasm = instance;
        wasm.exports.init();
        wasm.exports.draw();
    });
});
