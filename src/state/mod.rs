use std::io::{BufReader, Cursor};
use std::time::Duration;

use crate::graphics::sprites::SpriteMaps;
use crate::state::player::{Player, PlayerState};
use crate::Tile;
use minifb::Window;
use rodio::Source;
use crate::audio::engine::append_source_source;

pub mod event_loop;
pub mod player;
pub mod core_logic;

const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

const GRAVITY: f32 = 0.5;
pub const JUMP_VELOCITY: f32 = -5.0;
pub const MAX_VELOCITY: f32 = 2.0;
pub const ACCELERATION: f32 = 0.1;
const FRICTION: f32 = 0.2;
pub const GROUND: f32 = 205.0;
const LOWER_BOUND: f32 = 0.0;
const UPPER_BOUND: f32 = 225.0;
pub const KICK_FRAME_DURATION: u32 = 8;

pub const WALK_SOUND_1: usize = 0;
pub const WALK_SOUND_2: usize = 1;
pub const WALK_SOUND_3: usize = 2;
pub const WALK_SOUND_4: usize = 3;
pub const JUMP_SOUND: usize = 4;
const FALL_MILD_SOUND: usize = 5;
const FALL_HEAVY_SOUND: usize = 6;
const DOWN_SOUND: usize = 7;
const EXPLOSION_SOUND: usize = 8;
pub const KICK_SOUND: usize = 9;
pub const KICK_BOX_SOUND: usize = 10;


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
                append_source_source(&game_state, sink, FALL_MILD_SOUND, 2500);
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

