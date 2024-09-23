use macroquad::prelude::*;

pub struct Bullet {
    pub pos: Vec2,
    pub dir: f32,
    pub speed: f32,
}

impl Drop for Bullet {
    fn drop(&mut self) {
        println!("The bullet at {} was dropped.", self.pos);
    }
}
