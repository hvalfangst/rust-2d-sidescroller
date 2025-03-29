use std::io::{BufReader, Cursor};
use std::time::Duration;

use crate::graphics::sprites::SpriteMaps;
use crate::state::player::{Player, PlayerState};
use crate::Tile;
use minifb::Window;
use rodio::Source;

pub mod event_loop;
pub mod update;
pub mod player;
pub(crate) mod input_logic;
pub(crate) mod core_logic;

const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

const GRAVITY: f32 = 0.5;
const JUMP_VELOCITY: f32 = -5.0;
const MAX_VELOCITY: f32 = 2.0;
const ACCELERATION: f32 = 0.5;
const FRICTION: f32 = 0.2;
const GROUND: f32 = 205.0;
const LOWER_BOUND: f32 = 0.0;
const UPPER_BOUND: f32 = 225.0;
const KICK_FRAME_DURATION: u32 = 8;

const WALK_SOUND_1: usize = 0;
const WALK_SOUND_2: usize = 1;
const WALK_SOUND_3: usize = 2;
const WALK_SOUND_4: usize = 3;
const JUMP_SOUND: usize = 4;
const FALL_MILD_SOUND: usize = 5;
const FALL_HEAVY_SOUND: usize = 6;
const DOWN_SOUND: usize = 7;
const EXPLOSION_SOUND: usize = 8;
const KICK_SOUND: usize = 9;
const KICK_BOX_SOUND: usize = 10;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObstacleId(pub usize);

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub id: ObstacleId,
    pub x_left: f32,
    pub x_right: f32,
    pub y_top: f32,
    pub y_bottom: f32,
    pub velocity_y: f32, // For gravity
    pub falling: bool,   // Whether it's falling
    pub active: bool,    // If false, box is removed
    pub durability: u8,  // Health of the box
    pub is_bottom_obstacle: bool,
    pub is_top_obstacle: bool,
    pub is_leftmost_obstacle: bool,
    pub is_rightmost_obstacle: bool,
    pub left_obstacle: Option<ObstacleId>,
    pub right_obstacle: Option<ObstacleId>,
    pub over_obstacle: Option<ObstacleId>,
    pub under_obstacle: Option<ObstacleId>
}
pub fn jump_obstacles(mut game_state: &mut GameState, sink: &mut rodio::Sink) {

    // Apply vertical velocity if jumping
    if game_state.player.is_jumping {
        game_state.player.y += game_state.player.vy;
    }

    // Check if game_state.player is almost on the ground
    if game_state.player.y >= 140.0 && game_state.player.y <= 160.0 {
        game_state.player.almost_ground = true;
    } else {
        game_state.player.almost_ground = false;
    }

    let mut on_any_obstacle = false;

    // Check for each obstacle
    for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter() {

        if obstacle.active == false {
            continue;
        }

        if game_state.player.x + 10.0 > obstacle.x_left && game_state.player.x + 5.0 < obstacle.x_right {
            if game_state.player.y <= obstacle.y_bottom && game_state.player.y >= obstacle.y_top && obstacle.is_top_obstacle {
                 // println!("game_state.player.y: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", game_state.player.y, obstacle.y_bottom, obstacle.y_top);
                if game_state.player.state != PlayerState::OnObstacle {
                    // player just landed on the obstacle
                    game_state.player.y = obstacle.y_bottom - 1.0;
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                    game_state.player.is_jumping = false;
                    game_state.player.state = PlayerState::OnObstacle;
                    game_state.player.vy = 0.0;
                } else {
                    // game_state.player is already on the obstacle
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                }
                on_any_obstacle = true;
                break;
            } else if game_state.player.y < obstacle.y_top {
                // player is above the obstacle but not touching it
                game_state.player.on_ground = false;
                game_state.player.on_obstacle = false;
                game_state.player.above_obstacle = true;
                game_state.player.state = PlayerState::InAir;
                game_state.player.is_jumping = true;
                on_any_obstacle = true;
                break;
            }
        }
    }

    if !on_any_obstacle {
        if game_state.player.y >= GROUND {
            // player is on the ground (not on an obstacle)
            game_state.player.y = GROUND;
            game_state.player.vy = 0.0;
            game_state.player.on_ground = true;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = false;

            if game_state.player.state == PlayerState::InAir {
                let file = &game_state.sounds[FALL_MILD_SOUND]; // Get the raw sound data (Vec<u8>)
                let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

                let source = rodio::Decoder::new(BufReader::new(cursor))
                    .unwrap()
                    .take_duration(std::time::Duration::from_millis(2500));

                sink.append(source); // Play the sound
            }

            game_state.player.state = PlayerState::OnGround;


        } else {
            // player is in the air (not above any obstacle)
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.above_obstacle = false;
            game_state.player.state = PlayerState::InAir;
            game_state.player.is_jumping = true;
        }
    }
}


pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    pub fn update(&mut self, player_x: f32, player_y: f32) {
        self.x = player_x - self.width / 2.0;
        self.y = player_y - self.height / 2.0;
    }
}

fn remove_box(game_state: &mut GameState, box_index: usize, sink: &mut rodio::Sink) {
    println!("Removing box {}", box_index);
    let mut to_remove = false;
    if game_state.all_maps[game_state.current_map_index].obstacles[box_index].active {
        println!("Box is active");
        // Obtain the x_left and x_right values of the removed box
        let removed_box_x_left = game_state.all_maps[game_state.current_map_index].obstacles[box_index].x_left;
        let removed_box_x_right = game_state.all_maps[game_state.current_map_index].obstacles[box_index].x_right;
        let removed_box_y_top = game_state.all_maps[game_state.current_map_index].obstacles[box_index].y_top;

        println!("Box x_left: {}, x_right: {}", removed_box_x_left, removed_box_x_right);

        // Remove the box

        println!("Box {} removed", box_index);

        let file = &game_state.sounds[KICK_BOX_SOUND]; // Get the raw sound data (Vec<u8>)
        let cursor = Cursor::new(file.clone()); // Clone to create an owned Cursor<Vec<u8>>

        let source = rodio::Decoder::new(BufReader::new(cursor))
            .unwrap()
            .take_duration(std::time::Duration::from_millis(1000));

        sink.append(source); // Play the sound

        // Shift all boxes above the removed box down by 16 pixels
        for i in 0..game_state.all_maps[game_state.current_map_index].obstacles.len() {
            println!("Box id: {}", i);

            let obstacle = &mut game_state.all_maps[game_state.current_map_index].obstacles[i];
            println!("Box {} x_left: {}, x_right: {}", i, obstacle.x_left, obstacle.x_right);
            if obstacle.x_left >= removed_box_x_left && obstacle.x_right <= removed_box_x_right { //&& obstacle.y_top < removed_box_y_top {
                obstacle.falling = true;
                obstacle.velocity_y = 0.0;
                println!("Box {} is falling", i);
                // Remove the box
                to_remove = true;
            } else {
                println!("Box {} is not falling. obs.x_left: {} removed.x_left: {} obs.x_right {} removed.x_right {}", i, obstacle.x_left, removed_box_x_left, obstacle.x_right, removed_box_x_right);
            }
        }
    }
    if to_remove {
        game_state.all_maps[game_state.current_map_index].obstacles.remove(box_index);
        println!("Box {} removed", box_index);
    }
}


pub struct Map<'a> {
    pub id: usize,
    pub tiles: Vec<Tile>,
    pub obstacles: &'a mut Vec<Obstacle>,
    pub width: usize,
    pub height: usize,
    pub starting_x: f32,
    pub starting_y: f32,
    pub transition_x: f32,
    pub transition_y: f32
}

pub struct GameState<'a> {
    pub player: Player,
    pub sprites: SpriteMaps,
    pub window_buffer: &'a mut Vec<u32>,
    pub grass_sprite_index: usize,
    pub sky_sprite_index: usize,
    pub window_width: usize,
    pub window_height: usize,
    pub window: &'a mut Window,
    pub scaled_buffer: &'a mut Vec<u32>,
    pub game_over_index: usize,
    pub viewport: Viewport,
    pub all_maps: Vec<Map<'a>>,
    pub current_map_index: usize,
    pub footstep_index: usize,
    pub footstep_active: bool,
    pub sounds: Vec<Vec<u8>> // Store raw sounds data
}

