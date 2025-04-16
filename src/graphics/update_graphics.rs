use crate::graphics::sprites::draw_sprite;
use crate::state::Direction::{Left, Right};
use crate::state::*;

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_game_world(game_state);
    draw_player(game_state)
}



fn draw_player(game_state: &mut GameState) {

    // Determine the current direction and action of the player
    let direction = game_state.player.direction;

    // Determine the sprite to draw
    let sprite_to_draw =

    if game_state.player.is_kicking {
        game_state.player.kick_frame_timer += 1;
        if game_state.player.kick_frame_timer >= KICK_FRAME_DURATION as usize {
            game_state.player.kick_frame += 1;
            game_state.player.kick_frame_timer = 0;

            if game_state.player.kick_frame >= 2 {
                game_state.player.is_kicking = false;
                game_state.player.kick_frame = 0;
            }
        }

        // Select the correct kick frame based on direction
        if direction == Right {
            &game_state.sprites.kick[game_state.player.kick_frame]
        } else {
            &game_state.sprites.kick[2 + game_state.player.kick_frame]
        }
    }
    else if game_state.player.almost_ground && !game_state.player.on_obstacle && direction == Right {
        &game_state.sprites.jump[1]
    } else if game_state.player.almost_ground && !game_state.player.on_obstacle && direction == Left {
        &game_state.sprites.jump[4]
    } else if !game_state.player.on_ground && !game_state.player.on_obstacle && direction == Right {
        &game_state.sprites.jump[2]
    } else if !game_state.player.on_ground && !game_state.player.on_obstacle && direction == Left {
        &game_state.sprites.jump[5]
    } else if direction == Right {
        &game_state.sprites.player[game_state.player.right_increment]
    } else if direction == Left {
        &game_state.sprites.player[game_state.player.left_increment]
    } else { // Default is moving to the right
        &game_state.sprites.player[game_state.player.right_increment]
    };


    let fixed_x = (100.0) as usize; // Fixed x-coordinate for the player

    // Draw the chosen player sprite
    draw_sprite(
        fixed_x,
        game_state.player.y as usize - (sprite_to_draw.height - 10) as usize,
        sprite_to_draw,
        game_state.window_buffer,
        game_state.all_maps[game_state.current_map_index].width
    );

    // Draw different sizes of shadows based on player state
    let shadow_sprite = if game_state.player.on_ground {
            &game_state.sprites.shadow[0]
    } else if game_state.player.almost_ground {
            &game_state.sprites.shadow[2]
    } else { // Player is in the air
            &game_state.sprites.shadow[1]
    };

    // Draw associated shadow if not on or above obstacle
    // if !game_state.player.on_obstacle && !game_state.player.above_obstacle {
    //     draw_sprite(
    //         game_state.player.x as usize,
    //         GROUND as usize + 3,
    //         shadow_sprite,
    //         game_state.window_buffer,
    //         game_state.all_maps[game_state.current_map_index].width
    //     );

    // }
}

fn draw_game_world(game_state: &mut GameState) {
    let texture_width = game_state.all_maps[game_state.current_map_index].width;

    draw_sprite(0, 0, &game_state.sprites.layer_0[0], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);


    for (i, divisor) in [16, 6, 4, 1].iter().enumerate() {
        let offset_x = game_state.player.x as usize / divisor % texture_width;
        let offset_y = game_state.player.y as usize / 666;

        let layer = match i {
            0 => &game_state.sprites.layer_0[0],
            1 => &game_state.sprites.layer_1[0],
            2 => &game_state.sprites.layer_2[0],
            3 => &game_state.sprites.layer_3[0],
            _ => unreachable!(),
        };


        draw_sprite(
            (game_state.window_width).saturating_sub(offset_x),
            offset_y,
            layer,
            game_state.window_buffer,
            game_state.all_maps[game_state.current_map_index].width,
        );
    }


    // // Draw the obstacles, which in this case are metal boxes that have 3 different sprites based on durability
    // game_state.all_maps[game_state.current_map_index].obstacles.iter().enumerate().for_each(|(index, obstacle)| {
    //     if obstacle.active {
    //         let metal_box_sprite =
    //         if obstacle.durability == 2 {
    //             &game_state.sprites.metal_box[0] // undamaged
    //         } else if obstacle.durability == 1 {
    //             &game_state.sprites.metal_box[1] // slightly damaged
    //         } else {
    //             &game_state.sprites.metal_box[2] // damaged
    //         };
    //
    //         draw_sprite(obstacle.x_left as usize, obstacle.y_bottom as usize, metal_box_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
    //     }
    // });

}