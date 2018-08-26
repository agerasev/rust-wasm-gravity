use std::f64::consts::PI;

use vecmat::vec::*;

use console;
use math::{cos};
use canvas::{Canvas, Color, Path, Method};

pub struct App {
    time: f64,
    canvas: Canvas,
    //system: System,
}

impl App {
    pub fn new() -> Self {
        /*
        let mut system = System { bodies: vec!(
            Point { pos: Vec2f64::from_arr([200.0, 0.0]), vel: Vec2f64::from_arr([0.0, 10.0]) },
            Point { pos: Vec2f64::from_arr([400.0, 0.0]), vel: Vec2f64::from_arr([0.0,-10.0]) },
        ) };
        */
        my_print!("App created!");
        App { time: 0.0, canvas: Canvas::new() }//, system }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;
    }

    pub fn draw(&mut self) {
        self.canvas.clear();
        let mut rad = Vec2f64::from_scal(1.5) + Vec2f64::from_arr([1.0, -1.0])*cos(3.0*self.time);
        rad *= 120.0/(rad[0]*rad[1]).sqrt();
        self.canvas.draw(
            &Path::Ellipse {
                pos: Vec2f64::from_arr([400.0, 300.0]),
                rad, //: Vec2f64::from_arr([160.0, 100.0*(2.0 + cos(self.time))]),
                rot: 2.0*PI*self.time/19.0,
                angle: Vec2f64::from_arr([0.0, 2.0*PI]),
            },
            &Method::Fill { color: Color::from_arr([0.5, 0.2, 1.0, 1.0]) },
        );
    }
}
