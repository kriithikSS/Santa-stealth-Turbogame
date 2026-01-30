    use turbo::*;
    use crate::{Grid, is_wall};
    use crate::model::Enemy; // ✅ ADD THIS

    #[turbo::serialize]
#[derive(Copy,PartialEq)]
pub enum WeaponMode {
    Snowball,
    Gun,
}

    // ───────── Player sizing constants ─────────
    const SPRITE_SIZE: f32 = 32.0;
    const COLLISION_SIZE: f32 = 24.0;
    const COLLISION_OFFSET: f32 = (SPRITE_SIZE - COLLISION_SIZE) / 2.0;
    

    #[turbo::serialize]
    pub struct Player {
        pub x: f32,
        pub y: f32,
        pub vx: f32,
        pub vy: f32,
        pub speed: f32,
        pub hitbox: Bounds,
        pub facing_x: f32, // NEW: -1.0 = left, +1.0 = right
        pub health: i32,        // ⭐ NEW
        pub max_health: i32,    // ⭐ NEW

        // ─── Facing & melee ───
        pub facing: (f32, f32), // normalized direction
        pub is_moving: bool,
        pub weapon: WeaponMode,

        

    }


    impl Player {
        pub fn new() -> Self {
            

    let x = (screen().w() as f32 - SPRITE_SIZE) / 2.0;
    let y = (screen().h() as f32 - SPRITE_SIZE) / 2.0;


            Self {
        x,
        y,
        vx: 0.0,
        vy: 0.0,
        speed: 3.0,
        hitbox: Bounds::new(x, y, 32, 32),
        health: 5,
        max_health: 5,


        facing_x: 1.0,          // ✅ default facing right
        facing: (0.0, 1.0),     // down for melee
        is_moving: false,
        weapon: WeaponMode::Snowball,


    }

        }





    pub fn update(&mut self, map: &Grid) {


        let kb = keyboard::get();



                // ─── INPUT AS ACCELERATION ───
        let mut ax = 0.0;
        let mut ay = 0.0;

        if kb.arrow_left().pressed()  { ax -= 1.0; }
        if kb.arrow_right().pressed() { ax += 1.0; }
        if kb.arrow_up().pressed()    { ay -= 1.0; }
        if kb.arrow_down().pressed()  { ay += 1.0; }

        // Normalize diagonal acceleration
        let len = ((ax * ax + ay * ay) as f32).sqrt();

        if len > 0.0 {
            ax /= len;
            ay /= len;
        }

        // Strong brake when reversing direction (BEFORE acceleration)
        if ax != 0.0 && ax.signum() != self.vx.signum() {
            self.vx *= 0.4;
        }
        if ay != 0.0 && ay.signum() != self.vy.signum() {
            self.vy *= 0.4;
        }

        // Apply acceleration
        self.vx += ax * self.speed;
        self.vy += ay * self.speed;


        // ─── FRICTION ───
        let friction = 0.85;
        self.vx *= friction;
        self.vy *= friction;

        // ─── DEAD ZONE (prevents magnetic pull) ───
        if self.vx.abs() < 0.05 { self.vx = 0.0; }
        if self.vy.abs() < 0.05 { self.vy = 0.0; }


        // Clamp max speed
        let max_speed = 2.4;
        let vlen = (self.vx * self.vx + self.vy * self.vy).sqrt();

        if vlen > max_speed {
            self.vx = self.vx / vlen * max_speed;
            self.vy = self.vy / vlen * max_speed;
        }

        self.is_moving = vlen > 0.1;


        // ─── Update sprite facing ONLY on horizontal intent ───
    if self.vx.abs() > self.vy.abs() {
    if self.vx > 0.0 {
        self.facing_x = 1.0;
    } else if self.vx < 0.0 {
        self.facing_x = -1.0;
    }
}

    // If moving mostly up/down → do NOT change facing


if vlen > 0.1 {
    self.facing = (self.vx / vlen, self.vy / vlen);
}



        

    // collision box (smaller than sprite)




        // ───────── X AXIS MOVE ─────────
        let new_x = self.x + self.vx;


    let left   = new_x + COLLISION_OFFSET;
    let right  = new_x + COLLISION_OFFSET + COLLISION_SIZE - 1.0;
    let top    = self.y + COLLISION_OFFSET;
    let bottom = self.y + COLLISION_OFFSET + COLLISION_SIZE - 1.0;

    if !is_wall(map, left, top)
        && !is_wall(map, right, top)
        && !is_wall(map, left, bottom)
        && !is_wall(map, right, bottom)
    {
        self.x = new_x;
} else {
    self.vx = 0.0; // stop sliding into wall
}


        // ───────── Y AXIS MOVE ─────────
        let new_y = self.y + self.vy;


    let left   = self.x + COLLISION_OFFSET;
    let right  = self.x + COLLISION_OFFSET + COLLISION_SIZE - 1.0;
    let top    = new_y + COLLISION_OFFSET;
    let bottom = new_y + COLLISION_OFFSET + COLLISION_SIZE - 1.0;

    if !is_wall(map, left, top)
        && !is_wall(map, right, top)
        && !is_wall(map, left, bottom)
        && !is_wall(map, right, bottom)
    {
       self.y = new_y;
} else {
    self.vy = 0.0; // stop sliding into wall
}
    // ─── Melee timers ───

    // ✅ UPDATE HITBOX EVERY FRAME
    self.hitbox = self.hitbox.position(self.x, self.y);

    }

        pub fn draw(&self) {
        let anim = animation::get("santa_walk_anim");
    anim.use_sprite("santa_walk");

    if self.is_moving {
        anim.resume();
        anim.set_speed(1.0);
    } else {
        anim.pause();
        anim.restart(); // ✅ snaps to frame 0
    }


        // Draw animated Santa
        sprite!(
            animation_key = "santa_walk_anim",
            x = self.x as i32,
            y = self.y as i32,
            w = 32,
            h = 32,
            scale_x = 1.5,
            scale_y = 1.5,
            flip_x = self.facing_x < 0.0,
            cover = true
        );

        // ─── Candy cane melee ───

    }



    }