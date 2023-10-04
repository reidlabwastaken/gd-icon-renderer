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
    fn it_works() {
        let game_sheet_02 = load_spritesheet("assets/GJ_GameSheet02-uhd.plist");
        let game_sheet_glow = load_spritesheet("assets/GJ_GameSheetGlow-uhd.plist");
        let robot_sheet = load_animations("assets/Robot_AnimDesc2.plist");
        let spider_sheet = load_animations("assets/Spider_AnimDesc2.plist");

        let start = Instant::now();
        let rendered_icon = render_icon("ship", 44, [0.0, 0.0, 0.0], [255.0/255.0, 125.0/255.0, 125.0/255.0], true, game_sheet_02, game_sheet_glow, robot_sheet, spider_sheet);
        let end = start.elapsed();

        println!("time taken to render: {:?}", end);

        rendered_icon.save("rendered_icon.png").expect("saving image failed");
    }
}
