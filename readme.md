# gd-icon-renderer

A rust Geometry Dash icon renderer. Shout out to [gd-icon-renderer](https://github.com/oatmealine/gd-icon-renderer), this project is just a rewrite but not in [`libvips`](https://www.libvips.org/) and [`crystal-lang`](https://crystal-lang.org/) for mostly personal use.

[![Docs](https://img.shields.io/docsrs/gd-icon-renderer)](https://docs.rs/gd-icon-renderer/latest)
![Version](https://img.shields.io/crates/v/gd-icon-renderer)

## Usage

Provide your `GJ_GameSheet02-uhd`, `GJ_GameSheetGlow-uhd`, `Robot_AnimDesc2`, and `Spider_AnimDesc2` files along with their corresponding `*.plist` files. Rendering an icon is as follows:

1. Import the library:

    ```rs
    use gd_icon_renderer;
    ```

2. Load your spritesheets:

    ```rs
    // Replace filepaths for whatever you need for your usecase
    let game_sheet_02 = gd_icon_renderer::assets::load_spritesheet("assets/GJ_GameSheet02-uhd.plist");
    let game_sheet_glow = gd_icon_renderer::assets::load_spritesheet("assets/GJ_GameSheetGlow-uhd.plist");
    let robot_sheet = gd_icon_renderer::assets::load_animations("assets/Robot_AnimDesc2.plist");
    let spider_sheet = gd_icon_renderer::assets::load_animations("assets/Spider_AnimDesc2.plist");
    ```

3. Render the icon out:

    ```rs
    let icon_img = gd_icon_renderer::renderer::render_icon("ship", 44, [0.0, 0.0, 0.0], [255.0/255.0, 125.0/255.0, 125.0/255.0], true, game_sheet_02, game_sheet_glow, robot_sheet, spider_sheet);
    ```

    You'll now be given a [`DynamicImage`](https://docs.rs/image/latest/image/enum.DynamicImage.html)

4. You'll most likely want to save the resulting image somewhere:

    ```rs
    icon_img.save("icon_rendered.png").unwrap();
    ```

## Todo

- Add examples to the repo.
- Trim extra alpha space on the final result.
- I think theres some weird shifting and offsets going on, please investigate 🥺. Really big on `spider_16` for some reason?? Related issue on the inspired project [here](https://github.com/oatmealine/gd-icon-renderer/issues/2).