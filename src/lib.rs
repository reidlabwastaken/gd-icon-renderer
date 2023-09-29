#![feature(lazy_cell)]

pub mod assets;
pub mod constants;
pub mod renderer;
    
#[cfg(test)]
mod tests {
    use super::*;

    use renderer::*;
    use assets::*;

    // not actually used, just for benchmarking
    use std::time::Instant;
        
    #[test]
    fn it_works() {
        let game_sheet_02 = load_spritesheet("assets/GJ_GameSheet02-uhd.plist");
        let game_sheet_glow = load_spritesheet("assets/GJ_GameSheetGlow-uhd.plist");

        let start = Instant::now();
        let rendered_img = render_normal(
            "ship_18".to_string(),
            [0.0/255.0, 0.0/255.0, 0.0/255.0],
            [0.0/255.0, 0.0/255.0, 0.0/255.0],
            true,
            game_sheet_02,
            game_sheet_glow,
        );

        rendered_img.save("rendered_icon.png").expect("saving image failed");
        let end = Instant::now();
        println!("Time elapsed: {:?}", end.duration_since(start));
    }
}
