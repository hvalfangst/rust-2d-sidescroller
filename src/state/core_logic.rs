use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::state::collision::CollisionDetection;
use crate::state::gravity::{ApplyGravity, JumpingObstacles};
use crate::state::physics::{CheckTrapCollision, ModifyPosition};
use crate::state::player::Player;
use crate::state::{GameState, GROUND, LOWER_BOUND, UPPER_BOUND};
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::sleep;
use crate::graphics::sprites::draw_sprite;

pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state, sink);
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // println!("Player X: {}, Y: {}", game_state.player.x, game_state.player.y);

        // Prevent the player from moving out vertical (y) bounds
        if game_state.player.y <= 40.0 {
            game_state.player.on_ground = false;
            game_state.player.y = GROUND;
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Prevent the player from moving out horizontal (x) bounds
        if game_state.player.x < LOWER_BOUND {
            game_state.player.x = LOWER_BOUND;
            game_state.player.vx = 0.0;
        } else if game_state.player.x >= UPPER_BOUND {
            // game_state.player.x = 0.0;
            // game_state.player.vx = 0.0;
            // game_state.current_map_index += 1
        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.game_over {

            for _ in 0..9 {
                draw_sprite(0,0,
                            &game_state.sprites.game_over[game_state.game_over_index],
                            game_state.window_buffer,
                            game_state.all_maps[game_state.current_map_index].width
                );
                render_pixel_buffer(game_state);
                game_state.game_over_index += 1;
                sleep(std::time::Duration::from_millis(100));
            }

            // Reset game state
            game_state.game_over_index = 0;
            game_state.player = Player::new(0.0, GROUND); // Reset player state
            game_state.layer_0_index = 0;
        }
    }
}


pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();
    logic_map.insert("JumpingObstacles".to_string(), Rc::new(RefCell::new(JumpingObstacles)));
    logic_map.insert("CollisionDetection".to_string(), Rc::new(RefCell::new(CollisionDetection)));
    logic_map.insert("ApplyGravity".to_string(), Rc::new(RefCell::new(ApplyGravity)));
    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    logic_map.insert("CheckGameOver".to_string(), Rc::new(RefCell::new(CheckGameOver)));
    logic_map.insert("ModifyPosition".to_string(), Rc::new(RefCell::new(ModifyPosition)));
    logic_map.insert("CheckTrapCollision".to_string(), Rc::new(RefCell::new(CheckTrapCollision)));

    logic_map
}
