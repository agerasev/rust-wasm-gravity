use vecmat::vec::*;

use console;
use math::{pow};
use canvas::{Canvas, Color, Path, Method};
use physics::*;

pub struct Body {
    var: Point2,
    mass: f64,
    color: Color,
}

impl<'a> Variable<'a> for Body {
    type VarIter = <Point2 as Variable<'a>>::VarIter;
    type VarState = Point2;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.var.var_iter()
    }
    fn var_clone(&'a self) -> Self::VarState {
        self.var.var_clone()
    }
}

pub struct System {
    bodies: Vec<Body>,
    deriv: Vec<Point2>,
    buf0: Vec<Point2>,
    buf1: Vec<Point2>,
}

impl System {
    fn new(bodies: Vec<Body>) -> System {
        let deriv = bodies.var_clone();
        let (buf0, buf1) = (bodies.var_clone(), bodies.var_clone());
        System { bodies, deriv, buf0, buf1 }
    }
}

pub struct App {
    time: f64,
    canvas: Canvas,
    system: System,
}

impl App {
    pub fn new() -> Self {
        let system = System::new(vec!(
            Body { 
                var: Point2 { pos: Vec2f64::from_arr([200.0, 300.0]), vel: Vec2f64::from_arr([0.0, 100.0]) },
                mass: 10.0, color: Color::from_arr([1.0, 0.0, 0.0, 1.0]),
            },
            Body { 
                var: Point2 { pos: Vec2f64::from_arr([600.0, 300.0]), vel: Vec2f64::from_arr([0.0,-100.0]) },
                mass: 10.0, color: Color::from_arr([0.0, 0.0, 1.0, 1.0]),
            },
        ));
        my_print!("App created!");
        App { time: 0.0, canvas: Canvas::new(), system }
    }

    pub fn gravity(bodies: &Vec<Body>, deriv: &mut Vec<Point2>) {
        for (i, (b, db)) in bodies.iter().zip(deriv.iter_mut()).enumerate() {
            db.pos = b.var.vel;
            db.vel = Vec2f64::zero();
            for (j, bo) in bodies.iter().enumerate() {
                if i != j {
                    let r = bo.var.pos - b.var.pos;
                    let g = r/(pow(r.sqr(), 1.5));
                    db.vel += g*2e7;
                }
            }
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.time += dt;

        let s = &mut self.system;

        solve_rk4(
            &mut s.bodies,
            &mut s.deriv,
            &mut s.buf0,
            &mut s.buf1,
            dt,
            Self::gravity
        );
    }

    pub fn draw(&mut self) {
        self.canvas.fill(Color::from_arr([1.0,1.0,1.0,0.01]));
        for body in &self.system.bodies {
            self.canvas.draw(
                &Path::Circle {
                    pos: body.var.pos,
                    rad: body.mass,
                },
                &Method::Fill { color: body.color },
            );
        }
    }
}
