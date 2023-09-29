use std::{collections::HashMap, sync::LazyLock};

use maplit::hashmap;

pub const COLORS: &'static [[f32; 3]] = &[
    [125.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 255.0 / 255.0, 125.0 / 255.0],
    [0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0],
    [0.0 / 255.0, 125.0 / 255.0, 255.0 / 255.0],
    [0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0],
    [125.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0],
    [255.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0],
    [255.0 / 255.0, 0.0 / 255.0, 125.0 / 255.0],
    [255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0],
    [255.0 / 255.0, 125.0 / 255.0, 0.0 / 255.0],
    [255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0],
    [255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0],
    [185.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0],
    [255.0 / 255.0, 185.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 200.0 / 255.0, 255.0 / 255.0],
    [175.0 / 255.0, 175.0 / 255.0, 175.0 / 255.0],
    [90.0 / 255.0, 90.0 / 255.0, 90.0 / 255.0],
    [255.0 / 255.0, 125.0 / 255.0, 125.0 / 255.0],
    [0.0 / 255.0, 175.0 / 255.0, 75.0 / 255.0],
    [0.0 / 255.0, 125.0 / 255.0, 125.0 / 255.0],
    [0.0 / 255.0, 75.0 / 255.0, 175.0 / 255.0],
    [75.0 / 255.0, 0.0 / 255.0, 175.0 / 255.0],
    [125.0 / 255.0, 0.0 / 255.0, 125.0 / 255.0],
    [175.0 / 255.0, 0.0 / 255.0, 75.0 / 255.0],
    [175.0 / 255.0, 75.0 / 255.0, 0.0 / 255.0],
    [125.0 / 255.0, 125.0 / 255.0, 0.0 / 255.0],
    [75.0 / 255.0, 175.0 / 255.0, 0.0 / 255.0],
    [255.0 / 255.0, 75.0 / 255.0, 0.0 / 255.0],
    [150.0 / 255.0, 50.0 / 255.0, 0.0 / 255.0],
    [150.0 / 255.0, 100.0 / 255.0, 0.0 / 255.0],
    [100.0 / 255.0, 150.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 150.0 / 255.0, 100.0 / 255.0],
    [0.0 / 255.0, 100.0 / 255.0, 150.0 / 255.0],
    [100.0 / 255.0, 0.0 / 255.0, 150.0 / 255.0],
    [150.0 / 255.0, 0.0 / 255.0, 100.0 / 255.0],
    [150.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 150.0 / 255.0, 0.0 / 255.0],
    [0.0 / 255.0, 0.0 / 255.0, 150.0 / 255.0],
    [125.0 / 255.0, 255.0 / 255.0, 175.0 / 255.0],
    [125.0 / 255.0, 125.0 / 255.0, 255.0 / 255.0]
];

// `zany` = uses 2.0 gamemode render system w/ multiple moving parts
pub struct Gamemode {
    prefix: String,
    zany: bool
}

pub static GAMEMODES: LazyLock<HashMap<&str, Gamemode>> = LazyLock::new(|| { hashmap! {
    "cube" => Gamemode { prefix: "player_".to_string(), zany: false },
    "ship" => Gamemode { prefix: "ship_".to_string(), zany: false },
    "ball" => Gamemode { prefix: "player_ball_".to_string(), zany: false },
    "ufo" => Gamemode { prefix: "bird_".to_string(), zany: false },
    "wave" => Gamemode { prefix: "dart_".to_string(), zany: false },
    // unimplemented
    // "robot" => Gamemode { prefix: "robot_".to_string(), zany: true },
    // "spider" => Gamemode { prefix: "spider_".to_string(), zany: true },
}});
