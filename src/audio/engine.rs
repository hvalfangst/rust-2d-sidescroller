use std::io::{BufReader, Cursor};
use rodio::{Sink, Source};
use crate::state::GameState;

pub fn append_source_source(game_state: &&mut GameState, sink: &mut Sink, sample: usize, duration: u64) {
    let file = &game_state.sounds[sample];
    let cursor = Cursor::new(file.clone());

    let source = rodio::Decoder::new(BufReader::new(cursor))
        .unwrap()
        .take_duration(std::time::Duration::from_millis(duration));

    sink.append(source);
}