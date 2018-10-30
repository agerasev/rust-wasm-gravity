class MyModule {
    constructor() {
        this.screen = null;
        this.exports = {
            "set_screen": {
                "func": (cnv_id) => {
                    this.screen = OBJECTS[cnv_id];
                    let cnv = this.screen.canvas;
                    cnv.style.position = "fixed";
                    document.body.appendChild(cnv);
                    this.resize();
                },
                "args": ["u32"],
                "ret": "void",
            },
        };
    }
    init() {
        document.body.style.backgroundColor = "rgb(0,0,0)";
        window.addEventListener("resize", this.resize.bind(this));
    }
    resize() {
        if (this.screen != null) {
            let width = 
                window.innerWidth || 
                document.documentElement.clientWidth || 
                document.body.clientWidth;
            let height = 
                window.innerHeight ||
                document.documentElement.clientHeight ||
                document.body.clientHeight;

            this.screen.canvas.width = width;
            this.screen.canvas.height = height;

            console.log("[info] resize: " + width + " x " + height);
        }
    }
};

MODULES[document.currentScript._mod_name] = new MyModule();
