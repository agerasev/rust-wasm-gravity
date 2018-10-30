use rand::{Rng, SeedableRng};
use rand::rngs::{SmallRng};

use physsol::vec::*;
use physsol::mat::*;
use physsol::map::*;
use physsol::point::*;
use physsol::rk4::*;

use wasm;
use wasm::console;
use wasm::canvas::*;
use wasm::interop::*;

use body::*;
use helper::*;

pub struct System {
    bodies: Vec<Body>,
    g: f64,
    body_cfg: BodyCfg,
}

static HELPER_PATH: &str = "./res/helper.js";

pub struct App {
    time: f64,
    helper: Option<Helper>,
    canvas: Canvas,
    system: System,
}

impl App {
    pub fn new() -> Self {
        let time = 0.0;
        let body_cfg = BodyCfg { track_len: 32, step_dur: 0.2 };

        let mut seed = [0 as u8; 16];
        wasm::seed(&mut seed[..]);
        let mut rng = SmallRng::from_seed(seed);

        let system = System { bodies: (0..64).map(|_| {
            Body::new(
                &Point2 { 
                    pos: Vec2::from(800.0*(rng.gen::<f64>() - 0.5), 800.0*(rng.gen::<f64>() - 0.5)),
                    vel: Vec2::from(200.0*(rng.gen::<f64>() - 0.5), 200.0*(rng.gen::<f64>() - 0.5))
                },
                10.0, 
                Color::from(rng.gen(), rng.gen(), rng.gen(), 1.0),
                &body_cfg,
            )
        }).collect(), g: 1e5, body_cfg };

        App { time, helper: None, canvas: Canvas::new(), system }
    }
    
    pub fn gravity(&mut self) {
        for i in 0..self.system.bodies.len() {
            let (left, right) = self.system.bodies.split_at_mut(i);
            let b0 = &mut right[0];
            b0.var.1.pos = b0.var.0.vel;
            b0.var.1.vel = Vec2::zero();
            for b1 in left {
                let r = b1.var.0.pos - b0.var.0.pos;
                let l = r.length();
                let s = 6.0*(b0.mass + b1.mass);
                let g = (r/l)*self.system.g/(l*l + s*s);
                b0.var.1.vel += g;
                b1.var.1.vel -= g;
            }
        }
    }

    fn step(&mut self, dt: f64) {
        //console::log(&format!("{}", dt));
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

    fn render(&mut self) {
        self.canvas.clear();
        let size = self.canvas.size();
        let center = 0.5*Vec2::from(size[0] as f64, size[1] as f64);
        self.canvas.transform(Affine2::from(Mat2::one(), center));
        for body in &mut self.system.bodies {
            let canvas = &mut self.canvas;
            body.draw(|p, m| canvas.draw(p, m), &self.system.body_cfg, self.time);
            body.draw_track(|p, m| canvas.draw(p, m), &self.system.body_cfg, self.time);
        }
    }
}

impl wasm::App for App {
    fn handle(&mut self, event: Event) {
        match event {
            Event::Start => {
                wasm::module::load(HELPER_PATH);
            },
            Event::Timeout { dt } => console::log(&format!("timeout {}", dt)),
            Event::Loaded { path, resource } => {
                let resource = resource.unwrap();
                console::log(&format!("resource loaded: '{}'", path));
                if path == HELPER_PATH {
                    let mut helper = Helper::new(match resource {
                        Resource::Module(module) => module,
                        //_ => panic!("wrong resource type {:?}", resource),
                    });
                    helper.set_screen(&self.canvas);
                    self.helper = Some(helper);
                    wasm::request_frame();
                } else {
                    console::error(&format!("unknown resource: {}", path));
                }
            },
            Event::Render { dt } =>  {
                self.step(dt);
                self.render();
                wasm::request_frame();
            }
        }
    }
}
