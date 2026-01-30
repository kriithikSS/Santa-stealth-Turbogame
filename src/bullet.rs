use turbo::*;
use crate::{Grid, is_wall};
use crate::player::Player;
const BULLET_SPEED: f32 = 9.0;

#[turbo::serialize]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    prev_x: f32,
    prev_y: f32,
    pub dx: f32,
    pub dy: f32,
    pub alive: bool,
    
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            prev_x: x,
            prev_y: y,
            dx: angle.cos() * BULLET_SPEED,
            dy: angle.sin() * BULLET_SPEED,
            alive: true,
        }
    }

    pub fn update(&mut self, map: &Grid, player: &mut Player) {
        if !self.alive {
            return;
        }

            // store previous position
        self.prev_x = self.x;
        self.prev_y = self.y;

        // move bullet
        self.x += self.dx;
        self.y += self.dy;

        // ❌ hit wall
        if is_wall(map, self.x, self.y) {
            self.alive = false;
            return;
        }
        // 🎯 segment collision (NO tunneling)
        let steps: i32 = BULLET_SPEED as i32; // match bullet speed

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let ix = self.prev_x + (self.x - self.prev_x) * t;
            let iy = self.prev_y + (self.y - self.prev_y) * t;

            let bullet_box = Bounds::new(ix - 2.0, iy - 2.0, 8, 8);

            if bullet_box.intersects(&player.hitbox) {
    self.alive = false;
    player.health -= 1;

    // ❄️ Snow bullet impact sound
    audio::play("bullet_snow");

    return;
}

        }
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        // ➖ tracer
        path!(
            start = (self.x as i32, self.y as i32),
            end = (
                (self.x - self.dx * 2.0) as i32,
                (self.y - self.dy * 2.0) as i32
            ),
            color = 0xffffaa88
        );
    }
}
