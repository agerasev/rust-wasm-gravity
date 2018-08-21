use console;
use canvas::{Canvas, Color, Path, Method};
use std::f64::consts::PI;

pub struct App {
    canvas: Canvas,
}

impl App {
    pub fn new() -> Self {
        my_print!("App created!");
        App { canvas: Canvas::new() }
    }

    pub fn draw(&mut self) {
        self.canvas.draw(
            &Path::Ellipse { x: 400.0, y: 300.0, rx: 100.0, ry: 150.0, rot: PI/4.0, sa: 0.0, ea: 2.0*PI },
            &Method::Fill { color: Color(0.5, 0.0, 1.0, 1.0) }
        );
    }
}
