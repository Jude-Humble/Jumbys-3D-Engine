use macroquad::prelude::*;

pub trait Controllable {
    fn mov_obj(&mut self);
    fn rot(&mut self);
}
