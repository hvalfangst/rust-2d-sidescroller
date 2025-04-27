use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::Key;

use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::input::handler::{handle_user_input, InputLogicMap};
use crate::state::core_logic::{execute_core_logic, CoreLogic};
use crate::state::FRAME_DURATION;
use crate::state::GameState;

pub fn start_event_loop(mut game_state: GameState, input_logic_map: InputLogicMap, core_logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut rodio::Sink) {

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while game_state.window.is_open() && !game_state.window.is_key_down(Key::Escape) {
        let start = Instant::now();

        // Handle basic user input, which influence the player's state such as velocity, direction, etc.
        handle_user_input(&mut game_state, &input_logic_map, sink);

        // Process game logic such as obstacle detection, physics, sounds etc.
        execute_core_logic(&mut game_state, &core_logic_map, sink);

        // Update the pixel buffer with the current game state
        update_pixel_buffer(&mut game_state);

        // Render the updated buffer
        render_pixel_buffer(&mut game_state);

        // Maintain a frame rate of 60 fps
        let elapsed = start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed);
        }
    }
}