extern crate renderer;
use renderer::*;

use geometry::*;
use stl;
use rendering::{pixel_manip, clear};
use translations::*;

use std::time::Duration;

extern crate spin_sleep;

extern crate rand;
use rand::random;


extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() {

    let width = 800;
    let height = 800;
    let depth = 900;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, width , height ).unwrap();

    let mut test_closure;
    //width * height is number of pixels, then 3 color channels per pixel
    let pixels = Box::new( vec![30; (width * height * 3) as usize] ) ;
    let pixels = Box::into_raw(pixels);
    unsafe {
        test_closure = move |buffer: &mut [u8], pitch: usize| {
            
            for (i, pix) in buffer.into_iter().enumerate() {
                
                *pix = (*pixels)[i];
            }
            
        };
    }
    

    let mut triangles = stl::vec_from_stl(&String::from("/Users/davidsullivan/Desktop/Programing/Rust/renderer/renderer/src/assets/charmander_starter_1gen_flowalistik.STL"));

    'running: loop {

        translations::translate_triangles(10.0, 30.0, 0.0, &mut triangles);
        
        unsafe{ 
            clear(pixels, [100,100,255]);
            pixel_manip(pixels, &triangles, width as i32, height as i32);
            }

        texture.with_lock(None, test_closure).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                _ => {}
            }
        }

        //spin_sleep::sleep(Duration::new(1, 12_550_000));
    }
}



