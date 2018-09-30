use std::collections::VecDeque;

//use console;

use physsol::vec::*;
use physsol::point::*;
use physsol::rk4::*;

use wasm::canvas::*;

struct Curve {
    pts: [Vec2<f64>; 4],
    ort: [Vec2<f64>; 4],
    dt: f64,
}

impl Curve {
    fn from_points(p0: &Point2<f64>, p1: &Point2<f64>, dt: f64) -> Self {
        let mut curve = Curve {
            pts: [
                p0.pos,
                p0.pos - p0.vel*dt/3.0,
                p1.pos + p1.vel*dt/3.0,
                p1.pos,
            ],
            ort: [Vec2::<f64>::default(); 4],
            dt
        };

        let left = |v: Vec2<f64>| Vec2::from(-v[1], v[0]);
        for j in 0..4 {
            curve.ort[j] = left(curve.vel_at((j as f64)/3.0)).normalize()
        }

        curve
    }

    fn left(&self, j: usize) -> Vec2<f64> {
        self.pts[j] + self.ort[j]
    }
    fn right(&self, j: usize) -> Vec2<f64> {
        self.pts[j] - self.ort[j]
    }

    fn pos_at(&self, w: f64) -> Vec2<f64> {
        let rw = 1.0 - w;
        (self.pts[0]*rw + self.pts[1]*3.0*w)*rw*rw + (self.pts[2]*3.0*rw + self.pts[3]*w)*w*w
    }
    fn vel_at(&self, w: f64) -> Vec2<f64> {
        let rw = 1.0 - w;
        ((self.pts[0]*rw + self.pts[1]*(2.0*w - rw))*rw - (self.pts[2]*(2.0*rw - w) + self.pts[3]*w)*w)*3.0/self.dt
    }
}

pub struct BodyCfg {
    pub track_len: usize,
    pub step_dur: f64,
}

pub struct Body {
    pub var: Wrap<Point2<f64>>,
    pub mass: f64,
    pub color: Color,
    pub rad: f64,

    track: VecDeque<Point2<f64>>,
    last_step: f64,
}

impl Body {
    pub fn new(point: &Point2<f64>, mass: f64, color: Color, cfg: &BodyCfg) -> Self {
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

    pub fn draw<F: FnMut(&Path, &Method)>(&mut self, mut func: F, _cfg: &BodyCfg, _time: f64) {
        func(
            &Path::Circle {
                pos: self.var.0.pos,
                rad: self.rad,
            },
            &Method::Fill { color: self.color },
        );
    }

    pub fn draw_track<F: FnMut(&Path, &Method)>(&mut self, mut func: F, cfg: &BodyCfg, time: f64) {
        let mut ct = 0.0;
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
                if w < 1e-8 {
                    break;
                }
                
                let curve = Curve::from_points(&lp, &p, cfg.step_dur);
                np.pos = curve.pos_at(w);
                np.vel = curve.vel_at(w);

                dt = w*cfg.step_dur;
            }

            let mut curve = Curve::from_points(&lp, &np, dt);

            for j in 0..4 {
                curve.ort[j] *= self.rad*(1.0 - (ct + curve.dt*(j as f64)/3.0)/((cfg.track_len as f64)*cfg.step_dur));
            }
            ct += curve.dt;

            curves.push(curve);
            lp = p.clone();
        }

        let mut paths = Vec::<Path>::new();
        if curves.len() > 0 {
            paths.push(Path::MoveTo { pos: curves[0].left(0) });
            for curve in curves.iter() {
                paths.push(Path::BezierTo {
                    cp1: curve.left(1),
                    cp2: curve.left(2),
                    pos: curve.left(3),
                });
            }
            paths.push(Path::LineTo { pos: curves[curves.len() - 1].right(3) });
            for curve in curves.iter().rev() {
                paths.push(Path::BezierTo {
                    cp1: curve.right(2),
                    cp2: curve.right(1),
                    pos: curve.right(0),
                });
            }
            paths.push(Path::Close);
        }

        func(
            &Path::List { paths },
            &Method::Fill { color: self.color },
        );
    }
}