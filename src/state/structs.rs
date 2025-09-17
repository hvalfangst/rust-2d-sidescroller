use std::time::Instant;
use minifb::Window;
use crate::graphics::sprites::SpriteMaps;
use crate::state::player::Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObstacleId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyId(pub usize);

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub id: ObstacleId,
    pub x_left: f32, // left x coordinate of the box (lower x value)
    pub x_right: f32, // right x coordinate of the box (higher x value)
    pub y_top: f32, // top y coordinate of the box (lower y value)
    pub y_bottom: f32, // bottom y coordinate of the box (higher y value)
    pub velocity_y: f32, // For gravity
    pub falling: bool,   // Whether it's falling
    pub active: bool,    // If false, box is removed
    pub durability: u8,  // Health of the box
    pub is_bottom_obstacle: bool, // Whether it's a bottom obstacle
    pub is_top_obstacle: bool,   // Whether it's a top obstacle
    pub is_leftmost_obstacle: bool, // Whether it's the leftmost obstacle
    pub is_rightmost_obstacle: bool, // Whether it's the rightmost obstacle
    pub left_obstacle: Option<ObstacleId>, // Id of the left obstacle
    pub right_obstacle: Option<ObstacleId>, // Id of the right obstacle
    pub over_obstacle: Option<ObstacleId>, // Id of the obstacle above
    pub under_obstacle: Option<ObstacleId> // Id of the obstacle below
}

pub struct Map<'a> {
    pub id: usize, // Unique identifier for the map
    pub width: usize, // Width of the map
    pub height: usize, // Height of the map
    pub obstacles: &'a mut Vec<Obstacle>, // Obstacles for the map
    pub transition_x: Option<f32>, // X-coordinate for map transition
}

pub struct GameState<'a> {
    pub player: Player, // Player object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: &'a mut Window, // Window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub game_over_index: usize, // Game over index
    pub all_maps: Vec<Map<'a>>, // All maps
    pub current_map_index: usize, // Current map index
    pub footstep_index: usize, // Footstep index
    pub footstep_active: bool, // Footstep active
    pub sounds: Vec<Vec<u8>>,  // Sounds
    pub heart_sprite_frame_index: usize, // Index for the heart sprite animation frame
    pub lighthouse_sprite_frame_index: usize, // Index for the lighthouse sprite animation frame
    pub ground_sprite_frame_index: usize, // Index for the ground sprite animation frame
    pub mountains_sprite_frame_index: usize, // Index for the mountains sprite animation frame
    pub last_heart_sprite_frame_index_change: Instant, // Timestamp of the last heart sprite frame change
    pub last_ground_sprite_frame_index_change: Instant, // Timestamp of the last ground sprite frame change
    pub last_light_house_sprite_frame_index_change: Instant, // Timestamp of the last lighthouse sprite frame change
    pub obstacle_spawned: bool, // Indicates if an obstacle has been spawned
    pub designated_x: f32, // X-coordinate for the player to converge to
    pub damage_taken: bool, // Indicates if the player has taken damage
}