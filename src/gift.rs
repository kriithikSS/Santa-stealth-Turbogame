use turbo::*;

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum GiftType {
    Life,
    Bullet,
}

#[turbo::serialize]
pub struct Gift {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
    pub kind: GiftType,
}

impl Gift {
    pub fn new(x: f32, y: f32, kind: GiftType) -> Self {
        Self {
            x,
            y,
            alive: true,
            kind,
        }
    }

    pub fn hitbox(&self) -> Bounds {
        Bounds::new(self.x, self.y, 24, 24)
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        // Same look – Santa doesn't know which gift it is
        sprite!(
            "decor/gift",
            x = self.x as i32,
            y = self.y as i32,
            w = 32,
            h = 32,
            cover = true
        );
    }
}
