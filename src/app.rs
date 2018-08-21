use std::f64::consts::PI;

use vecmat::vec::*;

use console;
use canvas::{Canvas, Color, Path, Method};

pub struct App {
    time: f64,
    canvas: Canvas,
}

impl App {
    pub fn new() -> Self {
        my_print!("App created!");
        App { time: 0.0, canvas: Canvas::new() }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;
    }

    pub fn draw(&mut self) {
        self.canvas.clear();
        self.canvas.draw(
            &Path::Ellipse {
                pos: Vec2f64::new_array([400.0, 300.0]),
                rad: Vec2f64::new_array([160.0, 200.0]),
                rot: 2.0*PI*self.time,
                angle: Vec2f64::new_array([0.0, 2.0*PI]),
            },
            &Method::Fill { color: Color::new_array([0.5, 0.0, 1.0, 1.0]) },
        );
    }
}
