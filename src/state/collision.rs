use std::thread::sleep;
use crate::state::core_logic::CoreLogic;
use rodio::Sink;
use crate::graphics::sprites::SpriteMaps;
use crate::state::player::Player;
use crate::state::structs::{Direction, GameState, Obstacle};

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

pub fn check_collision(obstacles: &Vec<Obstacle>, sprites: &SpriteMaps, player: &Player, is_left: bool) -> (bool, Option<usize>) {
    let mut collision_id: Option<usize> = None;
    // println!("----------------------------------------------------------------------");
    let collision = obstacles.iter().enumerate().any(|(index, obstacle)| {
        // println!("Checking collision: _id: {:?}, x_left: {}, x_right: {}, y_bottom: {}, y_top: {}", obstacle._id, obstacle.x_left, obstacle.x_right, obstacle.y_bottom, obstacle.y_top);

        if obstacle.active == false {
            // println!("- - - - Obstacle is not active - - - -");
            return false;
        }

        let player_x = if is_left {
            player.x + (sprites.player[player.left_increment].width as f32 / 2.5)
        } else {
            player.x + (sprites.player[player.right_increment].width as f32 / 1.5)
        };

        if player_x > obstacle.x_left && player_x < obstacle.x_right {
            // println!("Player y: {}, obs.y_bottom: {}, obs.y_top: {}", player.y, obstacle.y_bottom, obstacle.y_top);

            if player.y >= obstacle.y_top + 10.0 && player.y <= obstacle.y_bottom + 25.0 {
                // println!("Collision of x detected: p_x: {}, obs.x_left: {}, obs.x_right: {}", player_x, obstacle.x_left, obstacle.x_right);

                collision_id = Some(index);
                // println!("Collision detected with obstacle _id {:?} x.left {}, x.right: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", obstacle._id, obstacle.x_left, obstacle.x_right , obstacle.y_bottom + 25.0, obstacle.y_top + 25.0);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if let Some(_id) = collision_id {
    }

    (collision, collision_id)
}

