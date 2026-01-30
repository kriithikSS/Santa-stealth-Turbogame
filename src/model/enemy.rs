use turbo::*;
use crate::{Grid, is_wall};
const VISION_RADIUS: f32 = 160.0;
const VISION_ANGLE: f32 = 60.0; // degrees (30° each side)
use crate::model::{find_path, TileType};
use crate::TILE_SIZE;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum EnemyState {
    Idle,
    Chasing,
}

#[turbo::serialize]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub hitbox: Bounds,
    pub state: EnemyState,
    pub alive: bool,
    shoot_cooldown: u32,

    pub facing_angle: f32,

    patrol_dir: (f32, f32), // NEW
    patrol_timer: u32,      // NEW
  
   see_through_walls_timer: u32,

    pub alerted_timer: u32,
    path: Vec<(usize, usize)>,
path_index: usize,
repath_timer: u32,


}



impl Enemy {

  


 pub fn alert(&mut self, target_x: f32, target_y: f32) {
    if !self.alive {
        return;
    }

    self.state = EnemyState::Chasing;
    self.alerted_timer = 180; // 🔥 FORCE chase for 3 seconds

    let dx = target_x - self.x;
    let dy = target_y - self.y;

    self.facing_angle = dy.atan2(dx);
    self.see_through_walls_timer = 180;
}



    pub fn new(x: f32, y: f32) -> Self {
    let angle = random::f32() * std::f32::consts::TAU;

    Self {
        x,
        y,
        hitbox: Bounds::new(x, y, 32, 32),
        state: EnemyState::Idle,
        alive: true,
        shoot_cooldown: 0,

        facing_angle: angle,
        patrol_dir: (angle.cos(), angle.sin()),
        patrol_timer: random::between(30, 120),
        see_through_walls_timer: 0,
            alerted_timer: 0,
            path: Vec::new(),
path_index: 0,
repath_timer: 0,




    }
}


fn can_see_player_strict(&self, player_x: f32, player_y: f32, map: &Grid) -> bool {

    let ex = self.x + 16.0;
    let ey = self.y + 16.0;
    let px = player_x + 16.0;
    let py = player_y + 16.0;

    let dx = px - ex;
    let dy = py - ey;
    let dist = (dx * dx + dy * dy).sqrt();

    // 1. Radius check
    if dist > VISION_RADIUS {
        return false;
    }

    // 2. Angle check
    let angle_to_player = dy.atan2(dx);
    let mut angle_diff = angle_to_player - self.facing_angle;

    // Normalize angle to [-PI, PI]
    while angle_diff > std::f32::consts::PI {
        angle_diff -= std::f32::consts::TAU;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += std::f32::consts::TAU;
    }

    let half_cone = (VISION_ANGLE.to_radians()) / 2.0;
    if angle_diff.abs() > half_cone {
        return false;
    }

    // 3. Line-of-sight (ray step)
    let steps = (dist / 4.0).ceil() as i32;
    if steps <= 0 {
    return true;
}
    let step_x = dx / steps as f32;
    let step_y = dy / steps as f32;

    let mut rx = ex;
    let mut ry = ey;

    for _ in 0..steps {
        if is_wall(map, rx, ry) {
            return false;
        }
        rx += step_x;
        ry += step_y;
    }

    true
}

fn can_see_player_ignore_walls(&self, player_x: f32, player_y: f32) -> bool {
    let ex = self.x + 16.0;
    let ey = self.y + 16.0;
    let px = player_x + 16.0;
    let py = player_y + 16.0;

    let dx = px - ex;
    let dy = py - ey;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist > VISION_RADIUS {
        return false;
    }

    let angle_to_player = dy.atan2(dx);
    let mut angle_diff = angle_to_player - self.facing_angle;

    while angle_diff > std::f32::consts::PI {
        angle_diff -= std::f32::consts::TAU;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += std::f32::consts::TAU;
    }

    let half_cone = (VISION_ANGLE.to_radians()) / 2.0;
    angle_diff.abs() <= half_cone
}




    pub fn update(
    &mut self,
    player_x: f32,
    player_y: f32,
    map: &Grid,
) -> bool {
        if !self.alive {
            return false;
        }
        self.see_through_walls_timer =
        self.see_through_walls_timer.saturating_sub(1);
        self.shoot_cooldown = self.shoot_cooldown.saturating_sub(1);

            // 🚨 ALERT OVERRIDE (force chase even without vision)
    if self.alerted_timer > 0 {
        self.alerted_timer -= 1;
        self.state = EnemyState::Chasing;
    }


        match self.state {
            EnemyState::Idle => {
    if self.can_see_player_strict(player_x, player_y, map) {
        self.state = EnemyState::Chasing;
        
        return false;
    }

    // Patrol
    let speed = 0.6;
    let (dx, dy) = self.patrol_dir;
    self.facing_angle = dy.atan2(dx);

    let try_x = self.x + dx * speed;
    if !is_wall(map, try_x + 16.0, self.y + 16.0) {
        self.x = try_x;
    } else {
        self.patrol_timer = 0;
    }

    let try_y = self.y + dy * speed;
    if !is_wall(map, self.x + 16.0, try_y + 16.0) {
        self.y = try_y;
    } else {
        self.patrol_timer = 0;
    }

    self.patrol_timer = self.patrol_timer.saturating_sub(1);
    if self.patrol_timer == 0 {
        let angle = random::f32() * std::f32::consts::TAU;
        self.patrol_dir = (angle.cos(), angle.sin());
        self.facing_angle = angle;
        self.patrol_timer = random::between(60, 180);
    }}

            EnemyState::Chasing => {
                // Santa just disappeared behind a wall → start grace period
if self.see_through_walls_timer == 0
    && !self.can_see_player_strict(player_x, player_y, map)
{
    self.see_through_walls_timer = 180; // 3 seconds @ 60 FPS
}

    let sees_player = if self.see_through_walls_timer > 0 {
        self.can_see_player_ignore_walls(player_x, player_y)
    } else {
        self.can_see_player_strict(player_x, player_y, map)
    };

    if !sees_player && self.alerted_timer == 0 {
    self.state = EnemyState::Idle;
    return false;
}


    

    self.repath_timer = self.repath_timer.saturating_sub(1);

// Compute tiles
let enemy_tile = world_to_tile(self.x + 16.0, self.y + 16.0);
let player_tile = world_to_tile(player_x + 16.0, player_y + 16.0);

// Repath occasionally OR when alerted
if self.repath_timer == 0 || self.path.is_empty() {
    if let Some(p) = find_path(map, enemy_tile, player_tile) {
        self.path = p;
        self.path_index = 0;
        self.repath_timer = 30; // recalc every 0.5s
    }
}

// Follow path
if self.path_index < self.path.len() {
    let (tx, ty) = self.path[self.path_index];
    let (cx, cy) = tile_center(tx, ty);

    let dx = cx - (self.x + 16.0);
    let dy = cy - (self.y + 16.0);
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < 4.0 {
        self.path_index += 1;
    } else {
        let speed = 1.6;
        let nx = dx / dist;
        let ny = dy / dist;

        let try_x = self.x + nx * speed;
        if !is_wall(map, try_x + 16.0, self.y + 16.0) {
            self.x = try_x;
        }

        let try_y = self.y + ny * speed;
        if !is_wall(map, self.x + 16.0, try_y + 16.0) {
            self.y = try_y;
        }

        self.facing_angle = ny.atan2(nx);
    }
}


    // 🎯 SHOOT IF SANTA IN FRONT
    let ex = self.x + 16.0;
    let ey = self.y + 16.0;
    let px = player_x + 16.0;
    let py = player_y + 16.0;

    let dx = px - ex;
    let dy = py - ey;
    let dist = (dx * dx + dy * dy).sqrt();

    let angle_to_player = dy.atan2(dx);
    let mut diff = angle_to_player - self.facing_angle;

    // normalize
    while diff > std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }

    let in_front = diff.abs() <= (VISION_ANGLE.to_radians() / 2.0);

    if in_front && dist < 120.0 && self.shoot_cooldown == 0 {
        self.shoot_cooldown = 45;

        // 🔫 muzzle flash
        circ!(
            x = ex as i32,
            y = ey as i32,
            d = 6,
            color = 0xffeeaa88
        );

        return true; // tells GameState to spawn bullet
    }

}
//startscreen fast
//

        }

        self.hitbox = self.hitbox.position(self.x, self.y);
        false
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        sprite!(
            "snowman",
            x = self.x as i32,
            y = self.y as i32,
            w = 32,
            h = 32,
            cover = true
        );
        // DEBUG: vision cone
let ex = self.x + 16.0;
let ey = self.y + 16.0;

let left = self.facing_angle - (VISION_ANGLE.to_radians() / 2.0);
let right = self.facing_angle + (VISION_ANGLE.to_radians() / 2.0);

path!(
    start = (ex as i32, ey as i32),
    end = (
        (ex + left.cos() * VISION_RADIUS) as i32,
        (ey + left.sin() * VISION_RADIUS) as i32
    ),
    color = 0xff000088
);

path!(
    start = (ex as i32, ey as i32),
    end = (
        (ex + right.cos() * VISION_RADIUS) as i32,
        (ey + right.sin() * VISION_RADIUS) as i32
    ),
    color = 0xff000088
);

    }
}



fn world_to_tile(x: f32, y: f32) -> (usize, usize) {
    (
        (x as i32 / TILE_SIZE) as usize,
        (y as i32 / TILE_SIZE) as usize,
    )
}

fn tile_center(tx: usize, ty: usize) -> (f32, f32) {
    (
        tx as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
        ty as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
    )
}
