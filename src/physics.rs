use std::iter::{Iterator, Chain, IntoIterator, Flatten};
use vecmat::vec::*;

pub trait Variable<'a> {
    type VarIter: Iterator<Item=&'a mut f64>;
    fn var_iter(&'a mut self) -> Self::VarIter;
}

pub fn solve_euler<'a, V> (y: &'a mut V, dy: &'a mut V, dt: f64) where V: Variable<'a> {
    for (x, dx) in y.var_iter().zip(dy.var_iter()) {
        *x += *dx*dt;
    }
}

impl<'a> Variable<'a> for Vec2f64 {
    type VarIter = <&'a mut Vec2f64 as IntoIterator>::IntoIter;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.iter_mut()
    }
}

#[derive(Clone)]
pub struct Point2 {
    pos: Vec2f64,
    vel: Vec2f64,
}

impl<'a> Variable<'a> for Point2 {
    type VarIter = Chain<<Vec2f64 as Variable<'a>>::VarIter, <Vec2f64 as Variable<'a>>::VarIter>;
    fn var_iter(&'a mut self) -> Self::VarIter {
        self.pos.var_iter().chain(self.vel.var_iter())
    }
}
/*
impl<'a, II> Variable<'a> for IntoIterator<Item=&'a mut Variable<'a>, IntoIter=II> {
    type VarIter = FlatMap<>
    fn var_iter(&'a mut self) -> Self::VarIter {

    }
}
*/
/*
pub struct FlatVarIter<'a, O, V: 'a> where O: Iterator<Item=&'a mut V>, V: Variable<'a> {
    outer: O,
    var: Option<V::VarIter>,
}

impl<'a, O, V> FlatVarIter<'a, O, V> where O: Iterator<Item=&'a mut V>, V: Variable<'a> {
    pub fn new(mut outer: O) -> Self {
        let var = match outer.next() {
            Some(v) => {
                Some(v.var_iter())
            },
            None => None,
        };
        Self {
            outer,
            var,
        }
    }

    fn step_outer(&mut self) {

    }
}

impl<'a, O, V: 'a> Iterator for FlatVarIter<'a, O, V> where O: Iterator<Item=&'a mut V>, V: Variable<'a> {
    type Item = &'a mut f64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.var {
            Some(mut v) => {
                match v.next() {
                    Some(x) => Some(x),
                    None => {
                        self.var = match self.outer.next() {
                            Some(v) => Some(v.var_iter()),
                            None => None
                        };
                        self.next()
                    }
                }
            },
            None => None
        }
    }
}
*/
/*
impl<'a, V: 'a> Variable<'a> for Vec<V> where V: Variable<'a> {
    type VarIter = VecVarIter<'a, V>;
    fn var_iter(&mut self) -> Self::VarIter {

    }
}
*/