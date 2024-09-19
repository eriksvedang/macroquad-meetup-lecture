use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub hp: u32,
}

impl Player {
    pub fn draw(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            self.hp as f32 * 0.2,
            BLACK,
        );
    }

    pub fn update(&mut self, input: Vec2, dt: f32) {
        self.position += input * self.speed * dt;
    }
}
