use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::Instant;

use minifb::Key;

use crate::graphics::render_graphics::render_pixel_buffer;
use crate::state::{BACKGROUND_CHANGE_INTERVAL, GameState, spawn_obstacle};
use crate::state::core_logic::{execute_core_logic, CoreLogic};
use crate::state::FRAME_DURATION;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::input::handler::{handle_user_input, InputLogicMap};

pub fn start_event_loop(mut game_state: GameState, input_logic_map: InputLogicMap, core_logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut rodio::Sink) {

    let mut last_grass_sprite_index_change = Instant::now();
    let mut last_footstep_time = Instant::now();
    let mut last_heart_sprite_index_change = Instant::now();
    let mut last_layer_4_sprite_index_change: Instant = Instant::now();
    let mut movement_key_press_count = 0;
    let mut spawned = false;

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while game_state.window.is_open() && !game_state.window.is_key_down(Key::Escape) {
        let start = Instant::now();

        // Alternate the heart sprite every 500 milliseconds
        if last_heart_sprite_index_change.elapsed() >= std::time::Duration::from_millis(500) {
            game_state.heart_sprite_index = (game_state.heart_sprite_index + 1) % 2; // Cycle between 0 and 1
            last_heart_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        // Alternate between lighthouse sprites every 250 milliseconds
        if last_layer_4_sprite_index_change.elapsed() >= std::time::Duration::from_millis(900) {
            game_state.layer_4_sprite_index = (game_state.layer_4_sprite_index + 1) % 4; // Cycle between 0 and 3
            last_layer_4_sprite_index_change = Instant::now(); // Reset the timer to current time
        }

        if !spawned {
            spawn_obstacle(200.0, &mut game_state.all_maps[game_state.current_map_index].obstacles);
            spawned = true;
        }

        if last_footstep_time.elapsed() >= std::time::Duration::from_millis(500) {
            game_state.footstep_active = true;
            last_footstep_time = Instant::now();
        }

        // Handle basic user input, which influence the player's state such as velocity, direction, etc.
        handle_user_input(&mut game_state, &input_logic_map, sink);

        if game_state.window.is_key_down(Key::D) || game_state.window.is_key_down(Key::A) {
            movement_key_press_count += 1;
            if movement_key_press_count >= 20 {
                if game_state.window.is_key_down(Key::D) {
                    game_state.mountain_index = (game_state.mountain_index + 1) % 4; // Increment up to 3
                } else if game_state.window.is_key_down(Key::A) {
                    if game_state.mountain_index > 0 {
                        game_state.mountain_index -= 1; // Decrement down to 0
                    }
                }

                movement_key_press_count = 0;
            }
        }

        // Process game logic such as obstacle detection, physics, sounds etc.
        execute_core_logic(&mut game_state, &core_logic_map, sink);

        // Change grass sprite every second - alternate between 0 and 1
        if last_grass_sprite_index_change.elapsed() >= BACKGROUND_CHANGE_INTERVAL {
            game_state.grass_sprite_index = (game_state.grass_sprite_index + 1) % 2; // Cycle between 0 and 1
            last_grass_sprite_index_change = Instant::now(); // Reset the timer to current time
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