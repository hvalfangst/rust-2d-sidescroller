use rodio::Sink;
use crate::graphics::sprites::SpriteMaps;
use crate::state::core_logic::CoreLogic;
use crate::state::{Direction, GameState, Obstacle};
use crate::state::physics::check_collision;
use crate::state::player::Player;

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
