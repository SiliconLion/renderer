extern crate renderer;
use renderer::*;

use geometry::*;
use stl;
use rendering::{render, clear};
use transformations::*;

use std::time::Duration;

extern crate spin_sleep;

extern crate rand;
use rand::random;


extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate nfd;

pub fn main() {

     
//  ######  ########  ##                                                                               
// ##    ## ##     ## ##                                                                               
// ##       ##     ## ##                                                                               
//  ######  ##     ## ##                                                                               
//       ## ##     ## ##                                                                               
// ##    ## ##     ## ##                                                                               
//  ######  ########  ########   

// ########   #######  #### ##       ######## ########  ########  ##          ###    ######## ######## 
// ##     ## ##     ##  ##  ##       ##       ##     ## ##     ## ##         ## ##      ##    ##       
// ##     ## ##     ##  ##  ##       ##       ##     ## ##     ## ##        ##   ##     ##    ##       
// ########  ##     ##  ##  ##       ######   ########  ########  ##       ##     ##    ##    ######   
// ##     ## ##     ##  ##  ##       ##       ##   ##   ##        ##       #########    ##    ##       
// ##     ## ##     ##  ##  ##       ##       ##    ##  ##        ##       ##     ##    ##    ##       
// ########   #######  #### ######## ######## ##     ## ##        ######## ##     ##    ##    ########


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
        test_closure = move |buffer: &mut [u8], _pitch: usize| {
            
            for (i, pix) in buffer.into_iter().enumerate() {
                
                *pix = (*pixels)[i];
            }
            
        };
    }
    


// ########  ######## ##    ## ########  ######## ########  ######## ########             
// ##     ## ##       ###   ## ##     ## ##       ##     ## ##       ##     ##            
// ##     ## ##       ####  ## ##     ## ##       ##     ## ##       ##     ##            
// ########  ######   ## ## ## ##     ## ######   ########  ######   ########             
// ##   ##   ##       ##  #### ##     ## ##       ##   ##   ##       ##   ##              
// ##    ##  ##       ##   ### ##     ## ##       ##    ##  ##       ##    ##             
// ##     ## ######## ##    ## ########  ######## ##     ## ######## ##     ##            


// ########   #######   #######  ########  ######  ######## ########     ###    ########  
// ##     ## ##     ## ##     ##    ##    ##    ##    ##    ##     ##   ## ##   ##     ## 
// ##     ## ##     ## ##     ##    ##    ##          ##    ##     ##  ##   ##  ##     ## 
// ########  ##     ## ##     ##    ##     ######     ##    ########  ##     ## ########  
// ##     ## ##     ## ##     ##    ##          ##    ##    ##   ##   ######### ##        
// ##     ## ##     ## ##     ##    ##    ##    ##    ##    ##    ##  ##     ## ##        
// ########   #######   #######     ##     ######     ##    ##     ## ##     ## ## 

    let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
  	    panic!(e);
    });

    let mut path = String::new();
    match result {
        nfd::Response::Okay(file_path) => path = file_path,
        // nfd::Response::OkayMultiple(files) => println!("Files {:?}", files),
        // nfd::Response::Cancel => println!("User canceled"),
        _ => println!("something went wrong with the file")
    }

     //spin_sleep::sleep(Duration::new(1, 12_550_000));

    let mut triangles = stl::vec_from_stl(&path);
    let camera = rendering::ViewPort::new_from_window_dimentions(
                Point::new(0.0, 0.0, 1.0),
                width,
                height
                );
    
  


// ########  ##     ## ##    ## ##    ## #### ##    ##  ######   
// ##     ## ##     ## ###   ## ###   ##  ##  ###   ## ##    ##  
// ##     ## ##     ## ####  ## ####  ##  ##  ####  ## ##        
// ########  ##     ## ## ## ## ## ## ##  ##  ## ## ## ##   #### 
// ##   ##   ##     ## ##  #### ##  ####  ##  ##  #### ##    ##  
// ##    ##  ##     ## ##   ### ##   ###  ##  ##   ### ##    ##  
// ##     ##  #######  ##    ## ##    ## #### ##    ##  ######   


// ##        #######   #######  ########                         
// ##       ##     ## ##     ## ##     ##                        
// ##       ##     ## ##     ## ##     ##                        
// ##       ##     ## ##     ## ########                         
// ##       ##     ## ##     ## ##                               
// ##       ##     ## ##     ## ##                               
// ########  #######   #######  ##         


    transformations::scale(3.0, &mut triangles);
    transformations::translate_triangles(100.0, 300.0, 200.0, &mut triangles);
    
    'running: loop {

        //transformations::flip_z(&mut triangles);
        transformations::translate_triangles(15.0, 10.0, 0.0, &mut triangles);

        unsafe{ 
            clear(pixels, [100,100,255]);
            render(&camera, pixels, &triangles);
            // pixel_manip(pixels, &triangles, width as i32, height as i32);
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



