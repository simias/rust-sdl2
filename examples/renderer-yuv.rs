extern crate sdl2;

use sdl2::video::{Window, WindowPos, SHOWN};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

pub fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = match Window::new("rust-sdl2 demo: YUV", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, SHOWN) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::IYUV, (256, 256)).unwrap();
    // Create a U-V gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        // `pitch` is the width of the Y component
        // The U and V components are half the width and height of Y

        let w = 256;
        let h = 256;

        // Set Y (constant)
        for y in 0us..h {
            for x in 0us..w {
                let offset = y*pitch + x;
                buffer[offset] = 128;
            }
        }

        let y_size = pitch*h;

        // Set U and V (X and Y)
        for y in 0us..h/2 {
            for x in 0us..w/2 {
                let u_offset = y_size + y*pitch/2 + x;
                let v_offset = y_size + (pitch/2 * h/2) + y*pitch/2 + x;
                buffer[u_offset] = (x*2) as u8;
                buffer[v_offset] = (y*2) as u8;
            }
        }
    }).unwrap();

    let mut drawer = renderer.drawer();
    drawer.clear();
    drawer.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)));
    drawer.present();

    loop {
        match poll_event() {
            Quit{..} => break,
            KeyDown { keycode: key, .. } => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
    }

    sdl2::quit();
}
