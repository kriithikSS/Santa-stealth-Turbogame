# 📘 README.md

Game youtube Link: https://youtu.be/dYx2P52Ma9o

# 🎅 Santa Stealth

A 2D top-down stealth action game built using Rust and the Turbo Game Engine.

---

## 📌 Overview

Santa Stealth is a festive stealth-action game where Santa must sneak through snowy maps, eliminate snowmen enemies, and defeat powerful bosses while avoiding detection. The game combines stealth mechanics, shooting, level progression, and boss battles in a pixel-art environment.

---

## 🧠 Core Gameplay Mechanics

* 🎯 Stealth System
  Enemies have visible detection lines. If Santa enters their line of sight, he gets attacked.

* ❄️ Weapons

  * Snowballs (default weapon)
  * Gun (unlocked via gift pickups)

* 🎁 Gifts (Power-ups)

  * ❤️ Life Gift – increases player health
  * 🔫 Bullet Gift – unlocks gun weapon
    

* 👹 Boss Fights

  * Boss appears at specific levels
  * Has a visible health bar
  * Health reduces gradually on hits
  * Boss UI disappears once defeated

* 🧭 Level Progression

  * Odd levels → Enemy-only stages
  * Even levels → Boss stages
  * Difficulty increases with each level

---

## 🎮 Controls

| Key                  | Action         |
| -------------------- | -------------- |
| Arrow Keys           | Move Santa     |
| Space                | Attack / Shoot |
| Space (Start Screen) | Start Game     |
| Space (Game Over)    | Retry          |

---

## 🖼️ Visual Features

* Pixel-art winter maps
* Animated Santa sprite
* Enemy vision lines
* Boss animations (Idle, Hurt, Death)
* Dynamic camera following the player
* HUD with:

  * Player health bar
  * Remaining enemies
  * Level indicator
  * Boss health bar (when active)

---

## 🛠️ Tech Stack

* Language: Rust
* Engine: Turbo Game Engine
* Rendering: Sprite-based 2D rendering
* Platform: Web (HTML export)

---

## Project structure
```
santa-stealth/
├── audio/                  # Sound effects & background music
├── sprites/                # Game sprites & animations
├── src/
│   ├── model/              # Core game models
│   │   ├── boss.rs         # Boss AI & behavior
│   │   ├── enemy.rs        # Snowman enemies
│   │   ├── level.rs        # Level loading & logic
│   │   ├── tile.rs         # Tile definitions
│   │   └── mod.rs
│   ├── bullet.rs           # Enemy bullets
│   ├── gift.rs             # Power-up system
│   ├── lib.rs              # Game entry point & state
│   ├── map.rs              # Map rendering & collision
│   ├── player.rs           # Player movement & combat
│   ├── player_bullet.rs    # Gun bullet logic
│   ├── player_snowball.rs  # Snowball attacks
│   ├── snow.rs             # Snow particle effects
│   └── start_screen.rs     # Start screen UI
├── www/                    # Web build output
├── target/                 # Compiled artifacts
├── Cargo.toml              # Dependencies
├── Cargo.lock
├── turbo.toml              # Turbo engine configuration
└── .gitignore
```


---

## ▶️ How to Run the Game

Follow these steps to run Santa Stealth locally using Turbo:

### 1️⃣ Clone the Repository

```
git clone <your-repository-link>
```

### 2️⃣ Navigate into the Project Folder

After cloning, you will be inside the project directory:

```
Santa-stealth-Turbogame
```

### 3️⃣ Move One Directory Back

```
cd ..
```

### 4️⃣ Run the Game Using Turbo

```
turbo run -w Santa-stealth-Turbogame-main
```

🎮 The game window will open, and you can start playing immediately.

---


## 🌐 How to Export (Web Version)

```bash
turbo export
```

After export:

* Open the `www/` folder
* Host it using:

  * GitHub Pages
  * Netlify
  * Vercel
  * Any static file server

---

## 🧪 Gameplay Flow

1. Start Screen appears
2. Player presses **Space** to start
3. Navigate the map stealthily
4. Defeat all enemies
5. Face the boss (on boss levels)
6. Progress to next level
7. Game ends when player health reaches zero


