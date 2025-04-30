use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::sprites::draw_sprite;
use crate::state::collision::{CheckTrapCollision, CollisionDetection};
use crate::state::gravity::{ApplyGravity, JumpingObstacles};
use crate::state::player::Player;
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::sleep;
use crate::state::constants::physics::{ACCELERATION, GROUND, LOWER_BOUND, MAX_VELOCITY, UPPER_BOUND};
use crate::state::structs::{Direction, GameState, Obstacle, ObstacleId, Trap, TrapId};

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

        // if damage has been taken the player must be displaced back to the left based on designated_x
        if game_state.damage_taken {
            if game_state.player.x > game_state.designated_x {
                game_state.player.x -= 2.0;
                game_state.player.invincible = true;
            } else {
                game_state.damage_taken = false;
                game_state.player.invincible = false;
            }
        }


        if game_state.player.game_over {

            for _ in 0..9 {
                draw_sprite(0,0,
                            &game_state.sprites.game_over[game_state.game_over_index],
                            game_state.window_buffer,
                            game_state.all_maps[game_state.current_map_index].width
                );
                render_pixel_buffer(game_state);
                game_state.game_over_index += 1;

                let amount_to_sleep = if game_state.game_over_index >= 6 {
                    500
                } else {
                    100
                };

                sleep(std::time::Duration::from_millis(amount_to_sleep));
            }

            // Reset game state
            game_state.game_over_index = 0;
            game_state.player = Player::new(0.0, GROUND); // Reset player state
            game_state.mountains_sprite_frame_index = 0;
        }
    }
}

pub fn increase_velocity(game_state: &mut GameState) {
    game_state.player.vx += ACCELERATION;

    if game_state.player.obstacle_detected {
        game_state.player.vx = 0.0;
    } else {
        if game_state.player.vx > MAX_VELOCITY {
            game_state.player.vx = MAX_VELOCITY;
        } else {
            game_state.player.vx *= 0.98;
            if game_state.player.vx > MAX_VELOCITY {
                game_state.player.vx = MAX_VELOCITY;
            }
        }
    }
}

pub fn decrease_velocity(game_state: &mut GameState) {
    game_state.player.vx *= 0.95;
    if game_state.player.vx.abs() < 0.1 {
        game_state.player.vx = 0.0;
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

pub struct AlternateHeartSpriteFrames;

impl CoreLogic for AlternateHeartSpriteFrames {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Alternate between the heart sprite frames every 200 milliseconds
        if game_state.last_heart_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(500) {
            game_state.heart_sprite_frame_index = (game_state.heart_sprite_frame_index + 1) % 2; // Cycle between 0 and 1
            game_state.last_heart_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}

pub struct AlternateGroundSpriteFrames;

impl CoreLogic for AlternateGroundSpriteFrames {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Alternate between the grass sprite frames every 200 milliseconds
        if game_state.last_ground_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(200) {
            game_state.ground_sprite_frame_index = (game_state.ground_sprite_frame_index + 1) % 2; // Cycle between 0 and 1
            game_state.last_ground_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}

pub struct AlternateLightHouseSpriteFrames;

impl CoreLogic for AlternateLightHouseSpriteFrames {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Alternate between the lighthouse sprite frames every 200 milliseconds
        if game_state.last_light_house_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(750) {
            game_state.lighthouse_sprite_frame_index = (game_state.lighthouse_sprite_frame_index + 1) % 4; // Cycle between 0 and 3
            game_state.last_light_house_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}

pub struct AlternateToxicTrapSpriteFrames;

impl CoreLogic for AlternateToxicTrapSpriteFrames {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Alternate between the toxic trap sprite frames
        if game_state.toxic_trap_sprite_frame_index >= 4 {
            if game_state.last_toxic_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(100) {
                game_state.toxic_trap_sprite_frame_index = if game_state.toxic_trap_sprite_frame_index == 4 { 5 } else { 4 };
                game_state.last_toxic_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
            }
        } else if game_state.last_toxic_sprite_frame_index_change.elapsed() >= std::time::Duration::from_millis(200) {
            game_state.toxic_trap_sprite_frame_index = (game_state.toxic_trap_sprite_frame_index + 1) % 6; // Cycle between 0 and 5
            game_state.last_toxic_sprite_frame_index_change = std::time::Instant::now(); // Reset the timer to current time
        }
    }
}

pub struct SpawnObstacles;

impl CoreLogic for SpawnObstacles {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if !game_state.obstacle_spawned {
            spawn_obstacle(200.0, 200.0, game_state.all_maps[game_state.current_map_index].obstacles);
            spawn_obstacle(350.0, 200.0, &mut game_state.all_maps[game_state.current_map_index].obstacles);
            game_state.obstacle_spawned = true;
        }


    }
}

pub struct SpawnTraps;

impl CoreLogic for SpawnTraps {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if !game_state.trap_spawned {
            spawn_trap(216.0, 200.0, game_state.all_maps[game_state.current_map_index].traps);
            spawn_trap(366.0, 200.0, game_state.all_maps[game_state.current_map_index].traps);
            game_state.trap_spawned = true;
        }
    }
}

fn spawn_obstacle(x: f32, y: f32, obstacles: &mut Vec<Obstacle>) {
    let x_left = x;
    let x_right = x + 16.0;
    let y_bottom = y;
    let y_top = y_bottom - 16.0;

    // Add a new obstacle
    obstacles.push(Obstacle {
        id: ObstacleId(obstacles.len()),
        x_left,
        x_right,
        y_bottom,
        y_top,
        active: true,
        durability: 2,
        falling: false,
        velocity_y: 0.0,
        left_obstacle: None,
        right_obstacle: None,
        over_obstacle: None,
        under_obstacle: None,
        is_bottom_obstacle: false,
        is_top_obstacle: true,
        is_leftmost_obstacle: false,
        is_rightmost_obstacle: false,
    });

    println!("Spawned obstacle at x: {}, y: {}", x, y_bottom);
}

fn spawn_trap(x: f32, y: f32, traps: &mut Vec<Trap>) {
    let x_left = x;
    let x_right = x + 16.0;
    let y_bottom = y;
    let y_top = y_bottom - 16.0;

    // Add a new trap
    traps.push(Trap {
        id: TrapId(traps.len()),
        x_left,
        x_right,
        y_bottom,
        y_top,
        active: true
    });

    println!("Spawned trap at x: {}, y: {}", x, y);
}

fn spawn_stacked_obstacles(
    x: f32,
    y_start: f32,
    count: usize,
    obstacles: &mut Vec<Obstacle>,
    traps: &mut Vec<Trap>,
) {
    let mut y_bottom = y_start;
    let mut y_top = y_bottom - 16.0;

    for i in 0..count {
        // Add a new obstacle
        obstacles.push(Obstacle {
            id: ObstacleId(obstacles.len()),
            x_left: x,
            x_right: x + 16.0,
            y_bottom,
            y_top,
            active: true,
            durability: 2,
            falling: false,
            velocity_y: 0.0,
            left_obstacle: None,
            right_obstacle: None,
            over_obstacle: if i > 0 { Some(ObstacleId(obstacles.len() - 1)) } else { None },
            under_obstacle: None,
            is_bottom_obstacle: i == 0,
            is_top_obstacle: i == count - 1,
            is_leftmost_obstacle: false,
            is_rightmost_obstacle: false,
        });

        // Update y-coordinates for the next obstacle
        y_bottom = y_top;
        y_top -= 16.0;
    }

    // Add a trap at the base of the stack
    traps.push(Trap {
        id: TrapId(traps.len()),
        x_left: x + 16.0,
        x_right: x + 32.0,
        y_bottom: y_start,
        y_top: y_start - 16.0,
        active: true,
    });

    println!(
        "Spawned {} stacked obstacles and a trap at x: {}, starting y: {}",
        count, x, y_start
    );
}

pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("AlternateLayerThreeSpriteFrames".to_string(), Rc::new(RefCell::new(AlternateGroundSpriteFrames)));
    logic_map.insert("AlternateHeartSprites".to_string(), Rc::new(RefCell::new(AlternateHeartSpriteFrames)));
    logic_map.insert("AlternateLightHouseSprites".to_string(), Rc::new(RefCell::new(AlternateLightHouseSpriteFrames)));
    logic_map.insert("AlternateToxicTrapSprites".to_string(), Rc::new(RefCell::new(AlternateToxicTrapSpriteFrames)));

    logic_map.insert("SpawnObstacles".to_string(), Rc::new(RefCell::new(SpawnObstacles)));
    logic_map.insert("SpawnTraps".to_string(), Rc::new(RefCell::new(SpawnTraps)));

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
