let mkcol = (r,g,b,a) => "rgba(" + 255*r + "," + 255*g + "," + 255*b + "," + a + ")"

let canvas = null;
let context = null;

let canvas_init = () => {
    canvas = document.getElementById("screen");
    context = canvas.getContext("2d");
};

let canvas_make_env = () => {
    let canvas_env = {
        size: (ptr) => {
            let view = new Uint32Array(wasm.exports.memory.buffer, ptr, 2);
            view[0] = canvas.width;
            view[1] = canvas.height;
        },

        fill_style: (r,g,b,a) => {
            context.fillStyle = mkcol(r,g,b,a);
        },
        stroke_style: (r,g,b,a) => {
            context.strokeStyle = mkcol(r,g,b,a);
        },
        line_width: (w) => {
            context.lineWidth = w;
        },

        clear_rect: (x,y,w,h) => {
            context.clearRect(x,y,w,h);
        },
        fill_rect: (x,y,w,h) => {
            context.fillRect(x,y,w,h);
        },
        stroke_rect: (x,y,w,h) => {
            context.strokeRect(x,y,w,h);
        },

        begin_path: () => {
            context.beginPath();
        },
        close_path: () => {
            context.closePath();
        },
        fill: () => {
            context.fill();
        },
        stroke: () => {
            context.stroke();
        },
        
        arc: (x,y,r,sa,ea) => {
            context.arc(x,y,r,sa,ea);
        },
        move_to: (x,y) => {
            context.moveTo(x,y);
        },
        line_to: (x,y) => {
            context.lineTo(x,y);
        },
        bezier_curve_to: (x1,y1,x2,y2,x,y) => {
            context.bezierCurveTo(x1,y1,x2,y2,x,y);
        },
        quadratic_curve_to: (x1,y1,x,y) => {
            context.quadraticCurveTo(x1,y1,x,y);
        },
        ellipse: (x,y,rx,ry,rot,sa,ea) => {
            context.ellipse(x,y,rx,ry,rot,sa,ea,0);
        },
        rect: (x,y,w,h) => {
            context.rect(x,y,w,h);
        },
    };
    return canvas_env;
};
