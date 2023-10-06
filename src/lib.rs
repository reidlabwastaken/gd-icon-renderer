//! # gd-icon-renderer
//! 
//! gd-icon-renderer is a library for rendering Geometry Dash icons.
//! 
//! It uses the [image](https://crates.io/crates/image) crate and [imageproc](https://crates.io/crates/imageproc) crate for image manipulation and [plist](https://crates.io/crates/plist) for parsing plist files.
//!
//! The main entrypoint is found in the [`renderer`](renderer/index.html) module.

#![feature(lazy_cell)]

pub mod assets;
pub mod constants;
pub mod renderer;
    
#[cfg(test)]
mod tests {
    use super::*;

    use renderer::*;
    use assets::*;

    use std::time::Instant;
        
    #[test]
    fn render_test() {
        let game_sheet_02 = load_spritesheet("assets/GJ_GameSheet02-uhd.plist");
        let game_sheet_glow = load_spritesheet("assets/GJ_GameSheetGlow-uhd.plist");
        let robot_sheet = load_animations("assets/Robot_AnimDesc2.plist");
        let spider_sheet = load_animations("assets/Spider_AnimDesc2.plist");

        let start = Instant::now();
        let rendered_icon = render_icon("spider", 16, [0.0, 0.0, 0.0], [255.0/255.0, 125.0/255.0, 125.0/255.0], true, game_sheet_02, game_sheet_glow, robot_sheet, spider_sheet);
        let end = start.elapsed();

        println!("time taken to render: {:?}", end);

        rendered_icon.save("rendered_icon.png").expect("saving image failed");
    }
}
