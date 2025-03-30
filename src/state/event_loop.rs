use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::Key;

use crate::graphics::render_graphics::render_pixel_buffer;
use crate::state::{BACKGROUND_CHANGE_INTERVAL, GameState};
use crate::state::core_logic::{execute_core_logic, CoreLogic};
use crate::state::FRAME_DURATION;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::input::handler::{handle_user_input, InputLogicMap};

pub fn start_event_loop(mut game_state: GameState, input_logic_map: InputLogicMap, core_logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut rodio::Sink) {

    // Variables for background sprite changing
    let mut last_grass_sprite_index_change = Instant::now();
    let mut last_sky_sprite_index_change = Instant::now();
    let mut last_footstep_time = Instant::now();

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while game_state.window.is_open() && !game_state.window.is_key_down(Key::Escape) {
        let start = Instant::now();

        if last_footstep_time.elapsed() >= std::time::Duration::from_millis(500) {
            game_state.footstep_active = true;
            last_footstep_time = Instant::now();
        }

        // Handle basic user input, which influence the player's state such as velocity, direction, etc.
        let any_key_pressed = handle_user_input(&mut game_state, &input_logic_map, sink);

        // Process game logic such as obstacle detection, physics, sounds etc.
        execute_core_logic(&mut game_state, &core_logic_map, sink, any_key_pressed);

        // Change grass sprite every second - alternate between 0 and 1
        if last_grass_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            game_state.grass_sprite_index = (game_state.grass_sprite_index + 1) % 2; // Cycle between 0 and 1
            last_grass_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Change sky sprite every 2 seconds - alternate between 0 and 3
        if last_sky_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL * 2 {
            game_state.sky_sprite_index = (game_state.sky_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_sky_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

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