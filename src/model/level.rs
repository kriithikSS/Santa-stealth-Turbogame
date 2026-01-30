use super::*;

#[derive(Clone, Copy)]
pub struct SpawnInfo {
    pub player_spawn: (usize, usize),
}

pub fn load_level_1() -> (Grid, SpawnInfo) {
    let layout = vec![
        "########################",
        "#..........#...........#",
        "#..######..#..#####....#",
        "#..#....#..#..#...#....#",
        "#..#....#......#...#..#",
        "#..##.###..######...#..#",
        "#..........#...........#",
        "######..########..######",
        "#..........#...........#",
        "#..#.####..#..#####....#",
        "#..#....#..#..#...#....#",
        "#..#....#......#...#..#",
        "#..######..######...#..#",
        "#..........#...........#",
        "########################",
    ];

    let grid: Grid = layout
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '#' => Tile { tile_type: TileType::Wall },
                    '.' => Tile { tile_type: TileType::Floor },
                    _ => Tile { tile_type: TileType::Floor },
                })
                .collect()
        })
        .collect();

    let spawn = SpawnInfo {
        player_spawn: (1, 1),
    };

    (grid, spawn)
}


pub fn load_level_2() -> (Grid, SpawnInfo) {
    let layout = vec![
        "########################",
        "#..........#..........#",
        "#..######..#..######...#",
        "#..#....#..#..#....#..#",
        "#..#....#......#....#.#",
        "#..######..######....#.#",
        "#..........#..........#",
        "######..########..######",
        "#..........#..........#",
        "#..######..#..#.####...#",
        "#..#....#..#..#....#..#",
        "#..#....#......#....#.#",
        "#..######..######....#.#",
        "#..........#..........#",
        "########################",
    ];

    let grid: Grid = layout
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '#' => Tile { tile_type: TileType::Wall },
                    '.' => Tile { tile_type: TileType::Floor },
                    _ => Tile { tile_type: TileType::Floor },
                })
                .collect()
        })
        .collect();

    let spawn = SpawnInfo {
        player_spawn: (1, 1),
    };

    (grid, spawn)
}


pub fn load_level_by_index(level: u32) -> (Grid, SpawnInfo) {
    match level {
        1 => load_level_1(),
        2 => load_level_2(),
        _ => load_level_1(),
    }
}

use std::collections::VecDeque;

pub fn compute_reachable(
    grid: &Grid,
    start: (usize, usize),
) -> Vec<Vec<bool>> {
    let h = grid.len();
    let w = grid[0].len();

    let mut visited = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start.1][start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for (nx, ny) in neighbors {
            if nx < w && ny < h
                && !visited[ny][nx]
                && grid[ny][nx].tile_type == TileType::Floor
            {
                visited[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }

    visited
}


pub fn is_wall(grid: &Grid, x: f32, y: f32) -> bool {
    let tx = (x as i32 / TILE_SIZE) as usize;
    let ty = (y as i32 / TILE_SIZE) as usize;

    if ty >= grid.len() || tx >= grid[0].len() {
        return true; // treat outside map as wall
    }

    grid[ty][tx].tile_type == TileType::Wall
}
pub fn tile_to_world(tx: usize, ty: usize) -> (f32, f32) {
    (
        (tx as i32 * TILE_SIZE) as f32,
        (ty as i32 * TILE_SIZE) as f32,
    )
}
pub fn is_spawn_position_valid(grid: &Grid, x: f32, y: f32, size: f32) -> bool {
    let _half = size / 2.0;


    let points = [
        (x + 1.0, y + 1.0),
        (x + size - 1.0, y + 1.0),
        (x + 1.0, y + size - 1.0),
        (x + size - 1.0, y + size - 1.0),
    ];

    for (px, py) in points {
        if is_wall(grid, px, py) {
            return false;
        }
    }

    true
}





pub fn find_large_spawn(
    grid: &Grid,
    size: f32,
) -> Option<(f32, f32)> {
    for ty in 0..grid.len() {
        for tx in 0..grid[0].len() {
            if grid[ty][tx].tile_type != TileType::Floor {
                continue;
            }

            let x = (tx as i32 * TILE_SIZE) as f32;
            let y = (ty as i32 * TILE_SIZE) as f32;

            if is_spawn_position_valid(grid, x, y, size) {
                return Some((x, y));
            }
        }
    }

    None
}


use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    cost: i32,
    priority: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> i32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32
}

pub fn find_path(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize), i32> = HashMap::new();

    open.push(Node {
        pos: start,
        cost: 0,
        priority: 0,
    });

    cost_so_far.insert(start, 0);

    while let Some(current) = open.pop() {
        if current.pos == goal {
            // reconstruct path
            let mut path = vec![goal];
            let mut p = goal;
            while let Some(prev) = came_from.get(&p) {
                p = *prev;
                path.push(p);
            }
            path.reverse();
            return Some(path);
        }

        let (x, y) = current.pos;
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for (nx, ny) in neighbors {
            if ny >= grid.len() || nx >= grid[0].len() {
                continue;
            }
            if grid[ny][nx].tile_type == TileType::Wall {
                continue;
            }

            let new_cost = cost_so_far[&current.pos] + 1;
            let next = (nx, ny);

            if !cost_so_far.contains_key(&next)
                || new_cost < cost_so_far[&next]
            {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost + heuristic(next, goal);
                open.push(Node {
                    pos: next,
                    cost: new_cost,
                    priority,
                });
                came_from.insert(next, current.pos);
            }
        }
    }

    None
}
pub fn find_nearest_floor(
    grid: &Grid,
    start: (usize, usize),
) -> Option<(usize, usize)> {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    queue.push_back(start);
    visited[start.1][start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if grid[y][x].tile_type == TileType::Floor {
            return Some((x, y));
        }

        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for (nx, ny) in neighbors {
            if ny < grid.len()
                && nx < grid[0].len()
                && !visited[ny][nx]
            {
                visited[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }

    None
}
