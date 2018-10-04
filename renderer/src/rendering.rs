
use geometry::*;
use math::*;


type x_color = (f32, [u8; 3]);

pub unsafe fn pixel_manip(buffer: *mut Vec<u8>, triangles: &Vec<Tri>, width: i32, height: i32) {
    //a convient way to index. instead of i = (col * height * 3) + row + channel; 
    // should be less memory reads, but i dont really know. its just a convience thing
    let mut counter = 0;
    //could be reinitialized for every pixel, but better to just modify it
    let mut color = [0,0,0];
    //counting which row
    for row in 0..height {
         //counting how far down the row
        for col in 0..width {

            let mut x_buffer: Vec<x_color> = Vec::new();

            //a line originating at the pixel, and moving straight out in the x direction
            let line = Line::new( 
                        Point::new(1.0, 0.0 , 0.0), Point::new(0.0, col as f32, row as f32)
                    );

            /*previous 2 loops get us to the right pixel. 
            this itterator checks every Tri to see if the pixel is in it.
            will modify color to be the color of the last tri the pixel is within.*/
            for tri in triangles {
                let (is_in_tri, location) = tri.intersects(&line);
                if is_in_tri {
                    x_buffer.push( (location.x(), tri.color));
                }
            }

            if x_buffer.len() == 0 {
                //sets "color" to the current color of the pixel
                color = [ (*buffer)[counter],(*buffer)[counter +1], (*buffer)[counter +2] ];
            } else {
                let mut closest = x_buffer[0];
                for xb in x_buffer {
                    if xb.0 < closest.0 {
                        closest = xb; 
                    }
                }
                color = closest.1;
            }
            

            
        
            //counting which color channel (RGB)
            for channel in 0..3 {
                
                let mut pix = &mut (*buffer)[ counter];
                *pix = color[channel];
                
                counter += 1;
            }
        }
    }

}








pub unsafe fn clear(buffer: *mut Vec<u8>, color: [u8; 3] ) {
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