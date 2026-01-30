use turbo::*;
use crate::snow::*;


/// Smooth cubic easing
fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = (2.0 * t) - 2.0;
        0.5 * f * f * f + 1.0
    }
}

#[turbo::serialize]
pub struct StartScreen {
    pub active: bool,

    // PHASE 1: title slide in
    title_t: f32,
    title_y: f32,

    // PHASE 2: screen slide out
    started: bool,
    screen_t: f32,

    snow: Vec<Snow>,
    // Ambient light band
    light_t: f32,
    frame: u32,

}

impl StartScreen {
    pub fn new() -> Self {
        let sh = screen().h() as f32;

        Self {
            active: true,

            title_t: 0.0,
            title_y: sh + 140.0, // start below screen

            started: false,
            screen_t: 0.0,
            light_t: 0.0,
            frame:0,
            snow: spawn_snow(160),
            

        }
    }

    pub fn update(&mut self) {
        let kb = keyboard::get();
        let sh = screen().h() as f32;
        self.frame += 1;

        // ---------- PHASE 1: AUTO TITLE SLIDE ----------
        if self.title_t < 1.0 {
            self.title_t += 1.0 / 250.0; // SLOW & cinematic
            if self.title_t > 1.0 {
                self.title_t = 1.0;
            }

            let eased = ease_in_out(self.title_t);
            let target_y = sh / 2.0 - 100.0;

// FINAL resting position of title
            let final_y = sh / 2.0 - 100.0;

            // How far BELOW the screen the title starts
            let start_offset = 220.0; // 👈 controls perceived speed

            self.title_y =
                final_y + (1.0 - eased) * start_offset;


            if self.title_y < target_y {
                self.title_y = target_y;
            }
        }

        // ---------- PHASE 2: WAIT FOR SPACE ----------
        if self.title_t >= 1.0 && !self.started && kb.space().just_pressed() {
            self.started = true;
            audio::play("intro");
        }

        // ---------- PHASE 2: SCREEN SLIDE UP ----------
        if self.started {
            self.screen_t += 1.0 / 150.0;
            if self.screen_t >= 1.0 {
                self.screen_t = 1.0;
                self.active = false; // ENTER GAME
            }
        }
    }

    pub fn draw(&mut self) {
        let sw = screen().w();
        let sh = screen().h();
        // screen slide offset (PHASE 2)
        let slide_y =
            -(ease_in_out(self.screen_t) * sh as f32) as i32;

        // 🖼️ Background sprite
        sprite!(
            "simpsnow-16colors",
            x = 0,
            y = slide_y,
            w = sw,
            h = sh,
            fixed = true
        );

        // Ambient light movement (slow, looping)
        self.light_t += 0.003;
            if self.light_t > 1.0 {
                self.light_t -= 1.0;
            }


        let time = self.frame as f32;
        let blink = 0.85 + 0.15 * (time * 0.03).sin(); // very soft pulse
        let alpha = (blink * 255.0) as u32;
        // 🎄 Hanging Christmas lights (top edge)
        let light_h = 18;
        let mut x = 0;

        while x < sw {
            sprite!(
                "christmas_lights",
                x = x,
                y = slide_y, // sticks to top even when screen slides
                w = 256,
                h = light_h,
                fixed = true,
                cover = true,
                
            );

            x += 256;
        }


        // ❄️ Snow
        for flake in self.snow.iter_mut() {
            flake.y += flake.speed;
            if flake.y > sh as f32 {
                flake.y = 0.0;
                flake.x = random::f32() * sw as f32;
            }

            circ!(
                x = flake.x as i32,
                y = flake.y as i32 + slide_y,
                d = 2,
                color = 0xffffffcc,
                fixed = true
            );
        }

        // 🌙 Ambient moonlight band (very subtle)
        let band_y = (sh as f32 * (0.25 + 0.5 * self.light_t.sin().abs())) as i32;

        rect!(
            x = 0,
            y = band_y + slide_y,
            w = sw,
            h = 60,
            fixed = true,
            color = 0xffffff10 // VERY faint
        );


        // ---------- TITLE ----------
        let title = "    SANTA STEALTH";

        let scale = 5.0; // BIG
        let char_w = 7.2; // font compensation
        let title_width = title.len() as f32 * char_w * scale;


        let title_x =
            (sw as f32 / 2.0 - title_width / 2.0) as i32;
        let title_y = self.title_y as i32 + slide_y;
        // Soft glow (same color)
        for (ox, oy) in [(-4,0),(4,0),(0,-4),(0,4)] {
            text!(
                title,
                x = title_x + ox,
                y = title_y + oy,
                fixed = true,
                scale = 5.5,
                color = 0x000000ff
            );
        }

        text!(
            title,
            x = title_x,
            y = title_y,
            fixed = true,
            scale = 5.5,
            color = 0xffffffff
        );

        

        // 🎅 Santa hat on the "S"
        let hat_w = 48;
        let hat_h = 40;

        // Each character width in pixels
        let letter_px = char_w * scale;

        // 4 leading spaces before "SANTA"
        let s_x = title_x as f32 + letter_px * 3.0;

        // Position the hat
        let hat_x = s_x - 15.0;
        let hat_y = title_y as f32 - 30.0; // sits nicely above S

        sprite!(
            "santa_hat",
            x = hat_x,
            y = hat_y,
            w = hat_w,
            h = hat_h,
            fixed = true,
            cover = true
        );
        rect!(
            x = hat_x as i32 + 6,
            y = hat_y as i32 + 30,
            w = 28,
            h = 6,
            fixed = true,
            color = 0x00000044
        );


        // 🎅 Retro Santa underline (simple + clean)
        rect!(
            x = title_x + 220,
            y = title_y + 60,
            w = 210,
            h = 6,
            fixed = true,
            color = 0xb11212ff // Santa red
        );

        // ---------- DESCRIPTION CARD ----------
        let desc1 = "Sneak. Hide. Strike from the shadows.";
let desc2 = "Take down snowmen before they see you.";


        let d_scale = 1.2;
        let line_h = 22;

        let card_w = 520;
        let card_h = 80;

        let card_x = sw / 2 - card_w / 2;
        let card_y = title_y + 90;

        // Soft light card (improves readability)
        rect!(
            x = card_x,
            y = card_y,
            w = card_w,
            h = card_h,
            fixed = true,
            color = 0xffffffaa

        );
        rect!(
            x = card_x,
            y = card_y,
            w = card_w,
            h = 4,
            fixed = true,
            color = 0xffffffff
        );


        // Text positions
        let text_x1 =
            (sw as f32 / 2.0 - desc1.len() as f32 * 7.0 * d_scale / 2.0) as i32;
        let text_x2 =
            (sw as f32 / 2.0 - desc2.len() as f32 * 7.0 * d_scale / 2.0) as i32;

        let text_y1 = card_y + 18;
        let text_y2 = card_y + 18 + line_h;

        // Line 1


        // Main text
        text!(
            desc1,
            x = text_x1,
            y = text_y1,
            fixed = true,
            scale = 1.3,
            color = 0x1a1a1aff
        );

        // Main text
        text!(
            desc2,
            x = text_x2,
            y = text_y2,
            fixed = true,
            scale = 1.6,
            color = 0x1a1a1aff
        );
// ---------- GAMEPLAY TUTORIAL ----------
if !self.started {
    let tut_w = 560;
    let tut_h = 70;
    let tut_x = sw / 2 - tut_w / 2;
    let tut_y = card_y + card_h + -15;






    // Dark frosted panel
   rect!(
    x = tut_x,
    y = tut_y + slide_y,
    w = tut_w,
    h = tut_h,
    fixed = true,
    color = 0x000000ee   // darker
);



    // Top glow line
   rect!(
    x = tut_x,
    y = tut_y,
    w = tut_w,
    h = 4,
    fixed = true,
    color = 0xb11212ff
);


   let line1 = "Arrow keys = MOVE";
let line2 = "SPACE = ATTACK / SHOOT";


    let scale = 1.8;

    let l1_x =
        (sw as f32 / 2.0 - line1.len() as f32 * 7.0 * scale / 2.0) as i32;
    let l2_x =
        (sw as f32 / 2.0 - line2.len() as f32 * 7.0 * scale / 2.0) as i32;

    // Text shadow
    text!(
        line1,
        x = l1_x + 2,
        y = tut_y  + 28 + 2,
        fixed = true,
        scale = scale,
        color = 0x000000ff
    );
    text!(
        line2,
        x = l2_x + 2,
        y = tut_y   + 56 + 2,
        fixed = true,
        scale = scale,
        color = 0x000000ff
    );

    

    // Main text
    text!(
        line1,
        x = l1_x,
        y = tut_y  + 28,
        fixed = true,
        scale = scale,
        color = 0xffffffff
    );
    text!(
        line2,
        x = l2_x,
        y = tut_y  + 56,
        fixed = true,
        scale = scale,
        color = 0xffffffff
    );

  // ---------- FLOATING START PROMPT ----------
let prompt = "▶ PRESS SPACE TO START ◀";
let scale = 1.6;

let px =
    (sw as f32 / 2.0
        - prompt.len() as f32 * 7.0 * scale / 2.0) as i32;

// ✅ ALWAYS visible, centered
let py = (sh as f32 * 0.9) as i32 + slide_y;

// Dark backing strip
rect!(
    x = px - 20,
    y = py - 10,
    w = (prompt.len() as u32 * 7 + 40),
    h = 36,
    fixed = true,
    color = 0x000000cc
);

// Blink
let pulse = 0.6 + 0.4 * (self.frame as f32 * 0.08).sin().abs();
let alpha = (pulse * 255.0) as u32;

// Text
text!(
    prompt,
    x = px,
    y = py,
    fixed = true,
    scale = scale,
    color = (alpha << 24) | 0xffffff
);

}



      

    }
}
