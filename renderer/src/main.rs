extern crate renderer;
use renderer::Point;
//use renderer::Tri;

use std::time::Duration;

extern crate spin_sleep;

extern crate rand;
use rand::random;

use std::cell::RefCell;

extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() {

    let width = 800;
    let height = 600;
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

    let a = Point::new(1.0 ,1.0 ,1.0 );
    let b = Point::new(1.0 ,1.0 ,1.0 );
    let c = a + b;
    print!("{:?}", c);


    

    

//     'running: loop {
//         let triangles = vec![
//             Tri::random(width as u32, height as u32),
//             Tri::random(width as u32, height as u32),
//             Tri::random(width as u32, height as u32),
//             Tri::random(width as u32, height as u32),
//             Tri::random(width as u32, height as u32),
//             Tri::random(width as u32, height as u32)
//             ];
//         unsafe{ 
//             clear(pixels, [100,100,255]);
//             pixel_manip(pixels, &triangles, width as i32, height as i32);
//             }

//         texture.with_lock(None, test_closure).unwrap();
//         canvas.clear();
//         canvas.copy(&texture, None, None);
//         canvas.present();

//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit{..} => break 'running,
//                 _ => {}
//             }
//         }

//         //spin_sleep::sleep(Duration::new(1, 12_550_000));
//     }
// }


// unsafe fn pixel_manip(buffer: *mut Vec<u8>, triangles: &Vec<Tri>, width: i32, height: i32) {
//     //a convient way to index. instead of i = (col * height * 3) + row + channel; 
//     // should be less memory reads, but i dont really know. its just a convience thing
//     let mut counter = 0;
//     //could be reinitialized for every pixel, but better to just modify it
//     let mut color = [0,0,0];
//     //counting which row
//     for row in 0..height {
//          //counting how far down the row
//         for col in 0..width {

//             //sets "color" to the current color of the pixel
//             color = [ (*buffer)[counter],(*buffer)[counter +1],(*buffer)[counter +2] ];

//             /*previous 2 loops get us to the right pixel. 
//             this itterator checks every Tri to see if the pixel is in it.
//             will modify color to be the color of the last tri the pixel is within.*/
//             for tri in triangles {
//                 let is_in_tri = tri.is_inside(&Point::new(col as  f32,row as f32,0.0));
//                 if is_in_tri {
//                     color = tri.color;
//                 }
//             }
        
//             //counting which color channel (RGB)
//             for channel in 0..3 {
                
//                 let mut pix = &mut (*buffer)[ counter];
//                 *pix = color[channel];
                
//                 counter += 1;
//             }
//         }
//     }

}

unsafe fn clear(buffer: *mut Vec<u8>, color: [u8; 3] ) {
    //gets the vec from buffer
    let mut pixels = &mut *buffer;
    
    //a convient way to index
    let mut counter = 0;
    for i in 0..(pixels.len() / 3)  {
        for j in 0..color.len() {
            pixels[counter] = color[j];
            counter += 1;
        }
    }
}
