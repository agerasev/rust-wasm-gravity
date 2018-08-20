extern {
    fn js_canvas_get_width() -> i32;
    fn js_canvas_get_height() -> i32;

    fn js_canvas_clear();
    fn js_canvas_fill();
    
    fn js_canvas_set_fill_style(r: f32, g: f32, b: f32, a: f32);
    fn js_canvas_set_stroke_style(r: f32, g: f32, b: f32, a: f32);
    fn js_canvas_set_line_width(w: f32);
    
    fn js_canvas_path_begin();
    fn js_canvas_path_fill();
    fn js_canvas_path_stroke();

    fn js_canvas_draw_circle(x: f32, y: f32, r: f32);
}

#[derive(Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

pub struct Canvas {
    width: i32,
    height: i32
}

pub enum Method {
    Fill { color: Color },
    Stroke { color: Color, width: f32 },
}

pub enum Geometry {
    Circle { x: f32, y: f32, r: f32 }
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { width: 800, height: 600 }
    }

    pub fn _sync_size(&mut self) {
        unsafe {
            self.width = js_canvas_get_width();
            self.height = js_canvas_get_height();
        }
    }

    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn _set_method(&mut self, method: &Method) {
        unsafe {
            match *method {
                Method::Fill { color: c } => {
                    js_canvas_set_fill_style(c.0,c.1,c.2,c.3);
                },
                Method::Stroke { color: c, width: w } => {
                    js_canvas_set_stroke_style(c.0,c.1,c.2,c.3);
                    js_canvas_set_line_width(w);
                }
            }
        }
    }
    pub fn clear(&mut self) {
        unsafe {
            js_canvas_clear();
        }
    }
    pub fn fill(&mut self, c: Color) {
        unsafe { 
            js_canvas_set_fill_style(c.0,c.1,c.2,c.3);
            js_canvas_fill();
        }
    }
    pub fn draw(&mut self, geometry: &Geometry, method: &Method) {
        self._set_method(method);
        unsafe { 
            js_canvas_path_begin();
            match *geometry {
                Geometry::Circle { x, y, r } => {
                    js_canvas_draw_circle(x, y, r);
                }
            }
            match *method {
                Method::Fill { color: _ } => { js_canvas_path_fill(); },
                Method::Stroke { color: _, width: _ } => { js_canvas_path_stroke(); },
            }
        }
    }
}
