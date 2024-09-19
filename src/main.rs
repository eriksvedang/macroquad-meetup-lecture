use macroquad::prelude::*;

#[macroquad::main("Lecture")]
async fn main() {
    let player = Player {
        position: vec2(screen_width() * 0.5, screen_height() * 0.5),
        hp: 100,
    };

    loop {
        clear_background(WHITE);
        draw_circle(
            player.position.x,
            player.position.y,
            player.hp as f32 * 0.2,
            BLACK,
        );
        next_frame().await;
    }
}

struct Player {
    position: Vec2,
    hp: u32,
}
