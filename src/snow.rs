use turbo::*;

#[turbo::serialize]
pub struct Snow {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub size: u32,
    pub alpha: u8,
    pub drift: f32,
}

pub fn spawn_snow(count: usize) -> Vec<Snow> {
    let mut snow = Vec::new();
    let w = screen().w() as f32;
    let h = screen().h() as f32;

    for _ in 0..count {
        snow.push(Snow {
            x: random::f32() * w,
            y: random::f32() * h,
            speed: 0.15 + random::f32() * 0.8,
            size: random::between(2, 4),
            alpha: random::between(30, 80) as u8,
            drift: -0.3 + random::f32() * 0.6,
        });
    }

    snow
}

pub fn draw_snow_fog(snow: &mut Vec<Snow>) {
    let w = screen().w() as f32;
    let h = screen().h() as f32;

    // 🌫 Soft white fog base
    rect!(
        x = 0,
        y = 0,
        w = w as u32,
        h = h as u32,
        color = 0xffffff33,
        fixed = true
    );

    // ❄ Soft snow particles
    for flake in snow.iter_mut() {
        flake.y += flake.speed;
        flake.x += flake.drift;

        if flake.y > h {
            flake.y = -10.0;
            flake.x = random::f32() * w;
        }

        let color =
            ((flake.alpha as u32) << 24) |
            (255 << 16) |
            (255 << 8) |
            255;

        circ!(
            x = flake.x as i32,
            y = flake.y as i32,
            d = flake.size,
            color = color,
            fixed = true
        );
    }

    // 🌫 Final mist overlay
    rect!(
        x = 0,
        y = 0,
        w = w as u32,
        h = h as u32,
        color = 0xffffff22,
        fixed = true
    );
}
