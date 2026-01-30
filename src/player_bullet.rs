use turbo::*;
use crate::{Grid, is_wall};

#[turbo::serialize]
pub struct PlayerBullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub alive: bool,
}

impl PlayerBullet {
    pub fn new(x: f32, y: f32, dir: (f32, f32)) -> Self {
        let speed = 6.0;
        Self {
            x,
            y,
            vx: dir.0 * speed,
            vy: dir.1 * speed,
            alive: true,
        }
    }

    pub fn update(&mut self, map: &Grid) {
        self.x += self.vx;
        self.y += self.vy;

        if is_wall(map, self.x, self.y) {
            self.alive = false;
        }
    }

    pub fn hitbox(&self) -> Bounds {
        Bounds::new(self.x - 4.0, self.y - 4.0, 8, 8)
    }

    pub fn draw(&self) {
        sprite!(
            "gift_gun", // 🔫 bullet sprite
            x = self.x as i32,
            y = self.y as i32,
            w = 16,
            h = 16,
            cover = true
        );
    }
}
