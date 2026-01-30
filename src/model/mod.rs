pub use tile::*;
pub use level::*;
pub mod tile;
pub mod level;
pub mod boss;

pub mod enemy;
pub use enemy::*;
pub use boss::*;

use turbo::*;

#[turbo::serialize]
#[derive(PartialEq, Copy)]
pub enum GameFlow {
    Start,
    Playing,
    Win,
    Lose,

}
