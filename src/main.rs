use macroquad::prelude::*;

mod player;

#[macroquad::main("Lecture")]
async fn main() {
    let mut player = player::Player {
        position: vec2(screen_width() * 0.5, screen_height() * 0.5),
        speed: 150.0,
        hp: 100,
    };

    loop {
        clear_background(WHITE);
        player.update(get_input(), get_frame_time());
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
