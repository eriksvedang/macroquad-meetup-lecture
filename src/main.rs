use macroquad::prelude::*;

mod bullet;
mod player;

use bullet::Bullet;

#[macroquad::main("Lecture")]
async fn main() {
    let mut player = player::Player {
        position: vec2(screen_width() * 0.5, screen_height() * 0.5),
        speed: 150.0,
        hp: 100,
    };

    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(WHITE);
        player.update(get_input(), get_frame_time());

        if is_mouse_button_pressed(MouseButton::Left) {
            bullets.push(Bullet {
                pos: player.position,
                dir: 0.0,
                speed: 200.0,
            })
        }

        for bullet in &mut bullets {
            bullet.pos += Vec2::from_angle(bullet.dir) * get_frame_time() * bullet.speed;
            draw_circle(bullet.pos.x, bullet.pos.y, 10.0, RED);
        }

        bullets.retain(|bullet| bullet.pos.x < screen_width() - 50.0);

        player.draw();
        next_frame().await;
    }
}

fn get_input() -> Vec2 {
    let mut x = 0.0;
    let mut y = 0.0;

    if is_key_down(KeyCode::A) {
        x -= 1.0;
    }

    if is_key_down(KeyCode::D) {
        x += 1.0;
    }

    if is_key_down(KeyCode::W) {
        y -= 1.0;
    }

    if is_key_down(KeyCode::S) {
        y += 1.0;
    }

    vec2(x, y).clamp_length(0.0, 1.0)
}
