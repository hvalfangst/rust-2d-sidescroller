use crate::audio::engine::append_source_source;
use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::update_pixel_buffer;
use crate::input::input_logic::check_collision;
use crate::state::player::Player;
use crate::state::{jump_obstacles, Direction, GameState, DOWN_SOUND, GRAVITY, GROUND, LOWER_BOUND, UPPER_BOUND};
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::sleep;


pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink, any_key_pressed: bool) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state, sink);
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct CollisionDetection;

impl CoreLogic for CollisionDetection {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let obstacles = &game_state.all_maps[game_state.current_map_index].obstacles;
        let direction = game_state.player.direction;
        let (obstacle, _id) = check_collision(obstacles, &game_state.sprites, &game_state.player, direction == Direction::Left);

        if obstacle {
            game_state.player.vx = 0.0;
            game_state.player.obstacle_detected = true;
        } else {
            game_state.player.obstacle_detected = false;
        }

    }
}

pub struct ApplyGravity;

impl CoreLogic for ApplyGravity {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Apply gravity to the player
        if !game_state.player.on_ground && !game_state.player.on_obstacle {
            game_state.player.vy += GRAVITY;
        }

        let mut obstacle_landed = false;

        // Apply gravity to all obstacles which has the falling boolean
        for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter_mut() {
            if obstacle.active && obstacle.falling {
                if obstacle.velocity_y >= 16.0 {
                    obstacle_landed = true;
                    obstacle.falling = false;
                } else {
                    obstacle.y_bottom += GRAVITY * 3.0;
                    obstacle.y_top += GRAVITY * 3.0;
                    obstacle.velocity_y += GRAVITY * 3.0;
                }
            }
        }

        if obstacle_landed {
            append_source_source(&game_state, sink, DOWN_SOUND, 3000);

            game_state.all_maps[game_state.current_map_index].obstacles.sort_by(|a, b| a.y_bottom.partial_cmp(&b.y_bottom).unwrap());
        }
    }
}

pub struct JumpingObstacles;

impl CoreLogic for JumpingObstacles {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        jump_obstacles(game_state, sink);
    }
}

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
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
            game_state.player.x = 0.0;
            game_state.player.vx = 0.0;
            game_state.current_map_index += 1
        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.game_over {
            println!("Game Over!");

            for _ in 0..4 {
                update_pixel_buffer(game_state);
                render_pixel_buffer(game_state);
                game_state.game_over_index += 1;
                sleep(std::time::Duration::from_millis(200));
            }

            game_state.game_over_index = 0;
            game_state.player = Player::new(0.0, GROUND); // Reset player state
        }
    }
}

pub struct ModifyPosition;

impl CoreLogic for ModifyPosition {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.direction == Direction::Left {
            game_state.player.x -= game_state.player.vx;
        } else {
            game_state.player.x += game_state.player.vx;
        }
        game_state.player.y += game_state.player.vy;
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

    logic_map
}
