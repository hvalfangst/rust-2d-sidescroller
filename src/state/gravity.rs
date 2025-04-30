use rodio::Sink;
use crate::audio::engine::append_source_source;
use crate::state::core_logic::CoreLogic;
use crate::state::constants::audio::{DOWN_SOUND, FALL_MILD_SOUND};
use crate::state::constants::physics::{GRAVITY, GROUND};
use crate::state::player::PlayerState;
use crate::state::structs::GameState;

pub struct ApplyGravity;

impl CoreLogic for ApplyGravity {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        Self::handle_player_gravity(game_state);
        Self::handle_falling_obstacles(game_state, sink);
    }
}

impl ApplyGravity {
    fn handle_falling_obstacles(game_state: &mut GameState, sink: &mut Sink) {
        let mut obstacle_landed = false;

        // Apply gravity to all obstacles which has the falling boolean
        for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter_mut() {
            if obstacle.active && obstacle.falling {
                if obstacle.velocity_y >= 16.0 {
                    obstacle_landed = true;
                    obstacle.falling = false;
                } else {
                    obstacle.y_bottom += GRAVITY * 3.0;
                    obstacle.y_top += GRAVITY * 3.0;
                    obstacle.velocity_y += GRAVITY * 3.0;
                }
            }
        }

        if obstacle_landed {
            append_source_source(&game_state, sink, DOWN_SOUND, 3000);
            game_state.all_maps[game_state.current_map_index].obstacles.sort_by(|a, b| a.y_bottom.partial_cmp(&b.y_bottom).unwrap());
        }
    }

    fn handle_player_gravity(game_state: &mut GameState) {
        // Apply gravity to the player
        if !game_state.player.on_ground && !game_state.player.on_obstacle {
            game_state.player.vy += GRAVITY;
        }
    }
}


pub struct JumpingObstacles;

impl CoreLogic for JumpingObstacles {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        jump_obstacles(game_state, sink);
    }
}

pub fn jump_obstacles(mut game_state: &mut GameState, sink: &mut rodio::Sink) {

    // Apply vertical velocity if jumping
    if game_state.player.is_jumping {
        game_state.player.y += game_state.player.vy;
    }

    // Check if game_state.player is almost on the ground
    if game_state.player.y >= 140.0 && game_state.player.y <= 160.0 {
        game_state.player.almost_ground = true;
    } else {
        game_state.player.almost_ground = false;
    }

    let mut on_any_obstacle = false;

    // Check for each obstacle
    for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter() {

        if obstacle.active == false {
            continue;
        }

        if game_state.player.x + 10.0 > obstacle.x_left && game_state.player.x + 5.0 < obstacle.x_right {
            if game_state.player.y <= obstacle.y_bottom && game_state.player.y >= obstacle.y_top && obstacle.is_top_obstacle {
                // println!("game_state.player.y: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", game_state.player.y, obstacle.y_bottom, obstacle.y_top);
                if game_state.player.state != PlayerState::OnObstacle {
                    // player just landed on the obstacle
                    game_state.player.y = obstacle.y_bottom - 10.0;
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                    game_state.player.is_jumping = false;
                    game_state.player.state = PlayerState::OnObstacle;
                    game_state.player.vy = 0.0;
                    // println!("Player is on an obstacle");
                } else {
                    // game_state.player is already on the obstacle
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                }
                on_any_obstacle = true;
                break;
            } else if game_state.player.y < obstacle.y_top {
                // player is above the obstacle but not touching it
                game_state.player.on_ground = false;
                game_state.player.on_obstacle = false;
                game_state.player.above_obstacle = true;
                game_state.player.state = PlayerState::InAir;
                game_state.player.is_jumping = true;
                on_any_obstacle = true;
                break;
            }
        }
    }

    if !on_any_obstacle {
        if game_state.player.y >= GROUND {
            // player is on the ground (not on an obstacle)
            game_state.player.y = GROUND;
            game_state.player.vy = 0.0;
            game_state.player.on_ground = true;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = false;

            if game_state.player.state == PlayerState::InAir {
                append_source_source(&game_state, sink, FALL_MILD_SOUND, 2500);
            }

            game_state.player.state = PlayerState::OnGround;

            // println!("Player is on the ground");
        } else {
            // player is in the air (not above any obstacle)
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.above_obstacle = false;
            game_state.player.state = PlayerState::InAir;
            game_state.player.is_jumping = true;
            // println!("Player is in the air");
        }
    }
}