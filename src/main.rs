use std::fs::File;
use std::io;
use std::io::{BufRead, Read};
use std::path::Path;
use minifb::{Window, WindowOptions};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::state::player::Player;
use crate::state::{GameState, Map, Obstacle, ObstacleId};
use crate::{
    graphics::sprites::SpriteMaps,
    state::core_logic::initialize_core_logic_map,
    state::event_loop::start_event_loop,
};
use rodio::{OutputStream, Sink};
use input::handler::initialize_input_logic_map;
use crate::graphics::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};

mod state;
mod graphics;
mod audio;
mod input;

fn main() {
    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();

    let sprites = SpriteMaps::new();
    let mut player = Player::new(1.0, 176.0);
    let input_logic = initialize_input_logic_map();
    let core_logic = initialize_core_logic_map();

    let (mut map_one_tiles, map_one_width, map_one_height) = read_grid_from_file("assets/maps/map_one.txt").expect("Failed to read grid from file");
    let (mut map_two_tiles, _map_two_width, _map_two_height) = read_grid_from_file("assets/maps/map_two.txt").expect("Failed to read grid from file");
    let (mut map_three_tiles, map_two_width, map_two_height) = read_grid_from_file("assets/maps/map_three.txt").expect("Failed to read grid from file");
    let mut map_one_obstacles = extract_obstacles(&map_one_tiles, false);
    let mut map_two_obstacles = extract_obstacles(&map_two_tiles, false);
    let mut map_three_obstacles = extract_obstacles(&map_three_tiles, false);

    print_obstacles(&mut map_one_obstacles, "1");
    print_obstacles(&mut map_two_obstacles, "2");
    print_obstacles(&mut map_three_obstacles, "3");

    let fullscreen = false;

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle =  EventLoop::new().primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Age of Panda",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; map_one_width * map_one_height];
    let mut scaled_buffer = vec![0; window_width * window_height];

    let map_one = Map {
        id: 1,
        tiles: map_one_tiles,
        obstacles: &mut map_one_obstacles,
        width: map_one_width,
        height: map_one_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let map_two = Map {
        id: 2,
        tiles: map_two_tiles,
        obstacles: &mut map_two_obstacles,
        width: map_two_width,
        height: map_two_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let map_three = Map {
        id: 3,
        tiles: map_three_tiles,
        obstacles: &mut map_three_obstacles,
        width: map_two_width,
        height: map_two_height,
        starting_x: 0.0,
        starting_y: 0.0,
        transition_x: 200.0,
        transition_y: 0.0
    };

    let all_maps = vec![map_one, map_two, map_three];

    let sounds: Vec<Vec<u8>> = load_sounds();

    let game_state = GameState {
        player,
        sprites,
        window_buffer: &mut window_buffer,
        grass_sprite_index: 0,
        sky_sprite_index: 0,
        window_width,
        window_height,
        window: &mut window,
        scaled_buffer: &mut scaled_buffer,
        game_over_index: 0,
        all_maps,
        current_map_index: 0,
        footstep_index: 0,
        footstep_active: false,
        sounds
    };

    start_event_loop(game_state, input_logic, core_logic, &mut sink);
}

fn print_obstacles(map_one_obstacles: &mut Vec<Obstacle>, id: &str) {
    println!("------------------------- {id} -----------------------------");
    // output obstacle one info for debugging
    for obstacle in map_one_obstacles {
        println!(
            "Obstacle one info: ID: {:?}, X.LEFT: {}, X.RIGHT: {}, Y.BOTTOM: {}, Y.TOP: {}, ACTIVE: {}, DURABILITY: {}, FALLING: {}, VELOCITY_Y: {}, LEFT_OBSTACLE: {:?}, RIGHT_OBSTACLE: {:?}, OVER_OBSTACLE: {:?}, UNDER_OBSTACLE: {:?}, IS_BOTTOM_OBSTACLE: {}, IS_TOP_OBSTACLE: {}, IS_LEFTMOST_OBSTACLE: {}, IS_RIGHTMOST_OBSTACLE: {}",
            obstacle.id,
            obstacle.x_left,
            obstacle.x_right,
            obstacle.y_bottom,
            obstacle.y_top,
            obstacle.active,
            obstacle.durability,
            obstacle.falling,
            obstacle.velocity_y,
            obstacle.left_obstacle,
            obstacle.right_obstacle,
            obstacle.over_obstacle,
            obstacle.under_obstacle,
            obstacle.is_bottom_obstacle,
            obstacle.is_top_obstacle,
            obstacle.is_leftmost_obstacle,
            obstacle.is_rightmost_obstacle
        );
    }
}

fn load_sound(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open sounds file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    buffer
}


fn load_sounds() -> Vec<Vec<u8>> {
    let mut sounds = Vec::new();
    sounds.push(load_sound("assets/sounds/walk_1.wav"));
    sounds.push(load_sound("assets/sounds/walk_2.wav"));
    sounds.push(load_sound("assets/sounds/walk_3.wav"));
    sounds.push(load_sound("assets/sounds/walk_4.wav"));
    sounds.push(load_sound("assets/sounds/jump.wav"));
    sounds.push(load_sound("assets/sounds/fall_mild.wav"));
    sounds.push(load_sound("assets/sounds/fall_heavy.wav"));
    sounds.push(load_sound("assets/sounds/down.wav"));
    sounds.push(load_sound("assets/sounds/explosion.wav"));
    sounds.push(load_sound("assets/sounds/kick.wav"));
    sounds.push(load_sound("assets/sounds/kick_box.wav"));
    sounds
}

fn read_grid_from_file(filename: &str) -> io::Result<(Vec<Tile>, usize, usize)> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?.trim().to_string();
        for (x, c) in line.split_whitespace().enumerate() {
            let x_left = x as f32 * 16.0;
            let x_right = x_left + 16.0;
            let y_bottom = y as f32 * 16.0;
            let y_top = y_bottom - 16.0;
            let tile_type = match c {
                "X" => TileType::Obstacle,
                "G" => TileType::Grass,
                "O" => TileType::Sky,
                _ => TileType::Unknown,
            };
            grid.push(Tile {
                tile_type,
                x_left,
                x_right,
                y_bottom,
                y_top,
            });
        }
    }

    // Automatically detect resolution based on grid size
    let (width, height) = if !grid.is_empty() {
        let width = grid.iter().map(|tile| tile.x_right).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0) as usize;
        let height = grid.iter().map(|tile| tile.y_bottom).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0) as usize;

        println!("Detected resolution: {}x{}", width, height);
        (width, height)
    } else {
        (0, 0)
    };

    Ok((grid, width, height))
}

fn extract_obstacles(grid: &Vec<Tile>, sort_by_y: bool) -> Vec<Obstacle> {
    let mut obstacles = Vec::new();

    for tile in grid {
        if let TileType::Obstacle = tile.tile_type {
            obstacles.push(Obstacle {
                id: ObstacleId(obstacles.len()),
                x_left: tile.x_left,
                x_right: tile.x_right,
                y_bottom: tile.y_bottom,
                y_top: tile.y_top,
                active: true,
                durability: 2,
                falling: false,
                velocity_y: 0.0,
                left_obstacle: None,
                right_obstacle: None,
                over_obstacle: None,
                under_obstacle: None,
                is_bottom_obstacle: false,
                is_top_obstacle: false,
                is_leftmost_obstacle: false,
                is_rightmost_obstacle: false,
            });
        }
    }

    // Determine obstacle relationships and positions
    for i in 0..obstacles.len() {
        let mut is_bottom = true;
        let mut is_top = true;
        let mut is_leftmost = true;
        let mut is_rightmost = true;

        for j in 0..obstacles.len() {
            if i != j {
                if obstacles[j].x_left < obstacles[i].x_right && obstacles[j].x_right > obstacles[i].x_left {
                    if obstacles[j].y_bottom > obstacles[i].y_bottom {
                        obstacles[i].under_obstacle = Some(obstacles[j].id);
                        is_bottom = false;
                    }
                    if obstacles[j].y_top < obstacles[i].y_top {
                        obstacles[i].over_obstacle = Some(obstacles[j].id);
                        is_top = false;
                    }
                }
                if obstacles[j].y_bottom < obstacles[i].y_top && obstacles[j].y_top > obstacles[i].y_bottom {
                    if obstacles[j].x_right < obstacles[i].x_left {
                        obstacles[i].left_obstacle = Some(obstacles[j].id);
                        is_leftmost = false;
                    }
                    if obstacles[j].x_left > obstacles[i].x_right {
                        obstacles[i].right_obstacle = Some(obstacles[j].id);
                        is_rightmost = false;
                    }
                }
            }
        }

        obstacles[i].is_bottom_obstacle = is_bottom;
        obstacles[i].is_top_obstacle = is_top;
        obstacles[i].is_leftmost_obstacle = is_leftmost;
        obstacles[i].is_rightmost_obstacle = is_rightmost;


        // Ensure the obstacle is marked as both top and bottom if it is alone
        if is_bottom && is_top {
            obstacles[i].is_bottom_obstacle = true;
            obstacles[i].is_top_obstacle = true;
        }
    }

    obstacles
}

#[derive(Debug)]
pub enum TileType {
    Obstacle,
    Grass,
    Sky,
    Unknown,
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    x_left: f32,
    x_right: f32,
    y_bottom: f32,
    y_top: f32,
}