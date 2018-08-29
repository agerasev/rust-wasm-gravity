use std::collections::VecDeque;

//use console;

use physsol::vec::*;
use physsol::point::*;
use physsol::rk4::*;

use canvas::*;

pub struct BodyCfg {
    pub track_len: usize,
    pub step_dur: f64,
}

pub struct Body {
    pub var: Wrap<Point2>,
    pub mass: f64,
    pub color: Color,

    pub track: VecDeque<Point2>,
    pub last_step: f64,
}

fn bezier_par(p0: &Point2, p1: &Point2, dt: f64) -> [Vec2f64; 4] {
    [
        p0.pos,
        p0.pos - p0.vel*dt/3.0,
        p1.pos + p1.vel*dt/3.0,
        p1.pos,
    ]
}

impl Body {
    pub fn new(point: &Point2, mass: f64, color: Color, cfg: &BodyCfg) -> Self {
        Self {
            var: wrap(point.clone()), mass, color,
            track: VecDeque::with_capacity(cfg.track_len),
            last_step: -2.0*cfg.step_dur,
        }
    }

    pub fn step(&mut self, cfg: &BodyCfg, time: f64) {
        if time - self.last_step > cfg.step_dur {
            self.track.push_back(self.var.0.clone());
            self.last_step = time;
            if self.track.len() > cfg.track_len {
                self.track.pop_front();
            }
        }
    }

    pub fn draw(&mut self, cfg: &BodyCfg, time: f64) -> Vec<(Path, Method)> {
        let mut out = Vec::new();

        out.push((
            Path::Circle {
                pos: self.var.0.pos,
                rad: self.mass,
            },
            Method::Fill { color: self.color },
        ));
        
        let mut paths = Vec::<Path>::new();
        paths.push(Path::MoveTo { pos: self.var.0.pos } );
        
        let mut lp = self.var.0.clone();
        for (i, p) in self.track.iter().rev().enumerate() {

            let mut dt = if i == 0 {
                time - self.last_step
            } else {
                cfg.step_dur
            };

            let mut np = p.clone();

            if i == cfg.track_len - 1 {
                let w = (time - self.last_step)/cfg.step_dur;
                let rw = 1.0 - w;
                dt = rw*cfg.step_dur;
                
                let par = bezier_par(&lp, &p, cfg.step_dur);
                np.pos = par[0]*w*w*w + par[1]*3.0*w*w*rw + par[2]*3.0*w*rw*rw + par[3]*rw*rw*rw;
                np.vel = ((par[0]*w + par[1]*(2.0*rw - w))*w - (par[2]*(2.0*w - rw) + par[3]*rw)*rw)*3.0/cfg.step_dur;
            }

            let par = bezier_par(&lp, &np, dt);
            paths.push(Path::BezierTo {
                cp1: par[1],
                cp2: par[2],
                pos: par[3],
            });
            lp = p.clone();
        }


        out.push((
            Path::List { paths },
            Method::Stroke { color: self.color, width: 2.0 },
        ));
        
        out
    }
}