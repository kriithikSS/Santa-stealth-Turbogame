use turbo::*;
use crate::{Grid, is_wall, TILE_SIZE};
use crate::model::{TileType, find_nearest_floor};

use crate::player::Player;
use crate::model::find_path;

const SPRITE_SIZE: f32 = 64.0;
const COLLISION_SIZE: f32 = 28.0;
const COLLISION_OFFSET: f32 = (SPRITE_SIZE - COLLISION_SIZE) / 2.0;
const ATTACK_DURATION: u32 = 35; // frames to fully play NightBorne_attack.gif
const ATTACK_RANGE: f32 = 50.0; // must match your intended melee reach
const ATTACK_IMPACT_FRAME: u32 = 20;


#[turbo::serialize]
#[derive(PartialEq)]
pub enum BossState {
    Idle,
    Chasing,
    Attacking,
    Hurt,
    Dead,
}

#[turbo::serialize]
pub struct Boss {
    pub x: f32,
    pub y: f32,
    pub hitbox: Bounds,

    pub state: BossState,
    pub health: i32,
    pub max_health: i32,

    pub facing_x: f32,
    pub attack_cooldown: u32,
    pub hurt_timer: u32,
    wall_follow_dir: (f32, f32),
    wall_follow_timer: u32,
    path: Vec<(usize, usize)>,
    path_index: usize,
    repath_timer: u32,
    death_timer: u32,
    pub can_take_damage: bool,
    pub attack_timer: u32, 
}

impl Boss {


   
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            hitbox: Bounds::new(
    x + COLLISION_OFFSET,
    y + COLLISION_OFFSET,
    COLLISION_SIZE as u32,
    COLLISION_SIZE as u32,
   

),



            state: BossState::Idle,
            health: 120,
            max_health: 120,

            facing_x: 1.0,
            attack_cooldown: 0,
            hurt_timer: 0,
            wall_follow_dir: (1.0, 0.0),
            wall_follow_timer: 0,
            path: Vec::new(),
            path_index: 0,
            repath_timer: 1,
            death_timer: 0,
            can_take_damage: true, // boss starts vulnerable
            attack_timer: 0,




        }
    }

    pub fn update(&mut self, player: &mut Player, map: &Grid) {
        if self.state == BossState::Dead {
    self.death_timer = self.death_timer.saturating_sub(1);
    return;
}


        self.attack_cooldown = self.attack_cooldown.saturating_sub(1);
        self.hurt_timer = self.hurt_timer.saturating_sub(1);

        // ─── HANDLE ATTACK ANIMATION (FULL COMMIT) ───
if self.state == BossState::Attacking {
    if self.attack_timer > 0 {
        self.attack_timer -= 1;

       // 💥 ATTACK IMPACT (MID ANIMATION)
if self.attack_timer == ATTACK_IMPACT_FRAME {
    let dx = (player.x + 16.0) - (self.x + SPRITE_SIZE / 2.0);
    let dy = (player.y + 16.0) - (self.y + SPRITE_SIZE / 2.0);
    let dist = (dx * dx + dy * dy).sqrt();

    if dist <= ATTACK_RANGE {
        player.health -= 1;
    }
}

// 🧘 END OF ATTACK (RECOVERY ONLY)
if self.attack_timer == 0 {
    self.can_take_damage = true;
    self.state = BossState::Chasing;
}



        // 🩸 ATTACK LUNGE — boss slides forward during attack
let dx = self.facing_x;
let lunge_speed = if self.health < self.max_health / 2 { 2.8 } else { 2.0 };

let try_x = self.x + dx * lunge_speed;
if self.can_move(
    map,
    try_x + COLLISION_OFFSET,
    self.y + COLLISION_OFFSET,
) {
    self.x = try_x;
}

// Update hitbox during attack
self.hitbox = Bounds::new(
    self.x + COLLISION_OFFSET,
    self.y + COLLISION_OFFSET,
    COLLISION_SIZE as u32,
    COLLISION_SIZE as u32,
);

return;

    }
}


        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // 🔥 Enrage phase when boss below 50% HP
let enraged = self.health < self.max_health / 2;


        // Face Santa
        if dx.abs() > 1.0 {
            self.facing_x = dx.signum();
        }

        // MOVE TOWARDS SANTA (SNOWMAN-STYLE RESOLUTION)

self.repath_timer = self.repath_timer.saturating_sub(1);

let boss_cx = self.x + COLLISION_OFFSET + COLLISION_SIZE / 2.0;
let boss_cy = self.y + COLLISION_OFFSET + COLLISION_SIZE / 2.0;

let player_cx = player.x + 16.0;
let player_cy = player.y + 16.0;

let mut boss_tile = world_to_tile(boss_cx, boss_cy);
let mut player_tile = world_to_tile(player_cx, player_cy);

// Clamp tiles if they landed inside walls
if map[boss_tile.1][boss_tile.0].tile_type == TileType::Wall {
    boss_tile = find_nearest_floor(map, boss_tile).unwrap();
}
if map[player_tile.1][player_tile.0].tile_type == TileType::Wall {
    player_tile = find_nearest_floor(map, player_tile).unwrap();
}


// Recalculate path periodically
if self.repath_timer == 0 || self.path.is_empty() {
    if let Some(new_path) = find_path(map, boss_tile, player_tile) {
        self.path = new_path;
        self.path_index = 0;
        self.repath_timer = 30;
    }
}

// Follow the path
if self.path_index < self.path.len() {
    let (tx, ty) = self.path[self.path_index];
    let (target_x, target_y) = tile_target(tx, ty);


let cx = self.x + COLLISION_OFFSET + COLLISION_SIZE / 2.0;
let cy = self.y + COLLISION_OFFSET + COLLISION_SIZE / 2.0;

let dx = target_x - cx;
let dy = target_y - cy;

    let dist = (dx * dx + dy * dy).sqrt();

  if dist < 8.0 {

    self.path_index += 1;
} else {
    let speed = 2.1;
    let step_x = dx / dist * speed;
    let step_y = dy / dist * speed;

  if self.can_move(
    map,
    self.x + step_x + COLLISION_OFFSET,
    self.y + COLLISION_OFFSET,
) {
    self.x += step_x;
}


  if self.can_move(
    map,
    self.x + COLLISION_OFFSET,
    self.y + step_y + COLLISION_OFFSET,
) {
    self.y += step_y;
}


}

}


// ─── ATTACK ONLY WHEN CLOSE ───
// ─── STATE LOGIC (DO NOT OVERRIDE HURT / DEAD) ───
// ─── STATE LOGIC (STRICT PRIORITY) ───
if self.state == BossState::Dead {
    // do nothing
}

// 1️⃣ If attacking, DO NOT override (handled earlier)
else if self.state == BossState::Attacking {
    // locked until attack_timer finishes
}

// 2️⃣ Hurt = visual reaction ONLY, still chase
else if self.hurt_timer > 0 {
    // Visual only, do NOT cancel chase
    self.state = BossState::Chasing;
}


// 3️⃣ Too far → must chase first
else if dist > ATTACK_RANGE {
    self.state = BossState::Chasing;
}

else if self.attack_cooldown == 0 {
    self.state = BossState::Attacking;
    self.attack_timer = ATTACK_DURATION;
    self.attack_cooldown = if self.health < self.max_health / 2 { 30 } else { 45 };

    // 🔒 LOCK FACING DIRECTION
    self.facing_x = (player.x - self.x).signum();
}


// 5️⃣ Fallback
else {
    self.state = BossState::Chasing;
}



        if self.health <= 0 {
            self.state = BossState::Dead;
        }

        self.hitbox = Bounds::new(
    self.x + COLLISION_OFFSET,
    self.y + COLLISION_OFFSET,
    COLLISION_SIZE as u32,
    COLLISION_SIZE as u32,
);

    }

   pub fn take_damage(&mut self, dmg: i32) {
    if self.state == BossState::Dead {
        return;
    }

    // ❌ Cannot take damage during attack
    if self.state == BossState::Attacking {
        return;
    }

    self.health -= dmg;

    // 🎭 Visual hurt reaction ONLY
    self.hurt_timer = 10;
    self.state = BossState::Hurt;

    if self.health <= 0 {
        self.health = 0;
        self.state = BossState::Dead;
        self.death_timer = 90;
    }
}



pub fn is_death_animation_finished(&self) -> bool {
    self.state == BossState::Dead && self.death_timer == 0
}



    pub fn draw(&self) {
       
        let anim = animation::get("nightborne");

     match self.state {
    BossState::Idle => anim.use_sprite("NightBorne_idle"),
    BossState::Chasing => anim.use_sprite("NightBorne_run"),
    BossState::Attacking => anim.use_sprite("NightBorne_attack"),
    BossState::Hurt => anim.use_sprite("NightBorne_hurt"),
    BossState::Dead => anim.use_sprite("NightBorne_death"),
}




        sprite!(
            animation_key = "nightborne",
            x = self.x as i32,
            y = self.y as i32,
            w = 64,
            h = 64,
            flip_x = self.facing_x < 0.0,
            cover = true
        );
  

       

    }


    fn can_move(&self, map: &Grid, x: f32, y: f32) -> bool {
    let w = COLLISION_SIZE;
let h = COLLISION_SIZE;


    let points = [
        (x + 2.0, y + 2.0),
        (x + w - 2.0, y + 2.0),
        (x + 2.0, y + h - 2.0),
        (x + w - 2.0, y + h - 2.0),
    ];

    for (px, py) in points {
        if is_wall(map, px, py) {
            return false;
        }
    }

    true
}

}

fn world_to_tile(x: f32, y: f32) -> (usize, usize) {
    (
        (x as i32 / TILE_SIZE) as usize,
        (y as i32 / TILE_SIZE) as usize,
    )
}

fn tile_target(tx: usize, ty: usize) -> (f32, f32) {
    (
        tx as f32 * TILE_SIZE as f32 + COLLISION_OFFSET,
        ty as f32 * TILE_SIZE as f32 + COLLISION_OFFSET,
    )
}
