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
    // println!("Fixed x: {}", fixed_x);

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
    draw_sprite(0, 0, &game_state.sprites.layer_0[game_state.layer_0_index], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);

    // Loop through the layers and draw them based on the player's position in relation to the divisor to achieve parallax scrolling
    for (i, divisor) in [16, 6, 6, 4, 1].iter().enumerate() {

        // // Layer 0 will have offset divided by 16, layer 1 by 6, layer 2 by 4, and layer 3 by 1
        let offset_x = game_state.player.x as usize / divisor % texture_width;
        let offset_y = game_state.player.y as usize / 666;

        let layer = match i {
            0 => &game_state.sprites.layer_0[game_state.layer_0_index],
            1 => &game_state.sprites.layer_1[0],
            2 => &game_state.sprites.layer_4[game_state.lighthouse_lights_sprite_index],
            3 => &game_state.sprites.layer_2[0],
            4 => &game_state.sprites.layer_3[game_state.ground_sprite_frame_index],
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

    let fixed_player_x = 109;

    // Draw the obstacles, which have a metal box sprite of 3 different frames based on durability
    game_state.all_maps[game_state.current_map_index].obstacles.iter().enumerate().for_each(|(index, obstacle)| {

        if obstacle.active  {

            let final_x = (obstacle.x_left as isize - game_state.player.x as isize + fixed_player_x as isize)
                .max(0) as usize;
            println!("player.x: {}, fixed player.x: {}, window_width/4 {} obstacle x.left: {}, final x : {}", game_state.player.x, fixed_player_x,  game_state.window_width/4,
                     obstacle.x_left,  final_x);

            // Only draw obstacles that are within the window width
            if final_x < game_state.window_width / 4 && final_x > 0 {

                let metal_box_sprite =
                    if obstacle.durability == 2 {
                        &game_state.sprites.metal_box[0] // undamaged
                    } else if obstacle.durability == 1 {
                        &game_state.sprites.metal_box[1] // slightly damaged
                    } else {
                        &game_state.sprites.metal_box[2] // damaged
                    };


                draw_sprite(final_x, obstacle.y_bottom as usize, metal_box_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
            }
        }
    });

    // Draw the traps
    game_state.all_maps[game_state.current_map_index].traps.iter().enumerate().for_each(|(index, trap)| {
        if trap.active  {

            let final_x = (trap.x_left as isize - game_state.player.x as isize + fixed_player_x as isize) as usize;

            if final_x < game_state.window_width / 4 && final_x > 0 {
                let toxic_trap_sprite = &game_state.sprites.toxic_trap[game_state.toxic_trap_sprite_index];
                draw_sprite(final_x, trap.y_bottom as usize, toxic_trap_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
            }
        }
    });



    // Draw temporary toxic trap right next to the obstacle
    // draw_sprite(obstacle_x_offset + 16.0 as usize, obstacle.y_bottom as usize, toxic_trap_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);

    let heart_sprite_width = game_state.sprites.heart[game_state.heart_sprite_index].width as usize;

    // Assign a triple of indices based on the player's health as there are 3 independent hearts
    let health_triple = match game_state.player.health {
        3 => [0, 0, 0],
        2 => [0, 0, 2],
        1 => [0, 2, 2],
        0 => [2, 2, 2],
        _ => [2, 2, 2],
    };

    for (i, &triple) in health_triple.iter().enumerate() {
        // Only alternate between the heart_sprite index if the triple value is less than 2
        let heart_index = if triple < 2 { game_state.heart_sprite_index } else { triple };

        // Draw the hearts in the top left corner of the screen
        draw_sprite(
            i * (heart_sprite_width + 1),
            0,
            &game_state.sprites.heart[heart_index],
            game_state.window_buffer,
            game_state.all_maps[game_state.current_map_index].width,
        );
    }
}