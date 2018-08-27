use std::iter::{Iterator, Chain, IntoIterator, FlatMap};
use vecmat::vec::*;

pub trait Variable<'a> {
    type VarIter: Iterator<Item=&'a mut f64>;
    type VarState: Variable<'a>;
    fn var_iter(&'a mut self) -> Self::VarIter;
    fn var_clone(&'a self) -> Self::VarState;
}

pub fn solve_euler<'a, V, F> (
    y: &'a mut V,
    dy: &'a mut V::VarState,
    dt: f64,
    mut f: F,
) where 
    V: Variable<'a>, 
    F: FnMut(&V, &mut V::VarState) 
{
    f(y, dy);
    for (x, dx) in y.var_iter().zip(dy.var_iter()) {
        *x += *dx*dt;
    }
}

pub fn solve_rk4<'a, V, F> (
    y: &'a mut V,
    dy: &'a mut V::VarState,
    _y1: &'a mut V::VarState,
    _y2: &'a mut V::VarState,
    dt: f64,
    mut f: F,
) where 
    V: Variable<'a>, 
    F: FnMut(&V, &mut V::VarState) 
{
    f(y, dy);
    for (x, dx) in y.var_iter().zip(dy.var_iter()) {
        *x += *dx*dt;
    }
}

impl<'a> Variable<'a> for Vec2f64 {
    type VarIter = <&'a mut Vec2f64 as IntoIterator>::IntoIter;
    type VarState = Self;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.iter_mut()
    }
    fn var_clone(&'a self) -> Self::VarState {
        self.clone()
    }
}

#[derive(Clone)]
pub struct Point2 {
    pub pos: Vec2f64,
    pub vel: Vec2f64,
}

impl<'a> Variable<'a> for Point2 {
    type VarIter = Chain<<Vec2f64 as Variable<'a>>::VarIter, <Vec2f64 as Variable<'a>>::VarIter>;
    type VarState = Self;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.pos.var_iter().chain(self.vel.var_iter())
    }
    fn var_clone(&'a self) -> Self::VarState {
        self.clone()
    }
}

impl<'a, V: 'a> Variable<'a> for Vec<V> where V: Variable<'a> {
    type VarIter = FlatMap<<&'a mut Self as IntoIterator>::IntoIter, V::VarIter, fn(&'a mut V) -> V::VarIter>;
    type VarState = Vec<V::VarState>;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.into_iter().flat_map(|v| v.var_iter())
    }
    fn var_clone(&'a self) -> Self::VarState {
        self.into_iter().map(|v| v.var_clone()).collect()
    }
}
