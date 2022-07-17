use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

struct Game {
    score: i32,
    state: GameState,

    player: Player,
    blocks: Vec<Block>,
    balls: Vec<Ball>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
            state: GameState::Menu,
            player: Player::new(),
            blocks: Vec::new(),
            balls: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        self.state = GameState::Menu;
        self.score = 0;
        self.player.lives = 3;
        self.player = Player::new();
        self.balls = Vec::new();
        self.blocks.clear();
        self.init_blocks();
    }
    pub fn init_blocks(&mut self) {
        let (width, height) = (6, 6);
        let padding = 5f32;
        let total_block_size = BLOCK_SIZE + vec2(padding, padding);

        let board_start_pos = vec2(
            (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
            50f32,
        );

        for i in 0..width * height {
            let block_x = (i % width) as f32 * total_block_size.x;
            let block_y = (i / width) as f32 * total_block_size.y;

            self.blocks
                .push(Block::new(board_start_pos + vec2(block_x, block_y)));
        }
    }
    
    pub fn init_ball(&mut self) {
        self.balls.push(Ball::new(vec2(
            screen_width() * 0.5f32,
            screen_height() * 0.5f32,
        )));
    }
    pub fn update_player(&mut self, dt: f32) {
        self.player.update(dt);
    }
    pub fn update_balls(&mut self, dt: f32) {
        for ball in self.balls.iter_mut() {
            ball.update(dt);
        }
    }
    pub fn check_collision(&mut self) {
        // Checking for a collision
        for ball in self.balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vel, &self.player.rect);
            for block in self.blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                    block.lives -= 1;
                    if block.lives <= 0 {
                        self.score += 10;
                    }
                }
            }
        }

        let balls_len = self.balls.len();
        let was_last_ball = balls_len == 1;
        self.balls.retain(|ball| ball.rect.y < screen_height());
        let removed_balls = balls_len - self.balls.len();
        if removed_balls > 0 && was_last_ball {
            self.player.lives -= 1;
            if self.player.lives <= 0 {
                self.state = GameState::Dead;
            }
        }

        self.blocks.retain(|block| block.lives > 0);

        if self.blocks.is_empty() {
            self.state = GameState::LevelCompleted;
        }
    }

    pub fn draw(&mut self) {
        self.player.draw();

        for block in self.blocks.iter() {
            block.draw();
        }
        for ball in self.balls.iter() {
            ball.draw();
        }

    }
}

pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: 50u16,
            color: BLACK,
            ..Default::default()
        },
    )
}

struct Player {
    rect: Rect,
    lives: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            lives: 3,
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
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

struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
        }
    }
    pub fn draw(&self) {
        let color = match self.lives {
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        // Reversing velocity
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}


#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/Heebo-VariableFont_wght.ttf")
        .await
        .unwrap();

    let mut game = Game::new();
    game.init_blocks();
    game.init_ball();

    loop {
        match game.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game.state = GameState::Game;
                }
            }
            GameState::Game => {
                if is_key_pressed(KeyCode::Space) {
                    game.init_ball();
                }
                game.update_player(get_frame_time());
                game.update_balls(get_frame_time());
                game.check_collision();
            }
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game.reset();
                }
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game.reset();
                }
            }
        }

        clear_background(WHITE);
        game.draw();

        // Simple state match
        match game.state {
            GameState::Menu => {
                draw_title_text("Press SPACE to start", font);
            }
            GameState::Game => {
                let score_text = format!("score: {}", game.score);
                let score_text_dimensions = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - (score_text_dimensions.width * 0.5f32),
                    40.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
                draw_text_ex(
                    &format!("lives: {}", game.player.lives),
                    30.0,
                    40.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!("You win! Score: {}", game.score), font);
            }
            GameState::Dead => {
                draw_title_text(&format!("You DIED! Score: {}", game.score), font);
            }
        }
        next_frame().await
    }
}
