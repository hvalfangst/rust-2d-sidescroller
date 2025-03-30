use crate::audio::engine::append_source_source;
use crate::graphics::sprites::SpriteMaps;
use crate::state::player::Player;
use crate::state::Direction::{Left, Right};
use crate::state::{remove_box, GameState, Obstacle, ACCELERATION, JUMP_SOUND, JUMP_VELOCITY, KICK_BOX_SOUND, KICK_SOUND, MAX_VELOCITY, WALK_SOUND_1, WALK_SOUND_2, WALK_SOUND_3, WALK_SOUND_4};
use minifb::{Key, KeyRepeat};
use rodio::Sink;
use std::collections::HashMap;
use std::sync::Arc;

pub fn handle_user_input(game_state: &mut GameState, commands: &InputLogicMap, sink: &mut Sink) -> bool {
    let legal_keys = [Key::Space, Key::D, Key::A, Key::X];
    let mut any_key_pressed = false;
    let mut movement_key_pressed = false;

    for key in legal_keys.iter() {
        if game_state.window.is_key_pressed(*key, KeyRepeat::Yes) {
            any_key_pressed = true;
            if *key == Key::A || *key == Key::D {
                movement_key_pressed = true;
            }
            delegate_command(*key, &commands, game_state, sink);
        }
    }

    // If no legal was pressed, decelerate the player to avoid sliding forever
    if !any_key_pressed {
        decelerate_velocity(game_state);
    }

    any_key_pressed
}

fn delegate_command(key: Key, commands: &InputLogicMap, game_state: &mut GameState, sink: &mut Sink) {
    if let Some(command) = commands.get(&key) {
        command.execute(game_state, sink);
    } else {
        println!("No command associated with key: {:?}", key);
    }
}

pub trait InputLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct MoveLeft;
impl InputLogic for MoveLeft {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity if no collision is detected
        update_velocity(game_state);

        game_state.player.last_key = Some(Key::A);
        game_state.player.direction = Left;

        // Initialize a new field to track the frame count
        game_state.player.left_increment_frame_count += 1;

        if game_state.player.left_increment_frame_count >= 3 {
            game_state.player.left_increment_frame_count = 0; // Reset the frame count

            match game_state.player.left_increment {
                7 => {
                    game_state.player.left_increment = 4;
                }
                _ => {
                    game_state.player.left_increment += 1;
                }
            };
        }

    if game_state.footstep_active {
        play_footstep_sound(game_state, sink);
    }
    }
}

pub struct MoveRight;

impl InputLogic for MoveRight {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity
       update_velocity(game_state);

        game_state.player.last_key = Some(Key::D);
        game_state.player.direction = Right;

        // Initialize a new field to track the frame count
        game_state.player.right_increment_frame_count += 1;

        if game_state.player.right_increment_frame_count >= 3 {
            game_state.player.right_increment_frame_count = 0; // Reset the frame count

            match game_state.player.right_increment {
                3 => {
                    game_state.player.right_increment = 0;
                }
                _ => {
                    game_state.player.right_increment += 1;
                }
            }
        }

        if game_state.footstep_active {
            play_footstep_sound(game_state, sink);
        }

    }
}

pub fn check_collision(obstacles: &Vec<Obstacle>, sprites: &SpriteMaps, player: &Player, is_left: bool) -> (bool, Option<usize>) {
    let mut collision_id: Option<usize> = None;
    println!("----------------------------------------------------------------------");
    let collision = obstacles.iter().enumerate().any(|(index, obstacle)| {
        println!("Checking collision: id: {:?}, x_left: {}, x_right: {}, y_bottom: {}, y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right, obstacle.y_bottom, obstacle.y_top);

        if obstacle.active == false {
            println!("- - - - Obstacle is not active - - - -");
            return false;
        }

        let player_x = if is_left {
            player.x + (sprites.player[player.left_increment].width as f32 / 2.5)
        } else {
            player.x + (sprites.player[player.right_increment].width as f32 / 1.5)
        };

        if player_x > obstacle.x_left && player_x < obstacle.x_right {
            println!("Collision of x axis detected: player_x: {}, obstacle.x_left: {}, obstacle.x_right: {}", player_x, obstacle.x_left, obstacle.x_right);

            if player.y >= obstacle.y_top + 25.0 && player.y <= obstacle.y_bottom + 25.0 {
                collision_id = Some(index);
                println!("Collision detected with obstacle id {:?} x.left {}, x.right: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right , obstacle.y_bottom + 25.0, obstacle.y_top + 25.0);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if let Some(id) = collision_id {
    }

    (collision, collision_id)
}

pub struct Jump;

impl InputLogic for Jump {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        if !game_state.player.is_jumping && (game_state.player.on_ground || game_state.player.on_obstacle) {
            game_state.player.vy = JUMP_VELOCITY;
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = true;
            game_state.player.last_key = Some(Key::Space);

            append_source_source(&game_state, sink, JUMP_SOUND, 1500);
        }
    }
}

pub struct Kick;

impl InputLogic for Kick {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        game_state.player.is_kicking = true;
        game_state.player.kick_frame = 0;
        game_state.player.kick_frame_timer = 0;

        let (collision, id) = check_collision(game_state.all_maps[game_state.current_map_index].obstacles, &game_state.sprites, &game_state.player, game_state.player.direction == Left);

        // Check if the player is adjacent to an obstacle
        if collision {
            append_source_source(&game_state, sink, KICK_BOX_SOUND, 1000);

            if game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability > 0 {
                game_state.all_maps[game_state.current_map_index].obstacles[id.unwrap()].durability -= 1;
            } else {

                remove_box(game_state, id.unwrap(), sink);
            }

        } else {
            append_source_source(&game_state, sink, KICK_SOUND, 1000);
        }
    }
}



fn play_footstep_sound(game_state: &mut GameState, sink: &mut Sink) {
    if game_state.footstep_index == 4 { game_state.footstep_index = 0; } else { game_state.footstep_index += 1; }

    let sound_index = match game_state.footstep_index {
        0 => WALK_SOUND_1,
        1 => WALK_SOUND_2,
        2 => WALK_SOUND_3,
        _ => WALK_SOUND_4,
    };

    append_source_source(&game_state, sink, sound_index, 200);
}

pub type InputLogicMap = HashMap<Key, Arc<dyn InputLogic>>;

pub fn initialize_input_logic_map() -> InputLogicMap {
    let mut logic_map: InputLogicMap = HashMap::new();

    logic_map.insert(Key::A, Arc::new(MoveLeft));
    logic_map.insert(Key::D, Arc::new(MoveRight));
    logic_map.insert(Key::Space, Arc::new(Jump));
    logic_map.insert(Key::X, Arc::new(Kick));

    logic_map
}

pub fn update_velocity(game_state: &mut GameState) {
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

fn decelerate_velocity(game_state: &mut GameState) {
    game_state.player.vx *= 0.95;
    if game_state.player.vx.abs() < 0.1 {
        game_state.player.vx = 0.0;
    }
}
