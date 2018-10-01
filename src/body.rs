use std::collections::VecDeque;

//use console;

use physsol::vec::*;
use physsol::point::*;
use physsol::rk4::*;

use wasm::canvas::*;

pub struct Curve {
    pts: [Vec2<f64>; 4],
    ort: [Vec2<f64>; 4],
    rad: [f64; 4],
    dt: f64,
}

impl Curve {
    fn new() -> Self {
        Curve {
            pts: [Vec2::<f64>::zero(); 4],
            ort: [Vec2::<f64>::zero(); 4],
            rad: [0.0; 4],
            dt: 0.0,
        }
    }

    fn from_points(p0: &Point2<f64>, p1: &Point2<f64>, dt: f64) -> Self {
        Curve {
            pts: [
                p0.pos,
                p0.pos - p0.vel*dt/3.0,
                p1.pos + p1.vel*dt/3.0,
                p1.pos,
            ],
            ort: [Vec2::<f64>::zero(); 4],
            rad: [0.0; 4],
            dt
        }
    }

    fn compute_ort(&mut self) {
        let left = |v: Vec2<f64>| Vec2::from(-v[1], v[0]);
        for j in 0..4 {
            self.ort[j] = left(self.vel_at((j as f64)/3.0)).normalize()
        }
    }

    fn left(&self, j: usize) -> Vec2<f64> {
        self.pts[j] + self.rad[j]*self.ort[j]
    }
    fn right(&self, j: usize) -> Vec2<f64> {
        self.pts[j] - self.rad[j]*self.ort[j]
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

pub struct Track {
    pub point: Point2<f64>,
    pub curve: Curve,
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

    tracks: VecDeque<Track>,
    last_step: f64,
}

impl Body {
    pub fn new(point: &Point2<f64>, mass: f64, color: Color, cfg: &BodyCfg) -> Self {
        Self {
            var: wrap(point.clone()), mass, color, rad: mass,
            tracks: VecDeque::with_capacity(cfg.track_len),
            last_step: -2.0*cfg.step_dur,
        }
    }

    pub fn step(&mut self, cfg: &BodyCfg, time: f64) {
        let mut dt = time - self.last_step;
        if dt > cfg.step_dur {
            if self.tracks.len() > 0 {
                let mut curve = Curve::from_points(&self.var.0, &self.tracks.back().unwrap().point, dt);
                curve.compute_ort();
                self.tracks.back_mut().unwrap().curve = curve;
            }
            self.tracks.push_back(Track { point: self.var.0.clone(), curve: Curve::new() } );
            self.last_step = time;
            dt = 0.0;
            if self.tracks.len() > cfg.track_len {
                self.tracks.pop_front();
            }
        }

        if self.tracks.len() > 0 {
            let mut curve = Curve::from_points(&self.var.0, &self.tracks.back().unwrap().point, dt);
            curve.compute_ort();
            self.tracks.back_mut().unwrap().curve = curve;
        }
        if self.tracks.len() >= cfg.track_len {
            let w = 1.0 - dt/cfg.step_dur;
            let full_curve = Curve::from_points(&self.tracks[1].point, &self.tracks[0].point, cfg.step_dur);
            let np = Point2 { pos: full_curve.pos_at(w), vel: full_curve.vel_at(w) };
            let mut curve = Curve::from_points(&self.tracks[1].point, &np, w*cfg.step_dur);
            curve.compute_ort();
            self.tracks[0].curve = curve;
        }

        let mut ct = 0.0;
        for track in self.tracks.iter_mut().rev() {
            for j in 0..4 {
                track.curve.rad[j] = self.rad*(1.0 - (ct + track.curve.dt*(j as f64)/3.0)/((cfg.track_len as f64)*cfg.step_dur));
            }
            ct += track.curve.dt;
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

    pub fn draw_track<F: FnMut(&Path, &Method)>(&mut self, mut func: F, _cfg: &BodyCfg, _time: f64) {
        if self.tracks.len() > 0 {
            let mut paths = Vec::<Path>::with_capacity(2*(self.tracks.len() + 2));

            paths.push(Path::MoveTo { pos: self.tracks[self.tracks.len() - 1].curve.left(0) });
            for track in self.tracks.iter().rev() {
                if track.curve.dt > 1e-8 {
                    paths.push(Path::BezierTo {
                        cp1: track.curve.left(1),
                        cp2: track.curve.left(2),
                        pos: track.curve.left(3),
                    });
                }
            }
            paths.push(Path::LineTo { pos: self.tracks[0].curve.right(3) });
            for track in self.tracks.iter() {
                if track.curve.dt > 1e-8 {
                    paths.push(Path::BezierTo {
                        cp1: track.curve.right(2),
                        cp2: track.curve.right(1),
                        pos: track.curve.right(0),
                    });
                }
            }
            paths.push(Path::Close);
        
            func(
                &Path::List { paths },
                &Method::Fill { color: self.color },
            );

            func(
                &Path::Circle {
                    pos: self.tracks[0].curve.pts[3],
                    rad: self.tracks[0].curve.rad[3],
                },
                &Method::Fill { color: self.color },
            );
        }
    }
}