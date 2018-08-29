use physsol::vec::*;

use console;
use math::{pow};
use canvas::{Canvas, Color, Path, Method};
use physsol::point::*;
use physsol::rk4::*;

pub struct Body {
    var: Wrap<Point2>,
    mass: f64,
    color: Color,
}

pub struct System {
    bodies: Vec<Body>,
    g: f64
}

pub struct App {
    time: f64,
    canvas: Canvas,
    system: System,
}

impl App {
    pub fn new() -> Self {
        let system = System { g: 2e7, bodies: vec!(
            Body { 
                var: wrap(Point2 { pos: Vec2f64::from_arr([200.0, 300.0]), vel: Vec2f64::from_arr([0.0, 100.0]) }),
                mass: 10.0, color: Color::from_arr([1.0, 0.0, 0.0, 1.0]),
            },
            Body {
                var: wrap(Point2 { pos: Vec2f64::from_arr([600.0, 300.0]), vel: Vec2f64::from_arr([0.0,-100.0]) }),
                mass: 10.0, color: Color::from_arr([0.0, 0.0, 1.0, 1.0]),
            },
        ) };
        my_print!("App created!");
        App { time: 0.0, canvas: Canvas::new(), system }
    }

    pub fn gravity(&mut self) {
        for i in 0..self.system.bodies.len() {
            let (left, right) = self.system.bodies.split_at_mut(i);
            let b0 = &mut right[0];
            b0.var.1.pos = b0.var.0.vel;
            b0.var.1.vel = Vec2f64::zero();
            for b1 in left {
                let r = b1.var.0.pos - b0.var.0.pos;
                let g = r*self.system.g/(pow(r.sqr(), 1.5));
                b0.var.1.vel += g;
                b1.var.1.vel -= g;
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;
        solve(|f, dt| {
            self.gravity();
            for b in &mut self.system.bodies {
                f(&mut b.var, dt)
            }
        }, dt);
    }

    pub fn draw(&mut self) {
        self.canvas.fill(Color::from_arr([1.0,1.0,1.0,0.01]));
        for body in &self.system.bodies {
            self.canvas.draw(
                &Path::Circle {
                    pos: body.var.0.pos,
                    rad: body.mass,
                },
                &Method::Fill { color: body.color },
            );
        }
    }
}
