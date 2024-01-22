use super::domain::{H_SIZE, PIXEL_SIZE, W_SIZE};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

pub enum PlayerDirection {
    Left,
    Right,
}

pub struct GameContext {
    pub player_position: Point,
    pub player_direction: PlayerDirection,
}

impl GameContext {
    pub fn new() -> Self {
        GameContext {
            player_direction: PlayerDirection::Right,
            player_position: Point::new(
                0,
                (((H_SIZE * PIXEL_SIZE) - PIXEL_SIZE) / PIXEL_SIZE) as i32,
            ),
        }
    }

    pub fn move_player(&mut self, direction: PlayerDirection) {
        match direction {
            PlayerDirection::Left => {
                if self.player_position.x > 0 {
                    self.player_position.x -= 1
                }
            }

            PlayerDirection::Right => {
                if self.player_position.x
                    < (((W_SIZE * PIXEL_SIZE) - PIXEL_SIZE) / PIXEL_SIZE) as i32
                {
                    self.player_position.x += 1
                }
            }
        }
    }

    pub fn tick(&mut self) {}
}
