use crate::input::jump::Jump;
use crate::input::kick::Kick;
use crate::input::move_left::MoveLeft;
use crate::input::move_right::MoveRight;
use crate::state::GameState;
use minifb::{Key, KeyRepeat};
use rodio::Sink;
use std::collections::HashMap;
use std::sync::Arc;
use crate::state::physics::decrease_velocity;

pub fn handle_user_input(game_state: &mut GameState, commands: &InputLogicMap, sink: &mut Sink) -> bool {
    let legal_keys = [Key::Space, Key::D, Key::A, Key::X];
    let mut any_key_pressed = false;
    let mut movement_key_pressed = false;

    for key in legal_keys.iter() {
        if game_state.window.is_key_pressed(*key, KeyRepeat::Yes) {
            any_key_pressed = true;
            if *key == Key::A || *key == Key::D {
                movement_key_pressed = true;
            }
            delegate_command(*key, &commands, game_state, sink);
        }
    }

    // If no legal was pressed, decelerate the player to avoid sliding forever
    if !any_key_pressed {
        decrease_velocity(game_state);
    }

    any_key_pressed
}

fn delegate_command(key: Key, commands: &InputLogicMap, game_state: &mut GameState, sink: &mut Sink) {
    if let Some(command) = commands.get(&key) {
        command.execute(game_state, sink);
    } else {
        println!("No command associated with key: {:?}", key);
    }
}

pub trait InputLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}


pub type InputLogicMap = HashMap<Key, Arc<dyn InputLogic>>;

pub fn initialize_input_logic_map() -> InputLogicMap {
    let mut logic_map: InputLogicMap = HashMap::new();

    logic_map.insert(Key::A, Arc::new(MoveLeft));
    logic_map.insert(Key::D, Arc::new(MoveRight));
    logic_map.insert(Key::Space, Arc::new(Jump));
    logic_map.insert(Key::X, Arc::new(Kick));

    logic_map
}
