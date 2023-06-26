use rand::prelude::*;
use rusty_engine::prelude::*;
use std::{collections::HashMap, f32::consts::PI};

const PIPE_SPEED: f32 = 400.0;
const PLAYER_X: f32 = -450.0;
const GRAVITY: f32 = 9.8;
const MOVEMENT_MULTIPLIER: f32 = 40.0;
const TURNING_RATE: f32 = 0.02;

struct GameState {
    game_over: bool,
    pipes_spawned: usize,
    spawn_timer: Timer,
    score: usize,
    pipe_list: HashMap<String, Pipe>,
    velocity: f32,
}

struct Pipe {
    scored: bool,
}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    let game_state = GameState {
        game_over: false,
        pipes_spawned: 0,
        spawn_timer: Timer::from_seconds(0.1, false),
        score: 0,
        pipe_list: HashMap::new(),
        velocity: -10.0,
    };

    game.window_settings(WindowDescriptor {
        title: "Flappy Bird".into(),
        ..Default::default()
    });

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.scale = 0.5;
    player.translation.x = PLAYER_X;
    player.layer = 10.0;

    player.rotation = -(PI / 2.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
    game_state.spawn_timer.tick(engine.delta);
    let mut labels_to_delete: Vec<String> = Vec::new();

    if game_state.game_over == true {
        return;
    }

    if engine.keyboard_state.just_pressed(KeyCode::Space) {
        game_state.velocity += 5.0;
        if game_state.velocity >= 10.0 {
            game_state.velocity = 10.0;
        }
        println!("New Velocity: {}", game_state.velocity);
    }

    if game_state.spawn_timer.just_finished() {
        let gap = thread_rng().gen_range(0..10) as f32;
        let spawn_location = thread_rng().gen_range(-180.0..180.0);
        println!("Gap: {}, Spawn Location: {}", gap, spawn_location);

        let top_label = format!("pipe_top{}", game_state.pipes_spawned);
        let bottom_label = format!("pipe_bot{}", game_state.pipes_spawned);

        let unscored_pipe = Pipe { scored: false };

        let mut pipe_top = engine.add_sprite(top_label, SpritePreset::RacingBarrierRed);
        pipe_top.rotation = UP;
        pipe_top.scale = 3.0;
        pipe_top.translation.y = 410.0 + spawn_location;
        pipe_top.translation.x = 1500.0;
        pipe_top.collision = true;

        game_state
            .pipe_list
            .insert(pipe_top.label.clone(), unscored_pipe);

        let mut pipe_bottom = engine.add_sprite(bottom_label, SpritePreset::RacingBarrierRed);
        pipe_bottom.rotation = UP;
        pipe_bottom.scale = 3.0;
        pipe_bottom.translation.y = -410.0 + spawn_location;
        pipe_bottom.translation.x = 1500.0;
        pipe_bottom.collision = true;

        game_state.pipes_spawned += 1;
        game_state.spawn_timer = Timer::from_seconds(1.5, false);
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("pipe") {
            sprite.translation.x -= PIPE_SPEED * engine.delta_f32;
            if sprite.translation.x <= -950.0 {
                labels_to_delete.push(sprite.label.clone());
            }
            if sprite.label.contains("top")
                && sprite.translation.x <= PLAYER_X
                && game_state.pipe_list.get_mut(&sprite.label).unwrap().scored == false
            {
                game_state.score += 1;
                game_state.pipe_list.get_mut(&sprite.label).unwrap().scored = true;

                println!("Score: {}", game_state.score);
            }
        }

        if sprite.label.starts_with("player") {
            game_state.velocity -= GRAVITY * engine.delta_f32;
            if game_state.velocity <= -10.0 {
                game_state.velocity = -10.0;
            }
            // println!("Velocity: {}", game_state.velocity);

            sprite.translation.y += game_state.velocity * MOVEMENT_MULTIPLIER * engine.delta_f32;

            // sprite.rotation = target_rotation;
            //let target_rotation = ((game_state.velocity / (10.0 / (PI / 2.0))).sin() * (PI / 2.0));
            let target_rotation = (game_state.velocity / (10.0 / (PI / 2.0))).sin() * (PI / 4.0);

            if target_rotation - sprite.rotation >= TURNING_RATE {
                sprite.rotation += TURNING_RATE;
            } else {
                sprite.rotation = target_rotation;
            }
            //println!("Rotation: {}", sprite.rotation);
        }
    }

    for sprite in labels_to_delete {
        engine.sprites.remove(&sprite);
        println!("Deleted: {}", sprite);
    }
}
