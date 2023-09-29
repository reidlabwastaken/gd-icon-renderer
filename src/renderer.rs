use image::*;
use image::{DynamicImage, imageops};

use std::cmp;

use crate::assets;
use crate::assets::LoadedSpritesheet;

// Internal function to easily transform an image
fn transform(image: &DynamicImage, color: Option<[f32; 3]>, scale: Option<(f32, f32)>, rotation: Option<i32>) -> DynamicImage {
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
        match rotation {
            0 => (),
            90 => transformed_image = transformed_image.rotate90(),
            180 => transformed_image = transformed_image.rotate180(),
            270 => transformed_image = transformed_image.rotate270(),
            _ => panic!("rotation must be 0, 90, 180, or 270"),
        }
    }

    return transformed_image;
}

// Mainly for internal use; given an array of images, their sizes and colors, tints and composits them into a single image
pub fn render_layered(images: Vec<DynamicImage>, positions: Vec<Option<(f32, f32)>>, colors: Vec<[f32; 3]>, scales: Vec<Option<(f32, f32)>>, rotations: Vec<Option<i32>>) -> DynamicImage {
    let transformed: Vec<DynamicImage> = images.iter().enumerate().map(|(i, img)| {
        transform(img, Some(colors[i]), scales[i], rotations[i])
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
        .fold((0, 0), |acc, (i, &size)| {
            let (width, height) = size;
            let (x, y) = positions.get(i).cloned().unwrap_or((0.0, 0.0));

            (
                cmp::max(acc.0, (width as f32 + x.abs() * 2.0) as i32),
                cmp::max(acc.1, (height as f32 + y.abs() * 2.0) as i32)
            )
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

// Renders out a non-robot/spider icon. You may be looking for `render_icon`.
pub fn render_normal(basename: String, col1: [f32; 3], col2: [f32; 3], glow: bool, game_sheet_02: LoadedSpritesheet, game_sheet_glow: LoadedSpritesheet) -> DynamicImage {
    let glow_col = if is_black(col2) { if is_black(col1) { [1.0, 1.0, 1.0] } else { col1 } } else { col2 };

    let layers = vec![
        (if glow || (is_black(col1) && is_black(col2)) {
            assets::get_sprite_from_loaded(game_sheet_glow, format!("{}_glow_001.png", basename))
        } else {
            None
        }),
        assets::get_sprite_from_loaded(game_sheet_02.clone(), format!("{}_2_001.png", basename)),
        assets::get_sprite_from_loaded(game_sheet_02.clone(), format!("{}_3_001.png", basename)),
        assets::get_sprite_from_loaded(game_sheet_02.clone(), format!("{}_001.png", basename)),
        assets::get_sprite_from_loaded(game_sheet_02, format!("{}_extra_001.png", basename))
    ];

    let colors: Vec<Option<[f32; 3]>> = vec![
        Some(glow_col),
        Some(col2),
        None,
        Some(col1),
        None
    ];

    return render_layered(
        layers.iter()
            .filter_map(|s| s.as_ref().map(|(img, _spr)| img.to_owned()))
            .collect(),
        layers.iter()
            .filter_map(|s| s.as_ref().map(|(_img, spr)| Some((spr.offset.0, spr.offset.1 * -1.0))))
            .collect(), 
        colors.iter()
            .enumerate()
            .filter_map(|(i, color)| layers[i].clone().map(|_| color.unwrap()))
            .collect(),
        vec![None, None, None, None, None],
        vec![None, None, None, None, None]
    );
}