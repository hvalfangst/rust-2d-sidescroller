use minifb::Key;
use rodio::Sink;
use crate::audio::engine::append_source_source;
use crate::input::handler::InputLogic;
use crate::state::{GameState, JUMP_SOUND, JUMP_VELOCITY};

pub struct Jump;

impl InputLogic for Jump {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        if !game_state.player.is_jumping && (game_state.player.on_ground || game_state.player.on_obstacle) {
            game_state.player.vy = JUMP_VELOCITY;
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = true;
            game_state.player.last_key = Some(Key::Space);

            append_source_source(&game_state, sink, JUMP_SOUND, 1500);
        }
    }
}