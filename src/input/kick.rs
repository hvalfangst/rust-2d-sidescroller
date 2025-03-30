use rodio::Sink;
use crate::audio::engine::append_source_source;
use crate::input::handler::{InputLogic};
use crate::state::Direction::Left;
use crate::state::{GameState, KICK_BOX_SOUND, KICK_SOUND};
use crate::state::physics::check_collision;

pub struct Kick;

impl InputLogic for Kick {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        game_state.player.is_kicking = true;
        game_state.player.kick_frame = 0;
        game_state.player.kick_frame_timer = 0;

        let (collision, id) = check_collision(game_state.all_maps[game_state.current_map_index].obstacles, &game_state.sprites, &game_state.player, game_state.player.direction == Left);

        // Check if the player is adjacent to an obstacle
        if collision {
            append_source_source(&game_state, sink, KICK_BOX_SOUND, 1000);
            Self::damage_obstacle(game_state, sink, id);

        } else {
            append_source_source(&game_state, sink, KICK_SOUND, 1000);
        }
    }
}

impl Kick {
    fn damage_obstacle(game_state: &mut GameState, sink: &mut Sink, id: Option<usize>) {
        if game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability > 0 {
            game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability -= 1;
        } else {
            Self::remove_obstacle(game_state, id.unwrap(), sink);
        }
    }
    fn remove_obstacle(game_state: &mut GameState, box_index: usize, sink: &mut rodio::Sink) {
        println!("Removing box {}", box_index);
        let mut to_remove = false;
        if game_state.all_maps[game_state.current_map_index].obstacles[box_index].active {
            println!("Box is active");
            // Obtain the x_left and x_right values of the removed box
            let removed_box_x_left = game_state.all_maps[game_state.current_map_index].obstacles[box_index].x_left;
            let removed_box_x_right = game_state.all_maps[game_state.current_map_index].obstacles[box_index].x_right;
            let removed_box_y_top = game_state.all_maps[game_state.current_map_index].obstacles[box_index].y_top;

            println!("Box x_left: {}, x_right: {}", removed_box_x_left, removed_box_x_right);
            println!("Box {} removed", box_index);

            append_source_source(&game_state, sink, KICK_BOX_SOUND, 1000);

            // Shift all boxes above the removed box down by 16 pixels
            for i in 0..game_state.all_maps[game_state.current_map_index].obstacles.len() {
                println!("Box id: {}", i);

                let obstacle = &mut game_state.all_maps[game_state.current_map_index].obstacles[i];
                println!("Box {} x_left: {}, x_right: {}", i, obstacle.x_left, obstacle.x_right);
                if obstacle.x_left >= removed_box_x_left && obstacle.x_right <= removed_box_x_right { //&& obstacle.y_top < removed_box_y_top {
                    obstacle.falling = true;
                    obstacle.velocity_y = 0.0;
                    println!("Box {} is falling", i);
                    // Remove the box
                    to_remove = true;
                } else {
                    println!("Box {} is not falling. obs.x_left: {} removed.x_left: {} obs.x_right {} removed.x_right {}", i, obstacle.x_left, removed_box_x_left, obstacle.x_right, removed_box_x_right);
                }
            }
        }
        if to_remove {
            game_state.all_maps[game_state.current_map_index].obstacles.remove(box_index);
            println!("Box {} removed", box_index);
        }
    }
}