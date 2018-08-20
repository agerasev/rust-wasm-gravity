let f4tos = (r,g,b,a) => "rgba(" + 255*r + "," + 255*g + "," + 255*b + "," + a + ")"

let canvas_env = {
    js_canvas_clear: () => {
        context.clearRect(0,0,canvas.width,canvas.height);
    },
    js_canvas_fill: () => {
        context.fillRect(0,0,canvas.width,canvas.height);
    },

    js_canvas_set_fill_style: (r,g,b,a) => {
        context.fillStyle = f4tos(r,g,b,a);
    },
    js_canvas_set_stroke_style: (r,g,b,a) => {
        context.strokeStyle = f4tos(r,g,b,a);
    },
    js_canvas_set_line_width: (w) => {
        context.lineWidth = w;
    },

    js_canvas_path_begin: () => {
        context.beginPath();
    },
    js_canvas_path_fill: () => {
        context.fill();
    },
    js_canvas_path_stroke: () => {
        context.stroke();
    },

    js_canvas_draw_circle: (x, y, r) => {
        context.arc(x, y, r, 0, 2*Math.PI);
    },
};
