use std::collections::VecDeque;

//use console;

use physsol::vec::*;
use physsol::point::*;
use physsol::rk4::*;

use canvas::*;

struct Curve {
    pts: [Vec2f64; 4],
    dt: f64,
}

impl Curve {
    fn from_points(p0: &Point2, p1: &Point2, dt: f64) -> Self {
        Curve { pts: [
            p0.pos,
            p0.pos - p0.vel*dt/3.0,
            p1.pos + p1.vel*dt/3.0,
            p1.pos,
        ], dt }
    }

    fn pos_at(&self, w: f64) -> Vec2f64 {
        let rw = 1.0 - w;
        (self.pts[0]*rw + self.pts[1]*3.0*w)*rw*rw + (self.pts[2]*3.0*rw + self.pts[3]*w)*w*w
    }

    fn vel_at(&self, w: f64) -> Vec2f64 {
        let rw = 1.0 - w;
        ((self.pts[0]*rw + self.pts[1]*(2.0*w - rw))*rw - (self.pts[2]*(2.0*rw - w) + self.pts[3]*w)*w)*3.0/self.dt
    }
}

pub struct BodyCfg {
    pub track_len: usize,
    pub step_dur: f64,
}

pub struct Body {
    pub var: Wrap<Point2>,
    pub mass: f64,
    pub color: Color,
    pub rad: f64,

    track: VecDeque<Point2>,
    last_step: f64,
}

impl Body {
    pub fn new(point: &Point2, mass: f64, color: Color, cfg: &BodyCfg) -> Self {
        Self {
            var: wrap(point.clone()), mass, color, rad: mass,
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
        
        let mut curves = Vec::<Curve>::new();
        let mut lp = self.var.0.clone();
        for (i, p) in self.track.iter().rev().enumerate() {

            let mut dt = if i == 0 {
                time - self.last_step
            } else {
                cfg.step_dur
            };

            if dt.abs() < 1e-8 {
                continue;
            }

            let mut np = p.clone();

            if i == cfg.track_len - 1 {
                let w = 1.0 - (time - self.last_step)/cfg.step_dur;
                
                let curve = Curve::from_points(&lp, &p, cfg.step_dur);
                np.pos = curve.pos_at(w);
                np.vel = curve.vel_at(w);

                dt = w*cfg.step_dur;
            }

            let curve = Curve::from_points(&lp, &np, dt);
            curves.push(curve);
            lp = p.clone();
        }

        let mut paths = Vec::<Path>::new();
        if curves.len() > 0 {
            let left = |v: Vec2f64| Vec2f64::from_arr([-v[1], v[0]]);
            paths.push(Path::MoveTo { pos: curves[0].pts[0] + left(curves[0].vel_at(0.0).normalize())*self.rad });
            let mut ct = 0.0;
            for curve in curves.iter() {
                let mut pts = curve.pts.clone();
                for j in 1..4 {
                    let r = self.rad*(1.0 - (ct + curve.dt*(j as f64)/3.0)/((cfg.track_len as f64)*cfg.step_dur));
                    pts[j] += left(curve.vel_at((j as f64)/3.0).normalize())*r;
                }
                paths.push(Path::BezierTo {
                    cp1: pts[1],
                    cp2: pts[2],
                    pos: pts[3],
                });
                ct += curve.dt;
            }
            ct = 0.0;
            for curve in curves.iter().rev() {
                let mut pts = curve.pts.clone();
                for j in 0..3 {
                    let r = self.rad*((ct + curve.dt*((2-j) as f64)/3.0)/((cfg.track_len as f64)*cfg.step_dur));
                    pts[j] -= left(curve.vel_at((j as f64)/3.0).normalize())*r;
                }
                paths.push(Path::BezierTo {
                    cp1: pts[2],
                    cp2: pts[1],
                    pos: pts[0],
                });
                ct += curve.dt;
            }
            paths.push(Path::Close);
        }

        out.push((
            Path::List { paths },
            Method::Fill { color: self.color },
        ));

        out.push((
            Path::Circle {
                pos: self.var.0.pos,
                rad: self.mass,
            },
            Method::Fill { color: self.color },
        ));
        
        out
    }
}