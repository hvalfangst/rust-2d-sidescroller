pub mod graphics {
    use std::time::Duration;

    pub const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
    pub const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

    pub const SCALED_WINDOW_WIDTH: usize = 960;
    pub const SCALED_WINDOW_HEIGHT: usize = 540;
    pub const TILE_WIDTH: usize = 16;
    pub const TILE_HEIGHT: usize = 16;

    pub const FIXED_PLAYER_X: isize = 109;

    pub const KICK_FRAME_DURATION: u32 = 8;

    pub const RIGHT_JUMP_INITIATED: usize = 1;
    pub const RIGHT_JUMP_MID_AIR: usize = 2;
    pub const LEFT_JUMP_INITIATED: usize = 4;
    pub const LEFT_JUMP_MID_AIR: usize = 5;

    pub const SHADOW_SMALL: usize = 0;
    pub const SHADOW_LARGE: usize = 1;
    pub const SHADOW_MEDIUM: usize = 2;
}

pub mod physics {
    pub const GRAVITY: f32 = 0.5;
    pub const JUMP_VELOCITY: f32 = -5.0;
    pub const MAX_VELOCITY: f32 = 2.0;
    pub const ACCELERATION: f32 = 0.1;
    pub const FRICTION: f32 = 0.2;
    pub const GROUND: f32 = 205.0;
    pub const LOWER_BOUND: f32 = 0.0;
    pub const UPPER_BOUND: f32 = 225.0;
}

pub mod audio {
    pub const WALK_SOUND_1: usize = 0;
    pub const WALK_SOUND_2: usize = 1;
    pub const WALK_SOUND_3: usize = 2;
    pub const WALK_SOUND_4: usize = 3;
    pub const JUMP_SOUND: usize = 4;
    pub const FALL_MILD_SOUND: usize = 5;
    pub const FALL_HEAVY_SOUND: usize = 6;
    pub const DOWN_SOUND: usize = 7;
    pub const EXPLOSION_SOUND: usize = 8;
    pub const KICK_SOUND: usize = 9;
    pub const KICK_BOX_SOUND: usize = 10;
}

