use macroquad::prelude::*;

mod bullet;
mod player;

use bullet::Bullet;
use player::Player;

#[macroquad::main("Lecture")]
async fn main() {
    let mut player = Player {
        position: vec2(screen_width() * 0.5, screen_height() * 0.5),
        speed: 150.0,
        hp: 100,
    };

    let mut player2 = player;
    player2.speed = 50.0;

    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(WHITE);

        if let Some(bullet) = player.update(get_input(), get_frame_time()) {
            bullets.push(bullet);
        }

        player2.update(get_input(), get_frame_time());

        for bullet in &mut bullets {
            bullet.pos += Vec2::from_angle(bullet.dir) * get_frame_time() * bullet.speed;
            draw_circle(bullet.pos.x, bullet.pos.y, 10.0, RED);
        }

        bullets.retain(|bullet| bullet.pos.distance(player.position) < 300.0);

        player.draw();
        player2.draw();

        draw_text(&format!("P1 {:?}", player), 10.0, 32.0, 16.0, BLACK);
        draw_text(&format!("P2 {:?}", player2), 10.0, 64.0, 16.0, BLACK);

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
