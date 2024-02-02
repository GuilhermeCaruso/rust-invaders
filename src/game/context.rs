use std::collections::HashSet;

use rand::Rng;

use super::domain::{H_SIZE, PIXEL_SIZE, W_SIZE};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn increment(&mut self, x: i32, y: i32) -> &mut Self {
        self.x += x;
        self.y += y;
        self
    }
}

pub enum PlayerDirection {
    Left,
    Right,
}

pub struct GameContext {
    pub player_position: Point,
    pub player_direction: PlayerDirection,

    pub enemies: Vec<Point>,
}

impl GameContext {
    pub fn new() -> Self {
        GameContext {
            player_direction: PlayerDirection::Right,
            player_position: Point::new(
                0,
                (((H_SIZE * PIXEL_SIZE) - PIXEL_SIZE) / PIXEL_SIZE) as i32,
            ),

            enemies: vec![Point::new(0, 0), Point::new(3, 0), Point::new(10, 0)],
        }
    }

    pub fn generate_enemies(&mut self) {
        let max_pos_x = (((W_SIZE * PIXEL_SIZE) - PIXEL_SIZE) / PIXEL_SIZE) as i32;
        let max_enemies = 2;

        for _ in 0..rand::thread_rng().gen_range(1..=max_enemies) {
            self.enemies
                .push(Point::new(rand::thread_rng().gen_range(0..=max_pos_x), 0))
        }
    }

    pub fn move_enemies(&mut self) {
        let mut idx_to_remove: HashSet<usize> = HashSet::new();
        for n in 0..self.enemies.len() {
            self.enemies[n].y += 1;

            if self.enemies[n].y > (((H_SIZE * PIXEL_SIZE) - PIXEL_SIZE) / PIXEL_SIZE) as i32 {
                idx_to_remove.insert(n);
            }
        }

        for i in idx_to_remove.iter() {
            self.enemies.remove(*i);
        }

        self.generate_enemies()
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
