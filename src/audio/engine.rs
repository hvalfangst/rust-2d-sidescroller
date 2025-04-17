use std::io::{BufReader, Cursor};
use rodio::{Sink, Source};
use crate::state::{GameState, WALK_SOUND_1, WALK_SOUND_2, WALK_SOUND_3, WALK_SOUND_4};

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