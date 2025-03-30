use minifb::Key;
use rodio::Sink;
use crate::audio::engine::play_footstep_sound;
use crate::input::handler::{InputLogic};
use crate::state::Direction::Left;
use crate::state::GameState;
use crate::state::physics::increase_velocity;

pub struct MoveLeft;
impl InputLogic for MoveLeft {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity if no collision is detected
        increase_velocity(game_state);

        game_state.player.last_key = Some(Key::A);
        game_state.player.direction = Left;

        // Initialize a new field to track the frame count
        game_state.player.left_increment_frame_count += 1;

        if game_state.player.left_increment_frame_count >= 3 {
            game_state.player.left_increment_frame_count = 0; // Reset the frame count

            match game_state.player.left_increment {
                7 => {
                    game_state.player.left_increment = 4;
                }
                _ => {
                    game_state.player.left_increment += 1;
                }
            };
        }

        if game_state.footstep_active {
            play_footstep_sound(game_state, sink);
        }
    }
}