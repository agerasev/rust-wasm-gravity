use std::f64::consts::PI;

extern {
    fn js_canvas_size(ptr: *mut i32);

    fn js_canvas_fill_style(r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_stroke_style(r:f64,g:f64,b:f64,a:f64);
    fn js_canvas_line_width(w:f64);

    fn js_canvas_clear_rect(x:f64,y:f64,w:f64,h:f64);
    fn js_canvas_fill_rect(x:f64,y:f64,w:f64,h:f64);
    #[allow(dead_code)]
    fn js_canvas_stroke_rect(x:f64,y:f64,w:f64,h:f64);

    fn js_canvas_begin_path();
    fn js_canvas_close_path();
    fn js_canvas_fill();
    fn js_canvas_stroke();
    
    fn js_canvas_arc(x:f64,y:f64,r:f64,sa:f64,ea:f64);
    fn js_canvas_move_to(x:f64,y:f64);
    fn js_canvas_line_to(x:f64,y:f64);
    fn js_canvas_bezier_curve_to(x1:f64,y1:f64,x2:f64,y2:f64,x:f64,y:f64);
    fn js_canvas_quadratic_curve_to(x1:f64,y1:f64,x:f64,y:f64);
    fn js_canvas_ellipse(x:f64,y:f64,rx:f64,ry:f64,rot:f64,sa:f64,ea:f64);
    fn js_canvas_rect(x:f64,y:f64,w:f64,h:f64);
}

#[derive(Clone, Copy)]
pub struct Color(pub f64, pub f64, pub f64, pub f64);

pub struct Canvas {
    
}

pub enum Method {
    Fill {
        color: Color
    },
    Stroke {
        color: Color,
        width: f64
    },
}

pub enum Path {
    Arc {
        x: f64,
        y: f64,
        r: f64,
        sa: f64,
        ea: f64,
    },
    Circle {
        x: f64,
        y: f64,
        r: f64,
    },
    MoveTo {
        x: f64,
        y: f64,
    },
    LineTo {
        x: f64,
        y: f64,
    },
    BezierTo {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    QuadraticTo {
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    Ellipse {
        x: f64,
        y: f64,
        rx: f64,
        ry: f64,
        rot: f64,
        sa: f64,
        ea: f64,
    },
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    },
    Close,
    List {
        paths: Vec<Path>
    }
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { }
    }

    pub fn size(&self) -> (i32, i32) {
        let mut buf: [i32;2] = [0, 0];
        unsafe { js_canvas_size(buf.as_mut_ptr()); }
        (buf[0], buf[1])
    }

    pub fn clear(&mut self) {
        let (w, h) = self.size();
        unsafe {
            js_canvas_clear_rect(0.0, 0.0, w as f64, h as f64);
        }
    }
    pub fn fill(&mut self, c: Color) {
        let (w, h) = self.size();
        unsafe { 
            js_canvas_fill_style(c.0,c.1,c.2,c.3);
            js_canvas_fill_rect(0.0, 0.0, w as f64, h as f64);
        }
    }

    fn draw_path(&mut self, path: &Path) {
        match *path {
            Path::Arc {x, y, r, sa, ea}               => unsafe { js_canvas_arc(x, y, r, sa, ea); },
            Path::Circle {x, y, r}                    => unsafe { js_canvas_arc(x, y, r, 0.0, 2.0*PI); },
            Path::MoveTo {x, y}                       => unsafe { js_canvas_move_to(x, y); },
            Path::LineTo {x, y}                       => unsafe { js_canvas_line_to(x, y); },
            Path::BezierTo {x1, y1, x2, y2, x, y}     => unsafe { js_canvas_bezier_curve_to(x1, y1, x2, y2, x, y); },
            Path::Close {}                            => unsafe { js_canvas_close_path(); },
            Path::QuadraticTo {x1, y1, x, y}          => unsafe { js_canvas_quadratic_curve_to(x1, y1, x, y); },
            Path::Ellipse {x, y, rx, ry, rot, sa, ea} => unsafe { js_canvas_ellipse(x, y, rx, ry, rot, sa, ea); },
            Path::Rect {x, y, w, h}                   => unsafe { js_canvas_rect(x, y, w, h); },
            Path::List {ref paths} => {
                for subpath in paths.iter() {
                    self.draw_path(subpath);
                }
            },
        }
    }

    fn apply_method(&mut self, method: &Method) {
        unsafe {
            match *method {
                Method::Fill { color: c } => {
                    js_canvas_fill_style(c.0,c.1,c.2,c.3);
                    js_canvas_fill();
                },
                Method::Stroke { color: c, width: w } => {
                    js_canvas_stroke_style(c.0,c.1,c.2,c.3);
                    js_canvas_line_width(w);
                    js_canvas_stroke();
                }
            }
        }
    }

    pub fn draw(&mut self, path: &Path, method: &Method) {
        unsafe { js_canvas_begin_path(); }
        self.draw_path(path);
        self.apply_method(method);
    }
}
