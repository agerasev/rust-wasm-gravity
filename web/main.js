let wasm = null;
let done = false;
let last = null;

let load_str = (ptr, len) => {
    const view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
    //const utf8dec = new TextDecoder("utf-8");
    //return utf8dec.decode(view);
    let str = "";
    for (let i = 0; i < view.length; i++) {
        str += String.fromCharCode(view[i]);
    }
    return str;
}

let env = {
    js_console: (type, ptr, len) => {
        let str = load_str(ptr, len);
        if (type == 1) {
            console.error(str);
        } else {
            console.log(str);
        }
    },
    js_timeout: (sec) => {
        setTimeout(() => {
            wasm.exports.timeout(parseFloat(sec));
        }, 1000*sec);
    }
};

let render = () => {
    let now = +new Date();
    let ms = now - last;
    last = now;
    wasm.exports.render(parseFloat(0.001*ms));
    if (!done) {
        window.requestAnimationFrame(render);
    }
};

let import_env = (env, im_env, prefix) => {
    prefix = !prefix ? "" : prefix;
    for (let key in im_env) {
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

    document.getElementById("stop").addEventListener("click", () => {
        console.log("stop");
        done = true;
    });

    load_wasm("./main.wasm", env, instance => {
        wasm = instance;
        wasm.exports.init();

        last = +new Date();
        window.requestAnimationFrame(render);
    });

});
