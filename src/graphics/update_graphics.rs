use crate::graphics::sprites::draw_sprite;

use crate::state::constants::graphics::{FIXED_PLAYER_X, KICK_FRAME_DURATION, LEFT_JUMP_INITIATED, LEFT_JUMP_MID_AIR, RIGHT_JUMP_INITIATED, RIGHT_JUMP_MID_AIR, SHADOW_LARGE, SHADOW_MEDIUM, SHADOW_SMALL};
use crate::state::constants::physics::{GROUND};
use crate::state::structs::Direction::{Left, Right};
use crate::state::structs::GameState;

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
        &game_state.sprites.jump[RIGHT_JUMP_INITIATED]
    } else if game_state.player.almost_ground && !game_state.player.on_obstacle && direction == Left {
        &game_state.sprites.jump[LEFT_JUMP_INITIATED]
    } else if !game_state.player.on_ground && !game_state.player.on_obstacle && direction == Right {
        &game_state.sprites.jump[RIGHT_JUMP_MID_AIR]
    } else if !game_state.player.on_ground && !game_state.player.on_obstacle && direction == Left {
        &game_state.sprites.jump[LEFT_JUMP_MID_AIR]
    } else if direction == Right {
        &game_state.sprites.player[game_state.player.right_increment]
    } else if direction == Left {
        &game_state.sprites.player[game_state.player.left_increment]
    } else { // Default is moving to the right
        &game_state.sprites.player[game_state.player.right_increment]
    };

    // Draw the chosen player sprite
    draw_sprite(
        FIXED_PLAYER_X as usize,
        game_state.player.y as usize - (sprite_to_draw.height - 10) as usize,
        sprite_to_draw,
        game_state.window_buffer,
        game_state.all_maps[game_state.current_map_index].width
    );

    // Draw different sizes of shadows based on player state
    let shadow_sprite = if game_state.player.on_ground {
            &game_state.sprites.shadow[SHADOW_SMALL]
    } else if game_state.player.almost_ground {
            &game_state.sprites.shadow[SHADOW_MEDIUM]
    } else { // Player is in the air
            &game_state.sprites.shadow[SHADOW_LARGE]
    };

    // Draw associated shadow if not on or above obstacle
    if !game_state.player.on_obstacle && !game_state.player.above_obstacle {
        draw_sprite(
            FIXED_PLAYER_X as usize,
            GROUND as usize + 7,
            shadow_sprite,
            game_state.window_buffer,
            game_state.all_maps[game_state.current_map_index].width
        );

    }
}

fn draw_game_world(game_state: &mut GameState) {
    draw_map(game_state);
    draw_obstacles(game_state);
    draw_traps(game_state);
    draw_hearts(game_state);
}

fn draw_map(game_state: &mut GameState) {
    let texture_width = game_state.all_maps[game_state.current_map_index].width;

    // Always draw the static background layer first in order to fill all pixels as the parallax effect can result in empty pixels
    draw_sprite(0, 0, &game_state.sprites.mountains[game_state.mountains_sprite_frame_index], game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);

    // Loop through the layers and draw them based on the player's position in relation to the divisor to achieve parallax scrolling
    for (i, divisor) in [16, 6, 6, 4, 1].iter().enumerate() {

        // // Layer 0 will have offset divided by 16, layer 1 by 6, layer 2 by 4, and layer 3 by 1
        let offset_x = game_state.player.x as usize / divisor % texture_width;
        let offset_y = game_state.player.y as usize / 666;

        let layer = match i {
            0 => &game_state.sprites.mountains[game_state.mountains_sprite_frame_index],
            1 => &game_state.sprites.docks[0],
            2 => &game_state.sprites.lighthouse[game_state.lighthouse_sprite_frame_index],
            3 => &game_state.sprites.sea[0],
            4 => &game_state.sprites.ground[game_state.ground_sprite_frame_index],
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
}

fn draw_obstacles(game_state: &mut GameState) {
    // Draw the obstacles, which have a metal box sprite of 3 different frames based on durability
    game_state.all_maps[game_state.current_map_index].obstacles.iter().enumerate().for_each(|(index, obstacle)| {
        if obstacle.active {
            let relative_x = calculate_relative_x(obstacle.x_left as isize, game_state.player.x as isize);

            println!("player.x: {}, fixed player.x: {}, window_width/4 {} obstacle x.left: {}, final x : {}", game_state.player.x, FIXED_PLAYER_X, game_state.window_width / 4,
                     obstacle.x_left, relative_x);

            // Only draw obstacles that are within the window width
            if relative_x < game_state.window_width / 4 && relative_x > 0 {
                let metal_box_sprite =
                    if obstacle.durability == 2 {
                        &game_state.sprites.metal_box[0] // undamaged
                    } else if obstacle.durability == 1 {
                        &game_state.sprites.metal_box[1] // slightly damaged
                    } else {
                        &game_state.sprites.metal_box[2] // damaged
                    };


                draw_sprite(relative_x, obstacle.y_bottom as usize, metal_box_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
            }
        }
    });
}

fn draw_traps(game_state: &mut GameState) {
    // Draw the traps
    game_state.all_maps[game_state.current_map_index].traps.iter().enumerate().for_each(|(index, trap)| {
        if trap.active {
            let relative_x = calculate_relative_x(trap.x_left as isize, game_state.player.x as isize);

            if relative_x < game_state.window_width / 4 && relative_x > 0 {
                let toxic_trap_sprite = &game_state.sprites.toxic_trap[game_state.toxic_trap_sprite_frame_index];
                draw_sprite(relative_x, trap.y_bottom as usize, toxic_trap_sprite, game_state.window_buffer, game_state.all_maps[game_state.current_map_index].width);
            }
        }
    });
}

fn draw_hearts(game_state: &mut GameState) {
    let heart_sprite_width = game_state.sprites.heart[game_state.heart_sprite_frame_index].width as usize;

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
        let heart_index = if triple < 2 { game_state.heart_sprite_frame_index } else { triple };

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

fn calculate_relative_x(world_object_x: isize, world_player_x: isize) -> usize {
    (world_object_x - world_player_x + FIXED_PLAYER_X).max(0) as usize
}