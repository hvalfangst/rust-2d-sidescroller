use rodio::Sink;
use crate::state::constants::audio::{WALK_SOUND_1, WALK_SOUND_2, WALK_SOUND_3, WALK_SOUND_4};
use crate::state::structs::GameState;

pub fn append_source_source(game_state: &&mut GameState, sink: &mut Sink, sample: usize, duration: u64) {
    // let file = &game_state.sounds[sample];
    // let cursor = Cursor::new(file.clone());
    //
    // let source = rodio::Decoder::new(BufReader::new(cursor))
    //     .unwrap()
    //     .take_duration(std::time::Duration::from_millis(duration));
    //
    // sink.append(source);
}

pub fn play_footstep_sound(game_state: &mut GameState, sink: &mut Sink) {
    if game_state.footstep_index == 4 { game_state.footstep_index = 0; } else { game_state.footstep_index += 1; }

    let sound_index = match game_state.footstep_index {
        0 => WALK_SOUND_1,
        1 => WALK_SOUND_2,
        2 => WALK_SOUND_3,
        _ => WALK_SOUND_4,
    };

    append_source_source(&game_state, sink, sound_index, 200);
}