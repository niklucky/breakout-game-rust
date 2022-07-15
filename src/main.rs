use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }

    pub fn update(&mut self, dt: f32) {
        // It will be much simpler to just write if / else, but why not tuples?
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        // Checking for screen bounds
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();

    loop {
        player.update(get_frame_time());
        clear_background(WHITE);
        player.draw();
        next_frame().await
    }
}
