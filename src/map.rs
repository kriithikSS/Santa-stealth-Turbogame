use turbo::*;
use crate::{Grid, TileType, TILE_SIZE};

/// Draws the static background (sky)
pub fn draw_background(map_width_px: f32, map_height_px: f32) {
    // Tile the background to cover the whole map
    let bg_w = 640; // width of background sprite
    let bg_h = 360; // height of background sprite

    let tiles_x = (map_width_px / bg_w as f32).ceil() as i32;
    let tiles_y = (map_height_px / bg_h as f32).ceil() as i32;

    for y in 0..tiles_y {
        for x in 0..tiles_x {
            sprite!(
                "background_sky",
                x = x * bg_w,
                y = y * bg_h,
                w = bg_w as u32,
                h = bg_h as u32,
                cover = true
            );
        }
    }
}

/// Draws walls and floors
pub fn draw_map(grid: &Grid) {
    for (row, line) in grid.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            let x = (col as i32) * TILE_SIZE;
            let y = (row as i32) * TILE_SIZE;

            match tile.tile_type {
                TileType::Floor => {
                    sprite!(
                        "decor/tile_dirt",
                        x = x,
                        y = y,
                        w = TILE_SIZE as u32,
                        h = TILE_SIZE as u32,
                        cover = true
                    );
                }

                TileType::Wall => {
                    sprite!(
                        "decor/tile_snow_dirt_edge_1",
                        x = x,
                        y = y,
                        w = TILE_SIZE as u32,
                        h = TILE_SIZE as u32,
                        cover = true
                    );
                }
            }
        }
    }
}

pub fn draw_corner_trees(grid: &Grid) {
    let h = grid.len();
    let w = grid[0].len();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if grid[y][x].tile_type != TileType::Floor {
                continue;
            }

            let up    = grid[y - 1][x].tile_type == TileType::Wall;
            let down  = grid[y + 1][x].tile_type == TileType::Wall;
            let left  = grid[y][x - 1].tile_type == TileType::Wall;
            let right = grid[y][x + 1].tile_type == TileType::Wall;

            if (up as u8 + down as u8 + left as u8 + right as u8) == 3 {
                let px = (x as i32) * TILE_SIZE;
                let py = (y as i32) * TILE_SIZE;

                sprite!(
                    "decor/decor_tree_christmas",
                    x = px - TILE_SIZE / 2,
                    y = py - TILE_SIZE,
                    w = (TILE_SIZE * 2) as u32,
                    h = (TILE_SIZE * 2) as u32,
                    cover = true
                );
            }
        }
    }
}


pub fn draw_lamp_posts(grid: &Grid, frame: u32) {
    let h = grid.len();
    let w = grid[0].len();

    let flicker =
        ((frame as f32 * 0.05).sin() * 20.0 + 200.0) as u32;

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if grid[y][x].tile_type != TileType::Floor {
                continue;
            }

            let up    = grid[y - 1][x].tile_type == TileType::Wall;
            let down  = grid[y + 1][x].tile_type == TileType::Wall;
            let left  = grid[y][x - 1].tile_type == TileType::Wall;
            let right = grid[y][x + 1].tile_type == TileType::Wall;

            // ✅ TRUE CORNER ONLY
            let is_corner =
                (up && left) ||
                (up && right) ||
                (down && left) ||
                (down && right);

            if !is_corner {
                continue;
            }

            let px = (x as i32) * TILE_SIZE;
            let py = (y as i32) * TILE_SIZE;

            // Lamp
            sprite!(
                "decor/decor_lamp_post",
                x = px + 10,
                y = py - 18,
                w = 12,
                h = 28,
                cover = true
            );

            // Glow
            rect!(
                x = px + 6,
                y = py - 22,
                w = 20,
                h = 20,
                color = (flicker << 24) | 0xffcc00
            );
        }
    }
}



pub fn draw_border_christmas_lights(grid: &Grid) {
    let h = grid.len();
    if h < 2 { return; }

    let w = grid[0].len();
    if w < 2 { return; }

    // ─── TOP BORDER ONLY ───
    for x in (0..w).step_by(3) {
        if grid[0][x].tile_type != TileType::Wall {
            continue;
        }

        let px = (x as i32) * TILE_SIZE;
        let py = 0;

        sprite!(
            "decor/decor_christmas_lights",
            x = px,
            y = py + 2,
            w = TILE_SIZE as u32,
            h = 8
        );
    }

    // ─── BOTTOM BORDER ONLY ───
    let y = h - 1;
    for x in (0..w).step_by(3) {
        if grid[y][x].tile_type != TileType::Wall {
            continue;
        }

        let px = (x as i32) * TILE_SIZE;
        let py = (y as i32) * TILE_SIZE;

        sprite!(
            "decor/decor_christmas_lights",
            x = px,
            y = py + 2,
            w = TILE_SIZE as u32,
            h = 8
        );
    }
}
