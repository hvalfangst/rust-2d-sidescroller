use rodio::Sink;
use crate::audio::engine::append_source_source;
use crate::state::core_logic::CoreLogic;
use crate::state::{GameState, DOWN_SOUND, GRAVITY};
use crate::state::physics::jump_obstacles;

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