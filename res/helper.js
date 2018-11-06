class HelperModule {
    constructor() {
        this.screen = null;

        this.pannel = null;
        this.pansize = 0.4;
        this.button = null;
        this.pause = false;

        this.codes = {
            "pause": 0x01,
            "resize": 0x02,
        };

        this.exports = {
            "set_screen": {
                "func": (cnv_id) => {
                    if (this.screen != null) {
                        this.screen.canvas.remove();
                    }
                    this.screen = OBJECTS[cnv_id];
                    let cnv = this.screen.canvas;
                    cnv.style.position = "fixed";
                    cnv.style.zIndex = 0;
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

        let pannel = document.createElement("div");
        Object.assign(pannel.style, {
            "display": "none",
            "position": "fixed",
            "backgroundColor": "rgba(255,255,255,0.5)",
            "zIndex": 1,
        });

        let hide_button = document.createElement("button");
        hide_button.innerText = "Hide pannel";
        hide_button.style.float = "right";
        hide_button.addEventListener("click", () => {
            this.button.style.display = "block";
            this.pannel.style.display = "none";
        });
        pannel.appendChild(hide_button);

        this.pannel = pannel;
        document.body.appendChild(this.pannel);

        let button = document.createElement("button");
        button.innerText = "Show pannel";
        Object.assign(button.style, {
            "display": "block",
            "position": "fixed",
            "zIndex": 1,
        });
        button.addEventListener("click", () => {
            this.button.style.display = "none";
            this.pannel.style.display = "block";
        });
        this.button = button;
        document.body.appendChild(this.button);
        
        let pause_button = document.createElement("button");
        pause_button.innerText = "Pause";
        pause_button.addEventListener("click", () => {
            pause_button.innerText = this.pause ? "Pause" : "Resume";
            this.pause = !this.pause;
            handle(EVENT.USER, [this.codes["pause"], this.pause], ["u32", "i32"]);
        });
        pannel.appendChild(pause_button);

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

            this.pannel.style.width = this.pansize*width + "px";
            this.pannel.style.height = height + "px";
            this.pannel.style.right = "0px";

            this.button.style.right = "0px";

            handle(EVENT.USER, [this.codes["resize"]], ["u32"]);

            console.log("[info] resize: " + width + " x " + height);
        }
    }
};

MODULES[document.currentScript._mod_id] = new HelperModule();
