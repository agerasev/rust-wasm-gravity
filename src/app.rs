use std::f64::consts::PI;

use vecmat::vec::*;

use console;
use canvas::{Canvas, Color, Path, Method};

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
            &Path::Ellipse {
                pos: Vec2f64::new_array([400.0, 300.0]),
                rad: Vec2f64::new_array([100.0, 150.0]),
                rot: PI/4.0,
                angle: Vec2f64::new_array([0.0, 2.0*PI]),
            },
            &Method::Fill { color: Color::new_array([0.5, 0.0, 1.0, 1.0]) },
        );
    }
}
