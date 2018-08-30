use console;

use physsol::vec::*;
use physsol::point::*;
use physsol::rk4::*;

use canvas::*;
use body::*;

pub struct System {
    bodies: Vec<Body>,
    g: f64,
    body_cfg: BodyCfg,
}

pub struct App {
    time: f64,
    canvas: Canvas,
    system: System,
}

impl App {
    pub fn new() -> Self {
        let time = 0.0;
        let body_cfg = BodyCfg { track_len: 8, step_dur: 0.2 };
        let system = System { bodies: vec!(
            Body::new(
                &Point2 { pos: Vec2f64::from_arr([200.0, 300.0]), vel: Vec2f64::from_arr([0.0, 100.0]) },
                10.0, Color::from_arr([1.0, 0.0, 0.0, 1.0]),
                &body_cfg,
            ),
            Body::new(
                &Point2 { pos: Vec2f64::from_arr([600.0, 300.0]), vel: Vec2f64::from_arr([0.0,-100.0]) },
                10.0, Color::from_arr([0.0, 0.0, 1.0, 1.0]),
                &body_cfg,
            ),
            Body::new(
                &Point2 { pos: Vec2f64::from_arr([400.0, 200.0]), vel: Vec2f64::from_arr([0.0, 0.0]) },
                10.0, Color::from_arr([0.0, 1.0, 0.0, 1.0]),
                &body_cfg,
            ),
        ), g: 2e7, body_cfg };
        my_print!("App created!");
        App { time, canvas: Canvas::new(), system }
    }

    pub fn gravity(&mut self) {
        for i in 0..self.system.bodies.len() {
            let (left, right) = self.system.bodies.split_at_mut(i);
            let b0 = &mut right[0];
            b0.var.1.pos = b0.var.0.vel;
            b0.var.1.vel = Vec2f64::zero();
            for b1 in left {
                let r = b1.var.0.pos - b0.var.0.pos;
                let l = r.length();
                let s = 4.0*(b0.mass + b1.mass);
                let g = (r/l)*self.system.g/(l*l + s*s);
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
        for b in &mut self.system.bodies {
            b.step(&self.system.body_cfg, self.time);
        }
    }

    pub fn draw(&mut self) {
        self.canvas.clear();
        for body in &mut self.system.bodies {
            for (path, method) in body.draw(&self.system.body_cfg, self.time) {
                self.canvas.draw(&path, &method);
            }
        }
    }
}
