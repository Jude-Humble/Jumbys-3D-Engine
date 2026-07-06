use macroquad::prelude::*;

pub trait Controllable {
    fn mov_obj(&mut self);
    //pub fn rot(&self);
}
