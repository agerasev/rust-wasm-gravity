//use std::iter::{Iterator, Chain, IntoIterator, FlatMap};
use vecmat::vec::*;

#[derive(Clone)]
pub struct Point2 {
    pub pos: Vec2f64,
    pub vel: Vec2f64,
}

pub type EulerWrap<T> = (T, T);
pub fn euler_wrap<T>(t: T) -> EulerWrap<T> where T: Clone {
    (t.clone(), t)
}
pub fn euler_wrap_ref<T>(t: &T) -> EulerWrap<T> where T: Clone {
    (t.clone(), t.clone())
}

pub trait EulerVar {
    fn step(&mut self, dt: f64);
}

impl<'a> EulerVar for EulerWrap<&'a mut f64> {
    fn step(&mut self, dt: f64) {
        *self.0 += *self.1*dt;
    }
}
impl EulerVar for EulerWrap<f64> {
    fn step(&mut self, dt: f64) {
        self.0 += self.1*dt;
    }
}
impl<'a> EulerVar for EulerWrap<&'a mut Vec2f64> {
    fn step(&mut self, dt: f64) {
        for i in 0..2 {
            unsafe { 
                (self.0.d.get_unchecked_mut(i), self.1.d.get_unchecked_mut(i)).step(dt)
            }
        }
    }
}
impl EulerVar for EulerWrap<Vec2f64> {
    fn step(&mut self, dt: f64) {
        (&mut self.0, &mut self.1).step(dt);
    }
}
impl<'a> EulerVar for EulerWrap<&'a mut Point2> {
    fn step(&mut self, dt: f64) {
        (&mut self.0.pos, &mut self.1.pos).step(dt);
        (&mut self.0.vel, &mut self.1.vel).step(dt);
    }
}
impl<'a> EulerVar for EulerWrap<Point2> {
    fn step(&mut self, dt: f64) {
        (&mut self.0, &mut self.1).step(dt);
    }
}

pub fn euler_solve<F, T>(mut fn_step: F, dt: f64) where F: FnMut(fn(&mut T, f64), f64), T: EulerVar {
    fn_step(|v, dt| v.step(dt), dt);
}


pub type RK4Wrap<T> = (T, T, T, T);
pub fn rk4_wrap<T>(t: T) -> RK4Wrap<T> where T: Clone {
    (t.clone(), t.clone(), t.clone(), t)
}
pub fn rk4_wrap_ref<T>(t: &T) -> RK4Wrap<T> where T: Clone {
    (t.clone(), t.clone(), t.clone(), t.clone())
}

pub trait RK4Var {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64));

    fn step_0(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.3 = *v.0;
            *v.2 = *v.1;
            *v.0 = *v.3 + *v.1*dt*0.5;
        });
    }
    fn step_1(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1*2.0;
            *v.0 = *v.3 + *v.1*dt*0.5;
        });
    }
    fn step_2(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1*2.0;
            *v.0 = *v.3 + *v.1*dt;
        });
    }
    fn step_3(&mut self, dt: f64) {
        self.step(dt, |v, dt| {
            *v.2 += *v.1;
            *v.0 = *v.3 + *v.2*dt/6.0;
        });
    }
}

impl<'a> RK4Var for RK4Wrap<&'a mut f64> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        f(self, dt);
    }
}
impl RK4Var for RK4Wrap<f64> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        f(&mut (&mut self.0, &mut self.1, &mut self.2, &mut self.3), dt);
    }
}
impl<'a> RK4Var for RK4Wrap<&'a mut Vec2f64> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        for i in 0..2 {
            unsafe { (
                self.0.d.get_unchecked_mut(i),
                self.1.d.get_unchecked_mut(i),
                self.2.d.get_unchecked_mut(i),
                self.3.d.get_unchecked_mut(i),
            ).step(dt, f) }
        }
    }
}
impl RK4Var for RK4Wrap<Vec2f64> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
    }
}
impl<'a> RK4Var for RK4Wrap<&'a mut Point2> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        (&mut self.0.pos, &mut self.1.pos, &mut self.2.pos, &mut self.3.pos).step(dt, f);
        (&mut self.0.vel, &mut self.1.vel, &mut self.2.vel, &mut self.3.vel).step(dt, f);
    }
}
impl<'a> RK4Var for RK4Wrap<Point2> {
    fn step(&mut self, dt: f64, f: fn(&mut RK4Wrap<&mut f64>, f64)) {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3).step(dt, f);
    }
}

pub fn rk4_solve<F, T>(mut fn_step: F, dt: f64) where F: FnMut(fn(&mut T, f64), f64), T: RK4Var {
    fn_step(|v, dt| v.step_0(dt), dt);
    fn_step(|v, dt| v.step_1(dt), dt);
    fn_step(|v, dt| v.step_2(dt), dt);
    fn_step(|v, dt| v.step_3(dt), dt);
}