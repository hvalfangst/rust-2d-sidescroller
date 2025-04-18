use minifb::Key;
use rodio::Sink;
use crate::audio::engine::play_footstep_sound;
use crate::input::handler::{InputLogic};
use crate::state::core_logic::increase_velocity;
use crate::state::Direction::Left;
use crate::state::GameState;

pub struct MoveLeft;
impl InputLogic for MoveLeft {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity
        increase_velocity(game_state);

        // Update direction
        game_state.player.last_key = Some(Key::A);
        game_state.player.direction = Left;

        // Initialize a new field to track the frame count
        game_state.player.left_increment_frame_count += 1;

        // Cycle through the sprite map for walking left
        Self::advance_walking_animation(game_state);

        // Play footstep sound if one is eligible to do so
        if game_state.footstep_active {
            play_footstep_sound(game_state, sink);
        }
    }
}

impl MoveLeft {
    fn advance_walking_animation(game_state: &mut GameState) {
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
    }
}