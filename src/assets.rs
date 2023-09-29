use plist;

use std::collections::HashMap;

use image::*;
use image::{DynamicImage, ImageBuffer, imageops};

// "{1,2}" -> `(1, 2)`
fn parse_vec(str: &str) -> (i32, i32) {
    let parts: Vec<&str> = str[1..str.len()-1].split(",").collect();
    let a: Vec<i32> = parts
        .iter()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    return (a[0], a[1])
}
// parse_vec, but for float64
fn parse_vec_f32(str: &str) -> (f32, f32) {
    let parts: Vec<&str> = str[1..str.len()-1].split(",").collect();
    let a: Vec<f32> = parts
        .iter()
        .map(|s| s.trim().parse::<f32>().unwrap())
        .collect();

    return (a[0], a[1])
}
// `"{{1,2},{3,4}}"` -> `{{1, 2}, {3, 4}}`
fn parse_rect_vecs(str: &str) -> ((i32, i32), (i32, i32)) {
    let cleaned_str = str.replace("{", "").replace("}", "");
    let parts: Vec<&str> = cleaned_str.split(",").collect();
    let a: Vec<i32> = parts
        .iter()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    return ((a[0], a[1]), (a[2], a[3]))
}

// Represents a sprite along with its texture data in a spritesheet.
#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    // Whenever rendering the sprite, offset it by this much
    pub offset: (f32, f32),
    // {left, top}, {width, height}. Controls the cropping
    rect: ((i32, i32), (i32, i32)),
    // Whether the texture needs to be counter-rotated 90 degrees counter-clockwise
    rotated: bool,
    size: (i32, i32),
    // Difference between this and `size` is unknown to me
    source_size: (i32, i32)
}

impl Sprite {
    // Shorthand for initializing a sprite with its .plist representation.
    fn initialize(obj: plist::Value) -> Sprite {
        let hash = obj.as_dictionary().expect("object must be a dict");

        let hash_keys = vec!["spriteOffset", "spriteSize", "spriteSourceSize", "textureRect", "textureRotated"];

        let isolated: Vec<(&&str, Option<&plist::Value>)> = hash_keys
            .iter()
            .map(|s| (s, hash.get(s)))
            .collect();

        let missing: Vec<&(&&str, Option<&plist::Value>)> = isolated
            .iter()
            .filter(|&&(_, value)| value.is_none())
            .collect();

        if !missing.is_empty() {
            let missing_entries: Vec<&str> = missing.iter().map(|(&key, _)| key).collect();
            panic!("missing entries: {:?}", missing_entries);
        }

        let isolated_hash: HashMap<String, plist::Value> = isolated
            .iter()
            .map(|&(key, value)| (key.to_string(), value.expect("value is none after checking").clone()))
            .collect();

        return Sprite {
            offset: parse_vec_f32(isolated_hash.get("spriteOffset").expect("missing spriteOffset").as_string().expect("spriteOffset is not a string")),
            rect: parse_rect_vecs(isolated_hash.get("textureRect").expect("missing textureRect").as_string().expect("textureRect is not a string")),
            rotated: isolated_hash.get("textureRotated").unwrap_or(&plist::Value::from(false)).as_boolean().expect("textureRotated is not a boolean").clone(),
            size: parse_vec(isolated_hash.get("spriteSize").expect("missing spriteSize").as_string().expect("spriteSize is not a string")),
            source_size: parse_vec(isolated_hash.get("spriteSourceSize").expect("missing spriteSourceSize").as_string().expect("spriteSourceSize is not a string"))
        }
    }
}

// Represents a spritesheet along with its sprites.
#[derive(Clone)]
pub struct Spritesheet {
    sprites: HashMap<String, Sprite>,

    texture_file_name: String,
    size: (i32, i32)
}

impl Spritesheet {
    // Shorthand for initializing a spritesheet with its .plist representation.
    fn initialize(obj: plist::Value) -> Spritesheet {
        let hash = obj.as_dictionary().expect("object must be a dict");

        let sprites = hash.get("frames").expect("object must have a `frames` object").as_dictionary().expect("`frames` must be a dict");
        let metadata = hash.get("metadata").expect("object must have a `metadata` object").as_dictionary().expect("`metadata` must be a dict");

        return Spritesheet {
            sprites: sprites.iter().map(|(key, value)| (key.clone(), Sprite::initialize(value.clone()))).collect(),
            texture_file_name: metadata.get("textureFileName").expect("metadata must have a `textureFileName` object").as_string().expect("`textureFileName` must be a string").to_string(),
            size: parse_vec(metadata.get("size").expect("metadata must have a `size` object").as_string().expect("`size` must be a string"))
        }
    }
}

// Stores both a spritesheet and its associated `DynamicImage` for easy access.
#[derive(Clone)]
pub struct LoadedSpritesheet {
    spritesheet: Spritesheet,
    texture: DynamicImage
}

// Loads the spritesheet and readies the associated image.
pub fn load_spritesheet(path: &str) -> LoadedSpritesheet {
    return LoadedSpritesheet {
        spritesheet: Spritesheet::initialize(plist::from_file(path).expect("could not load plist")),
        texture: image::open(path.replace(".plist", ".png")).expect("could not load texture")
    }
}

// Trims out a sprite from an image according to a .plist spritesheet.
pub fn get_sprite(spritesheet: Spritesheet, img: DynamicImage, key: String) -> Option<(DynamicImage, Sprite)> {
    let sprite = spritesheet.sprites.get(&key);

    let mut canvas = img.clone();

    if sprite.is_none() {
        return None;
    }

    if let Some(sprite) = sprite {
        let rect = sprite.rect;
        
        let (mut left, mut top, mut width, mut height) = (rect.0.0, rect.0.1, rect.1.0, rect.1.1);
        if sprite.rotated {
            (left, top, width, height) = (left, top, height, width);
        }
        
        canvas = canvas.crop(left as u32, top as u32, width as u32, height as u32);

        if sprite.rotated {
            canvas = canvas.rotate270();
        }

        return Some((canvas, sprite.clone()));
    }

    panic!("The sprite should have been found in the spritesheet or not found at all")
}

pub fn get_sprite_from_loaded(spritesheet: LoadedSpritesheet, key: String) -> Option<(DynamicImage, Sprite)> {
    let texture = spritesheet.texture.clone();
    let sprite = get_sprite(spritesheet.spritesheet.clone(), texture, key);
    return sprite;
}