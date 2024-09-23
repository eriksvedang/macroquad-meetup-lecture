use macroquad::prelude::*;

use crate::bullet::Bullet;

#[derive(Clone, Copy, Debug)]
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

    pub fn update(&mut self, input: Vec2, dt: f32) -> Option<Bullet> {
        self.position += input * self.speed * dt;

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();
            Some(Bullet {
                pos: self.position,
                dir: (mouse_pos - self.position).to_angle(),
                speed: 200.0,
            })
        } else {
            None
        }
    }
}

// impl Clone for Player {
//     fn clone(&self) -> Self {
//         Player {
//             position: self.position, // no need to clone here, the type is Copy:able
//             speed: self.speed,
//             hp: self.hp,
//         }
//     }
// }
