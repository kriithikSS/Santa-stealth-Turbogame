pub const TILE_SIZE: i32 = 32;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

#[turbo::serialize]
pub struct Tile {
    pub tile_type: TileType,
}

pub type Grid = Vec<Vec<Tile>>;