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


    // Calculate the fixed x position for the player sprite, which is to be centered in the window
    let fixed_x = (game_state.window_width / 8) - (sprite_to_draw.width as usize / 2);

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
    if !game_state.player.on_obstacle && !game_state.player.above_obstacle {
        draw_sprite(
            fixed_x,
            GROUND as usize + 7,
            shadow_sprite,
            game_state.window_buffer,
            game_state.all_maps[game_state.current_map_index].width
        );

    }
}

fn draw_game_world(game_state: &mut GameState) {
    let texture_width = game_state.all_maps[game_state.current_map_index].width;

    // Always draw the static background layer first in order to fill all pixels as the parallax effect can result in empty pixels
    draw_sprite(0, 0, &game_state.sprites.layer_0[0], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);

    // Loop through the layers and draw them based on the player's position in relation to the divisor to achieve parallax scrolling
    for (i, divisor) in [16, 6, 6, 4, 1].iter().enumerate() {

        // // Layer 0 will have offset divided by 16, layer 1 by 6, layer 2 by 4, and layer 3 by 1
        let offset_x = game_state.player.x as usize / divisor % texture_width;

        // let target_offset_x = if game_state.player.direction == Right {
        //     game_state.player.x as usize / divisor
        // } else {
        //     (game_state.player.x as usize / divisor).saturating_sub(30) % texture_width
        // };
        //
        // // Gradually adjust the offset_x towards the target_offset_x
        // let offset_x = (game_state.previous_offset_x as f32 * 0.8 + target_offset_x as f32 * 0.125) as usize % texture_width;
        // game_state.previous_offset_x = offset_x;

        let offset_y = game_state.player.y as usize / 666;

        let layer = match i {
            0 => &game_state.sprites.layer_0[0],
            1 => &game_state.sprites.layer_1[0],
            2 => &game_state.sprites.layer_4[game_state.layer_4_sprite_index],
            3 => &game_state.sprites.layer_2[0],
            4 => &game_state.sprites.layer_3[0],
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

    // Calculate the horizontal offset for the obstacles based on the player's position at a fixed distance
    let obstacle_x_offset = (game_state.window_width / 2)
        .saturating_sub(game_state.player.x as usize + game_state.sprites.player[0].width as usize / 2)
        .saturating_sub(160);

    // Draw the obstacles, which in this case are metal boxes that have 3 different sprites based on durability
    game_state.all_maps[game_state.current_map_index].obstacles.iter().enumerate().for_each(|(index, obstacle)| {
        if obstacle.active && (obstacle.x_left - game_state.player.x).abs() < 110.0 {
            let metal_box_sprite =
                if obstacle.durability == 2 {
                    &game_state.sprites.metal_box[0] // undamaged
                } else if obstacle.durability == 1 {
                    &game_state.sprites.metal_box[1] // slightly damaged
                } else {
                    &game_state.sprites.metal_box[2] // damaged
                };

            draw_sprite(obstacle_x_offset, obstacle.y_bottom as usize, metal_box_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
        }
    });

    let heart_sprite_width = game_state.sprites.heart[game_state.heart_sprite_index].width as usize;
    // Draw the three hearts in the top left corner of the screen
    draw_sprite(0, 0, &game_state.sprites.heart[game_state.heart_sprite_index], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
    draw_sprite(heart_sprite_width + 1, 0, &game_state.sprites.heart[game_state.heart_sprite_index], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
    draw_sprite((heart_sprite_width * 2) + 2, 0, &game_state.sprites.heart[game_state.heart_sprite_index], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);



}