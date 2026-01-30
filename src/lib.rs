use turbo::*;
use turbo::camera;
use crate::map::{
    draw_map,
    draw_background,
    draw_corner_trees,
    draw_lamp_posts,
    draw_border_christmas_lights,
};


mod map;
mod player;
mod model;
mod bullet;
mod snow;
mod gift;



use bullet::Bullet;
use model::*;
use player::Player;


mod start_screen;
use start_screen::StartScreen;
use snow::{spawn_snow, Snow};
mod player_snowball;
use player_snowball::Snowball;
use model::Boss;
use gift::{Gift, GiftType};
mod player_bullet;
use player_bullet::PlayerBullet;
use player::WeaponMode;



#[turbo::game]
struct GameState {
    flow: GameFlow,
    start_screen: StartScreen,
    player: Player,
    map: Grid,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    level: u32, // 👈 ADD THIS
    level_banner_timer: u32,
    player_snowballs: Vec<Snowball>,
    lose_timer: u32,
    gifts: Vec<Gift>, // 🎁 NEW
    gift_timer: u32,        // ⏱ runtime counter
    gift_spawn_time: u32,   // 🎲 random appearance time
    gifts_spawned: bool,    // 🚫 only once per level
    frame: u32,
    boss: Option<Boss>,

    player_bullets: Vec<PlayerBullet>,
    music_timer: u32,
music_phase: u8, // 0 = snowmusic, 1 = backgroundmusic




}

impl GameState {
fn alert_all_snowmen(&mut self) {
    let px = self.player.x;
    let py = self.player.y;

    for enemy in self.enemies.iter_mut() {
        if enemy.alive {
            enemy.alert(px, py);
        }
    }
}



    fn alert_nearby_snowmen(&mut self, x: f32, y: f32) {
    const ALERT_RADIUS: f32 = 220.0;

    for enemy in self.enemies.iter_mut() {
        if !enemy.alive {
            continue;
        }

        let dx = enemy.x - x;
        let dy = enemy.y - y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist <= ALERT_RADIUS {
            enemy.alert(x, y);
        }
    }

    audio::play("alert");
}


fn new() -> Self {

    camera::reset();

    // Load level 1
    let (map, spawn) = load_level_by_index(1);

    // Spawn enemies
    let enemies = Self::spawn_enemies(&map, 1);

    // Create player ONCE
    let mut player = Player::new();
    player.x = (spawn.player_spawn.0 as i32 * TILE_SIZE) as f32;
    player.y = (spawn.player_spawn.1 as i32 * TILE_SIZE) as f32;

    Self {
        flow: GameFlow::Start,
        start_screen: StartScreen::new(),
        player,                 // ✅ USE THE POSITIONED PLAYER
        map,
        enemies,
        bullets: Vec::new(),
        level: 1,
        level_banner_timer: 120,
        player_snowballs: Vec::new(),
        lose_timer: 0,

        // 🎁 Gifts
        gifts: Vec::new(),
        gift_timer: 0,
        gift_spawn_time: random::u32() % (60 * 20) + (60 * 3),
        gifts_spawned: false,

        frame: 0,

        // 👹 Boss (none at level 1)
        boss: None,
        player_bullets: Vec::new(),
        music_timer: 0,
music_phase: 0,


    }
}



    fn map_width_px(&self) -> f32 {
        (self.map[0].len() as i32 * TILE_SIZE) as f32
    }

    fn map_height_px(&self) -> f32 {
        (self.map.len() as i32 * TILE_SIZE) as f32
    }

    fn spawn_boss(&self) -> Boss {
        let mut candidates = Vec::new();

        for (ty, row) in self.map.iter().enumerate() {
            for (tx, tile) in row.iter().enumerate() {
                if tile.tile_type == TileType::Floor {
                    let x = tx as f32 * TILE_SIZE as f32;
                    let y = ty as f32 * TILE_SIZE as f32;

                    // IMPORTANT: size must match boss collision expectations
                    if is_spawn_position_valid(&self.map, x, y, 64.0) {
                        candidates.push((x, y));
                    }
                }
            }
        }

        random::shuffle(&mut candidates);

        let (x, y) = candidates
            .first()
            .expect("No valid boss spawn location");

        Boss::new(*x, *y)
    }


    fn spawn_enemies(map: &Grid, level: u32) -> Vec<Enemy> {
        let mut enemies = Vec::new();
        let mut floor_tiles = Vec::new();

        let (_map, spawn_info) = load_level_by_index(level);
        let reachable = compute_reachable(map, spawn_info.player_spawn);

        for (ty, row) in map.iter().enumerate() {
            for (tx, tile) in row.iter().enumerate() {
                if tile.tile_type == TileType::Floor && reachable[ty][tx] {
                    floor_tiles.push((tx, ty));
                }
            }
        }

        random::shuffle(&mut floor_tiles);

        const ENEMY_COUNT: usize = 5;

        for (tx, ty) in floor_tiles.into_iter() {
            let x = (tx as i32 * TILE_SIZE) as f32;
            let y = (ty as i32 * TILE_SIZE) as f32;

            if is_spawn_position_valid(map, x, y, 32.0) {
                enemies.push(Enemy::new(x, y));
            }

            if enemies.len() == ENEMY_COUNT {
                break;
            }
        }

        enemies
    }

    fn is_open_path(&self, tx: usize, ty: usize) -> bool {
        let h = self.map.len();
        let w = self.map[0].len();

        if tx == 0 || ty == 0 || tx + 1 >= w || ty + 1 >= h {
            return false;
        }

        use TileType::Floor;

        self.map[ty][tx].tile_type == Floor &&
        self.map[ty - 1][tx].tile_type == Floor &&
        self.map[ty + 1][tx].tile_type == Floor &&
        self.map[ty][tx - 1].tile_type == Floor &&
        self.map[ty][tx + 1].tile_type == Floor
    }


fn spawn_gifts(&self) -> Vec<Gift> {
    let mut tiles = Vec::new();

    for (ty, row) in self.map.iter().enumerate() {
        for (tx, tile) in row.iter().enumerate() {
            if tile.tile_type == TileType::Floor && self.is_open_path(tx, ty) {
                tiles.push((tx, ty));
            }
        }
    }

    random::shuffle(&mut tiles);

    let mut gifts = Vec::new();

    if tiles.len() >= 2 {
        let (tx1, ty1) = tiles[0];
        let (tx2, ty2) = tiles[1];

        gifts.push(Gift::new(
            tx1 as f32 * TILE_SIZE as f32 + 8.0,
            ty1 as f32 * TILE_SIZE as f32 + 8.0,
            GiftType::Life,
        ));

        gifts.push(Gift::new(
            tx2 as f32 * TILE_SIZE as f32 + 8.0,
            ty2 as f32 * TILE_SIZE as f32 + 8.0,
            GiftType::Bullet,
        ));
    }

    gifts
}

    
    fn advance_level(&mut self) {
        self.level += 1;

        let (map, spawn) = load_level_by_index(self.level);
        self.map = map;

        let (sx, sy) = tile_to_world(spawn.player_spawn.0, spawn.player_spawn.1);
        self.player = Player::new();
        self.player.x = sx;
        self.player.y = sy;

        self.enemies = Self::spawn_enemies(&self.map, self.level);
        self.bullets.clear();

self.boss = None;


        // 🎬 EFFECTS
        self.level_banner_timer = 120;

        audio::play("level_up"); // optional sound

        camera::reset();
        // 🎁 RESET GIFT LOGIC FOR NEW LEVEL
        self.gifts.clear();
        self.gift_timer = 0;
        self.gift_spawn_time = random::u32() % (60 * 20) + (60 * 3);
        self.gifts_spawned = false;


    }





    fn update(&mut self) {




        self.frame += 1;

        // 🎵 MUSIC SYSTEM (manual timing, Turbo-safe)

// Advance music timer only while playing
if self.flow == GameFlow::Playing {
    self.music_timer += 1;
}

// Phase 0 → Snow music (1:18 = 78s = 4680 frames)
if self.music_phase == 0 && self.music_timer == 1 {
    audio::play("snowmusic");
}

if self.music_phase == 0 && self.music_timer >= 4680 {
    audio::stop("snowmusic");
    audio::play("backgroundmusic");

    self.music_phase = 1;
    self.music_timer = 0;
}

// Phase 1 → Background music (2:39 = 159s = 9540 frames)
if self.music_phase == 1 && self.music_timer >= 9540 {
    audio::stop("backgroundmusic");
    audio::play("backgroundmusic"); // restart loop illusion
    self.music_timer = 0;
}


        let kb = keyboard::get();
        
        
        if self.flow == GameFlow::Lose {
            audio::stop("snowmusic");
audio::stop("backgroundmusic");


            self.lose_timer += 1;

                // DRAW (important!)
            self.draw_game_world();
            self.draw_lose_screen();


            // Restart with SPACE
            if keyboard::get().space().just_pressed() {
                self.reset_game();
            }

            return;
        }


                // 🟢 START SCREEN
        if self.flow == GameFlow::Start {

            
            self.start_screen.update();

            // ALWAYS draw the game world FIRST (prevents black gaps)
            self.draw_game_world();

            // Draw start screen on top
            self.start_screen.draw();

            // Switch flow ONLY after screen fully slid out
            if !self.start_screen.active {
                camera::reset();
                self.flow = GameFlow::Playing;
            }
            else{
                return;
            }
        }

        if self.level_banner_timer > 0 {
            self.level_banner_timer -= 1;
        }

        // ⏱ advance timer only while playing
        if self.flow == GameFlow::Playing {
            self.gift_timer += 1;
        }


        // 🎁 Spawn gifts ONCE at random time
        if !self.gifts_spawned && self.gift_timer >= self.gift_spawn_time {
            self.gifts = self.spawn_gifts(); // exactly 2
            self.gifts_spawned = true;
        }
        // ⏳ Remove gifts after 10 seconds
        if self.gifts_spawned
            && !self.gifts.is_empty()
            && self.gift_timer >= self.gift_spawn_time + (60 * 10)
        {
            self.gifts.clear();
        }

        // Update player
        self.player.update(&self.map);

        // 🎁 Gift pickup logic
        for gift in self.gifts.iter_mut() {
            if !gift.alive {
                continue;
            }

            if gift.hitbox().intersects(&self.player.hitbox) {
                gift.alive = false;

                match gift.kind {
                    GiftType::Life => {
                        self.player.health =
                            (self.player.health + 1).min(self.player.max_health);
                        audio::play("pickup");
                    }
                    GiftType::Bullet => {
                        self.player.weapon = WeaponMode::Gun;
                        audio::play("powerup");
                    }
                }
            }
        }

        

        // Remove collected gifts
        self.gifts.retain(|g| g.alive);


        // ❄ Throw snowball
        if keyboard::get().space().just_pressed() {
            let cx = self.player.x + 16.0;
            let cy = self.player.y + 16.0;
            let dir = self.player.facing;

            match self.player.weapon {
                WeaponMode::Snowball => {
                    self.player_snowballs.push(
                        Snowball::new(cx, cy, dir)
                    );
                    audio::play("throw");
                }
                WeaponMode::Gun => {
                    self.player_bullets.push(
                        PlayerBullet::new(cx, cy, dir)
                    );
                    audio::play("shoot");
                }
            }
        }

        let mut pending_alerts: Vec<(f32, f32)> = Vec::new();

for ball in self.player_snowballs.iter_mut() {
    // Snapshot enemy alive state BEFORE update
    let before_alive: Vec<bool> =
        self.enemies.iter().map(|e| e.alive).collect();

    // Update snowball (may kill enemies)
    ball.update(&self.map, &mut self.enemies);

    // Detect newly killed snowmen
    for (i, was_alive) in before_alive.iter().enumerate() {
        if *was_alive && !self.enemies[i].alive {
            pending_alerts.push((
                self.enemies[i].x,
                self.enemies[i].y,
            ));
        }
    }

    // 👹 Boss hit check (UNCHANGED)
    if let Some(boss) = &mut self.boss {
        if ball.alive {
            let ball_box = Bounds::new(
                ball.x - 3.0,
                ball.y - 3.0,
                6,
                6,
            );

            if ball_box.intersects(&boss.hitbox) {
                boss.take_damage(2);
                ball.alive = false;
            }
        }
    }
}
for (x, y) in pending_alerts {
    self.alert_nearby_snowmen(x, y);
}


        
        self.player_snowballs.retain(|b| b.alive);


        // Update enemies (vision + chase)
        for enemy in self.enemies.iter_mut() {
            if enemy.update(self.player.x, self.player.y, &self.map) {
                let bx = enemy.x + 16.0;
                let by = enemy.y + 16.0;
                let angle = enemy.facing_angle;
                
                // 🔫 PLAY SHOOT SOUND (ONCE PER SHOT)
                audio::play("shoot");

                self.bullets.push(Bullet::new(bx, by, angle));
                

                if self.player.health <= 0 {
                    self.flow = GameFlow::Lose;
                    self.lose_timer = 0;
                    audio::play("caught");
                    return;
                }

            }
        }

        for bullet in self.bullets.iter_mut() {
            bullet.update(&self.map, &mut self.player);
        }

        self.bullets.retain(|b| b.alive);








// Global alert if one enemy dies

// ─── Level clear check ───
// ─── LEVEL PROGRESSION LOGIC ───

// If boss exists → wait until boss dies
if let Some(boss) = &self.boss {
    if boss.is_death_animation_finished() {
        self.advance_level();
        return;
    }
}
// No boss yet
else {
    // All enemies dead
    if self.enemies.iter().all(|e| !e.alive) {
        // EVEN LEVEL → spawn boss
        if self.level % 2 == 0 {
            if let Some((bx, by)) = find_large_spawn(&self.map, 64.0) {
                self.boss = Some(Boss::new(bx, by));
                audio::play("boss_spawn");
            }
        }
        // ODD LEVEL → advance immediately
        else {
            self.advance_level();
            return;
        }
    }
}



        // Clamp player to map
        let map_w = self.map_width_px();
        let map_h = self.map_height_px();

        self.player.x = self.player.x.clamp(0.0, map_w - 32.0);
        self.player.y = self.player.y.clamp(0.0, map_h - 32.0);

        // Camera follows player
        let half_w = screen().w() as f32 / 2.0;
        let half_h = screen().h() as f32 / 2.0;

        let mut cam_x = self.player.x + 16.0;
        let mut cam_y = self.player.y + 16.0;

        cam_x = cam_x.clamp(half_w, map_w - half_w);
        cam_y = cam_y.clamp(half_h, map_h - half_h);

        camera::set_xy(cam_x, cam_y);

                // 👹 UPDATE BOSS (MUST HAPPEN BEFORE DRAWING)
if let Some(boss) = &mut self.boss {
    boss.update(&mut self.player, &self.map);

   if boss.is_death_animation_finished() {
    self.advance_level();
    return;
}


    if self.player.health <= 0 {
        self.flow = GameFlow::Lose;
        audio::play("caught");
        return;
    }
}

        
                // ─── Draw order ───

        // 1. Background (sky)
        draw_background(
            self.map_width_px(),
            self.map_height_px(),
        );

        // 2. Map tiles
        draw_map(&self.map);
        draw_border_christmas_lights(&self.map); // 🎄
        // 🌲 Decorative corner trees
        draw_corner_trees(&self.map);
                //snow piles
        //lamp posts
        draw_lamp_posts(&self.map, self.frame);
        
        // 🎁 ADD THIS BLOCK ⬇⬇⬇
        for gift in self.gifts.iter() {
            gift.draw();
        }


        // 3. Enemies
if let Some(boss) = &self.boss {
    boss.draw(); // ✅ NOW THIS WILL SHOW HURT / DEATH
}
if let Some(boss) = &self.boss {
    self.draw_boss_health_bar(boss);
}


let mut enemy_killed = false;

for bullet in self.player_bullets.iter_mut() {
    bullet.update(&self.map);

    if !bullet.alive {
        continue;
    }

    let b_box = bullet.hitbox();

    // ❄️ HIT SNOWMEN (ENEMIES)
    for enemy in self.enemies.iter_mut() {
    if enemy.alive && b_box.intersects(&enemy.hitbox) {
    enemy.alive = false;
    bullet.alive = false;
    audio::play("hit");

    enemy_killed = true; // ✅ MARK EVENT

    break;
}


    }

    // 👹 HIT BOSS (HIGH DAMAGE)
    if let Some(boss) = &mut self.boss {
        if bullet.alive && b_box.intersects(&boss.hitbox) {
            boss.take_damage(4); // 🔥 MUCH STRONGER THAN SNOWBALL
            bullet.alive = false;
            audio::play("hit");
        }
    }
}

self.player_bullets.retain(|b| b.alive);
// 🚨 GLOBAL ALERT AFTER BULLET LOOP
if enemy_killed {
    self.alert_all_snowmen();
}

for bullet in self.player_bullets.iter() {
    bullet.draw();
}

        for enemy in self.enemies.iter() {
                    enemy.draw();
                }
                self.draw_health_ui(&self.player);

                for bullet in self.bullets.iter() {
            bullet.draw();
        }

        // 4. Player
        self.player.draw();
        for ball in self.player_snowballs.iter() {
            ball.draw();
        }

        self.draw_level_banner();
        self.draw_health_ui(&self.player);
        self.draw_top_hud();

    }

    fn draw_map(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let px = x as i32 * TILE_SIZE;
                let py = y as i32 * TILE_SIZE;

                match tile.tile_type {
                    TileType::Wall => {
                        rect!(
                            x = px,
                            y = py,
                            w = TILE_SIZE as u32,
                            h = TILE_SIZE as u32,
                            color = 0x666666ff
                        );
                    }
                    TileType::Floor => {
                        rect!(
                            x = px,
                            y = py,
                            w = TILE_SIZE as u32,
                            h = TILE_SIZE as u32,
                            color = 0x1e1e2eff
                        );
                    }
                }
            }
        }
    }
    fn draw_health_ui(&self, player: &Player) {
        let x = 12;
        let y = 12;

        let bar_width = 120;
        let bar_height = 14;

        // Background
        rect!(
            x = x,
            y = y,
            w = bar_width + 4,
            h = bar_height + 4,
            fixed = true,
            color = 0x000000aa
        );

        // Health ratio
        let health_ratio = player.health.max(0) as f32 / player.max_health as f32;
        let filled_width = (bar_width as f32 * health_ratio) as u32;

        // Green health bar
        rect!(
            x = x + 2,
            y = y + 2,
            w = filled_width,
            h = bar_height,
            fixed = true,
            color = 0x00ff00ff
        );

        // Optional border
        rect!(
            x = x + 1,
            y = y + 1,
            w = bar_width + 2,
            h = bar_height + 2,
            fixed = true,
            color = 0xffffff44
        );
    }

fn draw_top_hud(&self) {
    let sw = screen().w() as i32;

    // ─── LEFT: Snowmen left (below health bar) ───
    let snowmen = self.snowmen_left();
    let snowmen_text = format!("x {}", snowmen);

    sprite!(
        "snowman",
        x = 12,
        y = 36, // ✅ moved down
        w = 16,
        h = 16,
        fixed = true
    );

    // Shadow
    text!(
        &snowmen_text,
        x = 33,
        y = 37,
        fixed = true,
        scale = 1.8,
        color = 0x000000ff
    );

    // Main
    text!(
        &snowmen_text,
        x = 32,
        y = 36,
        fixed = true,
        scale = 1.8,
        color = 0xffffffff
    );

    // ─── RIGHT: Level (top-right, always visible) ───
    let level_text = format!("LEVEL {}", self.level);
    let width = level_text.len() as i32 * 8 * 2;

    text!(
        &level_text,
        x = sw - width - 14,
        y = 12,
        fixed = true,
        scale = 2.0,
        color = 0xffffffff
    );
}



    fn reset_game(&mut self) {

        audio::stop("snowmusic");
audio::stop("backgroundmusic");

        let mut new_game = GameState::new();

        // ⛔ SKIP START SCREEN ON RESTART
        new_game.flow = GameFlow::Playing;
        new_game.start_screen.active = false;

        *self = new_game;
    }

    fn draw_level_banner(&self) {
        if self.level_banner_timer == 0 {
            return;
        }

        let sw = screen().w();
        let sh = screen().h();

        let t = self.level_banner_timer as f32 / 120.0;
        let alpha = ((1.0 - t) * 180.0) as u32;

        // 🌫 Soft dim background
        rect!(
            x = 0,
            y = 0,
            w = sw,
            h = sh,
            fixed = true,
            color = (alpha << 24)
        );

        // 📄 Center card   
        rect!(
            x = sw / 2 - 140,
            y = sh / 2 - 50,
            w = 280,
            h = 80,
            fixed = true,
            color = 0x000000cc
        );

        let text = format!("LEVEL {}", self.level);
        let scale = 3.0;

        let width = text.len() as f32 * 8.0 * scale;
        let x = (sw as f32 / 2.0 - width / 2.0) as i32;
        let y = sh / 2 - 22;

        text!(
            &text,
            x = x,
            y = y,
            fixed = true,
            scale = scale,
            color = 0xffffffff
        );
    }
    fn draw_game_world(&self) {
        // 1. Background
        draw_background(
            self.map_width_px(),
            self.map_height_px(),
        );

        // 2. Map
        draw_map(&self.map);
        draw_border_christmas_lights(&self.map); 
        // 🌲 Decorative corner trees
        draw_corner_trees(&self.map);
        //lamp_posts
        draw_lamp_posts(&self.map, self.frame);
        
        // 🎁 Gifts
        for gift in self.gifts.iter() {
            gift.draw();
        }
        
        // 3. Enemies
        if let Some(boss) = &self.boss {
    boss.draw();
}
        for enemy in self.enemies.iter() {
            enemy.draw();
        }

        // 4. Bullets
        for bullet in self.bullets.iter() {
            bullet.draw();
        }

        // 5. Player
        self.player.draw();

        // 6. Snowballs
        for ball in self.player_snowballs.iter() {
            ball.draw();
        }

        // 7. UI (always during gameplay)
        self.draw_health_ui(&self.player);
        self.draw_top_hud(); // ✅ always visible while playing
    }


fn draw_lose_screen(&self) {
    let sw = screen().w();
    let sh = screen().h();

    // 🔲 Full-screen dim (same idea as level banner)
    rect!(
        x = 0,
        y = 0,
        w = sw,
        h = sh,
        fixed = true,
        color = 0x000000aa // soft dark dim
    );

    // 📄 Center card (optional but makes text pop)
    rect!(
        x = sw / 2 - 220,
        y = sh / 2 - 70,
        w = 440,
        h = 140,
        fixed = true,
        color = 0x000000cc
    );

    let title = "                   YOU WERE CAUGHT";
    let scale = 4.0;
    let width = title.len() as f32 * 8.0 * scale;

    let title_x = (sw as f32 / 2.0 - width / 2.0) as i32;
    let title_y = sh / 2 - 45;

    // Shadow
    text!(
        title,
        x = title_x + 8,
        y = title_y + 4,
        fixed = true,
        scale = scale,
        color = 0x000000ff
    );

    // Main text (bright)
    text!(
        title,
        x = title_x,
        y = title_y,
        fixed = true,
        scale = scale,
        color = 0xffffffff
    );

    // Retry prompt
    text!(
        "   PRESS SPACE TO RETRY",
        x = sw / 2 - 120,
        y = title_y + 55,
        fixed = true,
        scale = 1.6,
        color = 0xffffffcc
    );
}
    fn snowmen_left(&self) -> usize {
        self.enemies.iter().filter(|e| e.alive).count()
    }



fn draw_boss_health_bar(&self, boss: &Boss) {
    let bar_width = 240;
    let bar_height = 14;

    let screen_w = screen().w();

    // 🔹 Bar position
    let bar_x = (screen_w - bar_width) / 2;
    let bar_y = 16;

    // 🔹 Boss icon position (left of bar)
    let icon_size = 36;
    let icon_x = bar_x - icon_size - 10;
    let icon_y = bar_y - 10;

    let health_ratio =
        boss.health.max(0) as f32 / boss.max_health as f32;
    let filled_width = (bar_width as f32 * health_ratio) as u32;

    // 👹 BOSS ICON (GIF)
    sprite!(
        "NightBorne_idle",
        x = icon_x,
        y = icon_y,
        w = icon_size,
        h = icon_size,
        fixed = true,
        cover = true
    );

    // 🖤 Background
    rect!(
        x = bar_x - 2,
        y = bar_y - 2,
        w = bar_width + 4,
        h = bar_height + 4,
        fixed = true,
        color = 0x000000aa
    );

    // ❤️ Health bar
    rect!(
        x = bar_x,
        y = bar_y,
        w = filled_width,
        h = bar_height,
        fixed = true,
        color = 0xff3b3bff
    );

    // 🧱 Border overlay
    rect!(
        x = bar_x,
        y = bar_y,
        w = bar_width,
        h = bar_height,
        fixed = true,
        color = 0xffffff33
    );
}


}
