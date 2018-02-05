use gfx_device_gl::Resources;
use piston_window::*;
use image;

pub fn new_window(width: u32, height: u32) -> PistonWindow {
    let window = WindowSettings::new("", [width, height])
        .exit_on_esc(true)
        .decorated(false)
        .build()
        .unwrap();

    window
}

pub fn new_texture(window: &mut PistonWindow, image: &image::RgbaImage)
    -> Texture<Resources> {
    let texture = Texture::from_image(
        &mut window.factory,
        &image,
        &TextureSettings::new()
        ).unwrap();

    texture
}
