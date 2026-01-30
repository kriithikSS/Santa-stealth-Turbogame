use turbo::*;
use crate::{Grid, is_wall};
use crate::model::Enemy;

#[turbo::serialize]
pub struct Snowball {
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    pub alive: bool,
}

impl Snowball {
    pub fn new(x: f32, y: f32, dir: (f32, f32)) -> Self {
        Self {
            x,
            y,
            dx: dir.0 * 6.0,
            dy: dir.1 * 6.0,
            alive: true,
        }
    }

    pub fn update(&mut self, map: &Grid, enemies: &mut Vec<Enemy>) {
        if !self.alive {
            return;
        }

        self.x += self.dx;
        self.y += self.dy;

        // ❌ Wall hit
        if is_wall(map, self.x, self.y) {
            self.alive = false;
            return;
        }

        let hitbox = Bounds::new(self.x, self.y, 6, 6);

        // 🎯 Enemy hit
        for enemy in enemies.iter_mut() {
            if enemy.alive && hitbox.intersects(&enemy.hitbox) {
                enemy.alive = false;
                self.alive = false;

                audio::play("snow_hit");
                return;
            }
        }
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        // ❄ Simple white snowball
        circ!(
            x = self.x as i32,
            y = self.y as i32,
            d = 6,
            color = 0xffffffff
        );
    }
}
