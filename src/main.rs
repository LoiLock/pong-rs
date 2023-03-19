use std::{collections::LinkedList, process::exit};

use macroquad::{audio, prelude::*, ui::root_ui};

const SQUARE_SIZE: f32 = 10.0;

type Coordinate = (f32, f32);

type Velocity = (f32, f32);

struct MoveablePlayer {
    length: i16,
    pos: Coordinate,
    direction: i16, // -1, 0 or 1
}

impl MoveablePlayer {
    // Called every tick to make sure we're not moving unless key is pressed
    fn prepare_tick(&mut self) {
        self.direction = 0
    }

    fn move_up(&mut self) {
        self.direction = -1
    }

    fn move_down(&mut self) {
        self.direction = 1
    }
}

struct Ball {
    size: f32,
    pos: Coordinate,
    velocity: Velocity,
}

fn get_display_text(score_one: i32, score_two: i32) -> String {
    format!("{}, {}", score_one, score_two)
}

fn random_direction() {
    rand::srand(macroquad::miniquad::date::now() as _);

    let y_direction = rand::rand();
}

#[macroquad::main("WAJO (just pong)")]
async fn main() {
    let mut last_update = get_time();
    let mut last_player_update = get_time();
    let point_sound = audio::load_sound("point.wav")
        .await
        .expect("Can't find the point sound file");

    let wall_sound = audio::load_sound("wall.wav")
        .await
        .expect("Can't find the wall sound file");

    let paddle_sound = audio::load_sound("paddle.wav")
        .await
        .expect("Can't find the paddle sound file");

    let squares_x = screen_width() / SQUARE_SIZE;
    let squares_y = screen_height() / SQUARE_SIZE;

    let mut ball_speed = 0.05;
    let mut player_speed = 0.02;
    let startingpoint: Coordinate = (0.0, 0.0);

    let font_size = 30.0;

    let mut speed_multiplier = 1.0;

    rand::srand(macroquad::miniquad::date::now() as _);

    let mut ball = Ball {
        size: 10.0,
        pos: (
            (screen_width() / SQUARE_SIZE) / 2.0,
            (screen_height() / SQUARE_SIZE) / 2.0,
        ),
        velocity: (-1.0, 1.0),
    };

    let mut player = MoveablePlayer {
        pos: (squares_x - 4.0, squares_y / 2.0),
        direction: 0,
        length: 10,
    };

    let mut computer_player = MoveablePlayer {
        pos: (4.0, squares_y / 2.0 - 10.0),
        direction: 0,
        length: 10,
    };

    println!("Ball pos: {:?}", ball.pos);

    let mut gameover = false;
    let mut computer_score = 0;
    let mut player_score = 0;

    let mut display_text = get_display_text(computer_score, player_score);

    let mut cheat_enabled = false;

    loop {
        if (is_key_down(KeyCode::Escape)) {
            exit(0);
        }
        if !gameover {
            display_text = get_display_text(computer_score, player_score);
            clear_background(BLACK);

            if is_key_down(KeyCode::Space) {
                speed_multiplier = 3.0;
            } else {
                speed_multiplier = 1.0;
            }

            if (is_key_down(KeyCode::X)) {
                cheat_enabled = true
            } else {
                cheat_enabled = false
            }

            if is_key_down(KeyCode::Up) {
                player.move_up();
            }
            if is_key_down(KeyCode::Down) {
                player.move_down();
            }

            // Draw ball
            draw_rectangle(
                ball.pos.0 * SQUARE_SIZE,
                ball.pos.1 * SQUARE_SIZE,
                ball.size,
                ball.size,
                WHITE,
            );

            // Draw player
            draw_rectangle(
                player.pos.0 * SQUARE_SIZE,
                (player.pos.1 - (player.length as f32 / 2.0)) * SQUARE_SIZE,
                ball.size,
                player.length as f32 * SQUARE_SIZE,
                WHITE,
            );

            // Draw computer_player
            draw_rectangle(
                computer_player.pos.0 * SQUARE_SIZE,
                (computer_player.pos.1 - (computer_player.length as f32 / 2.0)) * SQUARE_SIZE,
                ball.size,
                computer_player.length as f32 * SQUARE_SIZE,
                WHITE,
            );

            if ball.pos.0 < 0.0 {
                player_score += 1;

                gameover = true;
            }

            if ball.pos.0 > squares_x - 1.0 {
                computer_score += 1;

                gameover = true;
            }

            if get_time() - last_player_update > player_speed {
                last_player_update = get_time();

                player.pos.1 += player.direction as f32;
                computer_player.pos.1 += computer_player.direction as f32;
            }

            // If 0.1 seconds has elapsed, update the playing field
            if get_time() - last_update > ball_speed / speed_multiplier {
                player.prepare_tick();
                last_update = get_time();

                ball.pos.0 += ball.velocity.0;
                ball.pos.1 += ball.velocity.1;
                println!("Ball pos: {:?}", ball.pos);

                if ball.pos.1 >= squares_y - 1.0 {
                    ball.velocity.1 = -1.0;
                    audio::play_sound_once(wall_sound);
                } else if ball.pos.1 <= 0.0 {
                    ball.velocity.1 = 1.0;
                    audio::play_sound_once(wall_sound);
                }

                // Player collision
                if ball.pos.0 == player.pos.0 - 1.0
                    && ball.pos.1 > player.pos.1 - player.length as f32 / 2.0
                    && ball.pos.1 < player.pos.1 + player.length as f32 / 2.0
                {
                    audio::play_sound_once(paddle_sound);
                    ball.velocity = (ball.velocity.0 * -1.0, ball.velocity.1);

                    ball_speed *= 0.9;
                }

                // Computer collision
                if ball.pos.0 == computer_player.pos.0 + 1.0
                    && ball.pos.1 > computer_player.pos.1 - computer_player.length as f32 / 2.0
                    && ball.pos.1 < computer_player.pos.1 + computer_player.length as f32 / 2.0
                {
                    audio::play_sound_once(paddle_sound);
                    ball.velocity = (ball.velocity.0 * -1.0, ball.velocity.1);

                    ball_speed *= 0.9;
                }

                // println!("Delta X to ball: {}", delta_x_to_ball);
                // println!("Delta Y to ball: {}", delta_y_to_ball);
                // println!("px seconds: {}", px_seconds);
                // println!("remaining_y_delta: {}", remaining_y_delta);
            }

            // Computer prediction
            // let paddle_center = computer_player.pos.1 - (computer_player.length as f32 / 2.0);
            // let delta_y_to_ball =
            //     ball.pos.1 - paddle_center;

            // let delta_x_to_ball = ball.pos.0 - computer_player.pos.0 - 1.0;

            // let px_seconds = 1.0 / ball_speed;

            // ! Really unfair :)
            computer_player.pos = (computer_player.pos.0, ball.pos.1);

            // Exit game loop whenever it's game over
            if gameover {
                audio::play_sound_once(point_sound);

                continue;
            }
        } else {
            player.pos = (squares_x - 4.0, squares_y / 2.0);

            computer_player.pos = (4.0, squares_y / 2.0 - 10.0);

            ball.pos = (
                (screen_width() / SQUARE_SIZE) / 2.0,
                (screen_height() / SQUARE_SIZE) / 2.0,
            );

            ball_speed = 0.05;
            player_speed = 0.02;

            gameover = false;
        }

        let title_text = "WAJO".to_string();

        let title_size = measure_text(&title_text, None, 40, 1.0);
        let score_size = measure_text(&display_text, None, font_size as _, 1.0);

        draw_text(
            &title_text,
            screen_width() / 2. - score_size.width / 2.,
            title_size.height * 2.0,
            40.0,
            DARKGRAY,
        );

        draw_text(
            &display_text,
            screen_width() / 2. - score_size.width / 2.,
            screen_height() / 2. - score_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        next_frame().await
    }
}
