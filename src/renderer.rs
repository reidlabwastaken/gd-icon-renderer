use image::*;
use image::{DynamicImage, imageops};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};

use std::cmp;

use crate::assets;
use crate::assets::{LoadedSpritesheet, Animations, Sprite};

/// Internal function to easily transform an image
fn transform(image: &DynamicImage, color: Option<[f32; 3]>, scale: Option<(f32, f32)>, rotation: Option<f32>) -> DynamicImage {
    let mut transformed_image = image.clone();

    if let Some(color) = color {
        let mut img_buffer = image.to_rgba8();

        for (_x, _y, pixel) in img_buffer.enumerate_pixels_mut() {
            for channel in 0..3 {
                pixel.0[channel] = (pixel.0[channel] as f32 * color[channel]) as u8;
            }
        }

        transformed_image = DynamicImage::ImageRgba8(img_buffer);
    }

    if let Some((scale_x, scale_y)) = scale {
        let width = transformed_image.width();
        let height = transformed_image.height();

        let abs_scale_x = scale_x.abs();
        let abs_scale_y = scale_y.abs();

        transformed_image = transformed_image.resize_exact(
            (width as f32 * abs_scale_x) as u32,
            (height as f32 * abs_scale_y) as u32,
            image::imageops::FilterType::Lanczos3
        );

        if scale_x < 0.0 {
            transformed_image = transformed_image.fliph();
        }
        if scale_y < 0.0 {
            transformed_image = transformed_image.flipv();
        }
    }

    if let Some(rotation) = rotation {
        // lets not rotate if we dont need to
        if rotation == 0.0 {
            return transformed_image;
        }

        let radians = rotation.to_radians();

        let (width, height) = transformed_image.dimensions();
        
        let trig_width = (width as f32 * radians.cos() + height as f32 * radians.sin())
            .abs()
            .ceil() as u32;
        let trig_height = (width as f32 * radians.sin() + height as f32 * radians.cos())
            .abs()
            .ceil() as u32;

        let transform_x = ((trig_width as f32 / 2.0) - (width as f32 / 2.0)).ceil() as u32;
        let transform_y = ((trig_height as f32 / 2.0) - (height as f32 / 2.0)).ceil() as u32;

        let mut canvas = ImageBuffer::new(cmp::max(trig_width, width), cmp::max(trig_height, height));
        canvas.copy_from(&transformed_image, transform_x, transform_y).expect("couldnt copy from img");
        canvas = rotate_about_center(&canvas, radians, Interpolation::Bilinear, Rgba([0, 0, 0, 0]));
    
        transformed_image = DynamicImage::ImageRgba8(canvas);
    }

    return transformed_image;
}

/// Mainly for internal use; given an array of images, their sizes and colors, tints and composits them into a single image
pub fn render_layered(images: Vec<DynamicImage>, positions: Vec<Option<(f32, f32)>>, colors: Vec<Option<[f32; 3]>>, scales: Vec<Option<(f32, f32)>>, rotations: Vec<Option<f32>>) -> DynamicImage {
    let transformed: Vec<DynamicImage> = images.iter().enumerate().map(|(i, img)| {
        transform(img, colors[i], scales[i], rotations[i])
    }).collect();
    let sizes: Vec<(i64, i64)> = transformed.iter().map(|img| {
        (img.width() as i64, img.height() as i64)
    }).collect();

    let positions: Vec<(f32, f32)> = images.iter().enumerate().map(|(i, _v)| {
        positions[i].unwrap_or((0.0, 0.0))
    }).collect();

    let bounding_box = sizes
        .iter()
        .enumerate()
        .map(|(i, &size)| {
            let (width, height) = size;
            let (x, y) = positions.get(i).cloned().unwrap_or((0.0, 0.0));
            ((width as f32 + x.abs() * 2.0) as i32, (height as f32 + y.abs() * 2.0) as i32)
        })
        .fold((0, 0), |acc, size| {
            (cmp::max(acc.0, size.0), cmp::max(acc.1, size.1))
        });

    let mut canvas = ImageBuffer::new(bounding_box.0 as u32, bounding_box.1 as u32);

    // base
    canvas.copy_from(
        transformed.get(0).expect("no images provided"),
        (bounding_box.0 as f32 / 2.0 + positions[0].0 as f32 - sizes[0].0 as f32 / 2.0) as u32,
        (bounding_box.1 as f32 / 2.0 + positions[0].1 as f32 - sizes[0].1 as f32 / 2.0) as u32
    ).expect("couldnt copy from img");
    
    // stacking
    for (i, image) in transformed.iter().enumerate().skip(1) {
        let x = (bounding_box.0 as f32 / 2.0 + positions[i].0 as f32 - image.width() as f32 / 2.0) as i64;
        let y = (bounding_box.1 as f32 / 2.0 + positions[i].1 as f32 - image.height() as f32 / 2.0) as i64;
    
        imageops::overlay(&mut canvas, image, x, y)
    }

    return DynamicImage::ImageRgba8(canvas);
}

fn is_black(c: [f32; 3]) -> bool {
    c == [0.0, 0.0, 0.0]
}

fn crop_whitespace(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    let mut left = width;
    let mut right = 0;
    let mut top = height;
    let mut bottom = 0;
    
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            if pixel[3] != 0 {
                left = left.min(x);
                right = right.max(x);
                top = top.min(y);
                bottom = bottom.max(y);
            }
        }
    }

    let cropped_image = img.clone().crop(left, top, right - left, bottom - top);

    return cropped_image
}

/// Renders out a non-robot/spider icon. You may be looking for `render_icon`.
pub fn render_normal(basename: String, col1: [f32; 3], col2: [f32; 3], glow: bool, game_sheet_02: LoadedSpritesheet, game_sheet_glow: LoadedSpritesheet) -> DynamicImage {
    let glow_col = if is_black(col2) { if is_black(col1) { [1.0, 1.0, 1.0] } else { col1 } } else { col2 };

    let layers = vec![
        (if glow || (is_black(col1) && is_black(col2)) {
            assets::get_sprite_from_loaded(&game_sheet_glow, format!("{}_glow_001.png", basename))
        } else {
            None
        }),
        assets::get_sprite_from_loaded(&game_sheet_02, format!("{}_2_001.png", basename)),
        assets::get_sprite_from_loaded(&game_sheet_02, format!("{}_3_001.png", basename)),
        assets::get_sprite_from_loaded(&game_sheet_02, format!("{}_001.png", basename)),
        assets::get_sprite_from_loaded(&game_sheet_02, format!("{}_extra_001.png", basename))
    ];

    let colors: Vec<Option<[f32; 3]>> = vec![
        Some(glow_col),
        Some(col2),
        None,
        Some(col1),
        None
    ];

    let layered_images = render_layered(
        layers.iter()
            .filter_map(|s| s.as_ref().map(|(img, _spr)| img.to_owned()))
            .collect(),
        layers.iter()
            .filter_map(|s| s.as_ref().map(|(_img, spr)| Some((spr.offset.0, spr.offset.1 * -1.0))))
            .collect(), 
        colors.iter()
            .enumerate()
            .filter_map(|(i, color)| layers[i].clone().map(|_| color.to_owned()))
            .collect(),
        vec![None, None, None, None, None],
        vec![None, None, None, None, None]
    );

    return crop_whitespace(layered_images);
}

fn flip(scale: (f32, f32), flipped: (bool, bool)) -> (f32, f32) {
    (scale.0 * (if flipped.0 { -1 } else { 1 }) as f32, scale.1 * (if flipped.1 { -1 } else { 1 }) as f32)
}

/// Renders out a robot/spider icon. You may be looking for `render_icon`.
pub fn render_zany(basename: String, col1: [f32; 3], col2: [f32; 3], glow: bool, game_sheet_02: LoadedSpritesheet, _game_sheet_glow: LoadedSpritesheet, animations: Animations) -> DynamicImage {
    let glow_col = if is_black(col2) { if is_black(col1) { [1.0, 1.0, 1.0] } else { col1 } } else { col2 };
    let glow = glow || (is_black(col1) && is_black(col2));

    let mut anim = animations.get("Robot_idle_001.png").unwrap_or_else(|| animations.get("Spider_idle_001.png").expect("no animations found")).clone();
    anim.sort_by_key(|spr| spr.z);

    // TODO: this is a bit of a mess
    // TODO: this is also very slow, but i dont think it can be helped
    // TODO: im not good at memory management so srry

    let mut layers: Vec<(Option<(DynamicImage, Sprite)>, (f32, f32), (f32, f32), f64, bool, Option<[f32; 3]>)> = Vec::new();

    for a in anim {
        let texture_name = a.texture.replace("spider_01", &basename).replace("robot_01", &basename);
        let mut names = vec![
            texture_name.replace("_001.png", "_2_001.png"),
            texture_name.replace("_001.png", "_3_001.png"),
            texture_name.clone(),
            texture_name.replace("_001.png", "_extra_001.png")
        ];
        let mut colors = vec![
            Some(col2),
            None,
            Some(col1),
            None
        ];

        if glow {
            names.push(texture_name.replace("_001.png", "_glow_001.png"));
            colors.push(Some(glow_col));
        }

        layers.extend(names.iter().enumerate().map(|(i, v)| {
            (
                assets::get_sprite_from_loaded(&game_sheet_02, v.clone()),
                a.position,
                flip(a.scale, a.flipped),
                a.rotation,
                glow && i == names.len() - 1,
                colors[i]
            )
        }))
    }

    // put glow b4 everything else
    layers.sort_by_key(|t| if t.4 { 0 } else { 1 });

    let layers_r = layers.iter()
        .filter(|v| v.0.is_some())
        .filter_map(|(opt_sprite, pos, scale, rot, glow, color)| opt_sprite.clone().map(|sprite| ((sprite.0, sprite.1), *pos, *scale, *rot, *glow, *color)))
        .collect::<Vec<((DynamicImage, Sprite), (f32, f32), (f32, f32), f64, bool, Option<[f32; 3]>)>>();

    let layered_images = render_layered(
        layers_r.iter().map(|t| t.0.0.clone()).collect(),
        layers_r.iter().map(|t| Some((t.0.1.offset.0 + t.1.0 * 4.0, t.0.1.offset.1 * -1.0  + t.1.1 * -4.0))).collect(),
        layers_r.iter().map(|t| t.5).collect(),
        layers_r.iter().map(|t| Some(t.2)).collect(),
        layers_r.iter().map(|t| Some(t.3 as f32)).collect()
    );

    return crop_whitespace(layered_images);
}

/// The main entrypoint for icon rendering; this should be all you need to render out an icon.
///
/// `gamemode` must be one of `cube`, `ship`, `ball`, `ufo`, `wave`, `robot`, or `spider`
pub fn render_icon(gamemode_str: &str, icon: i32, col1: [f32; 3], col2: [f32; 3], glow: bool, game_sheet_02: LoadedSpritesheet, game_sheet_glow: LoadedSpritesheet, robot_animations: Animations, spider_animations: Animations) -> DynamicImage {
    let gamemode = crate::constants::GAMEMODES.get(gamemode_str).expect("invalid gamemode");

    if gamemode.zany {
        return render_zany(format!("{}{:02}", gamemode.prefix, icon), col1, col2, glow, game_sheet_02, game_sheet_glow, if gamemode_str == "robot" { robot_animations } else { spider_animations })
    } else {
        return render_normal(format!("{}{:02}", gamemode.prefix, icon), col1, col2, glow, game_sheet_02, game_sheet_glow)
    }
}