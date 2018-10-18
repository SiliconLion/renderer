
use geometry::*;
use math::*;
use std::f32;


/*
a special use of a triangle. 
- the normal is the direction the camera is looking.
- the first two cordiates specify the direction and width of the rows of pixels
- the last coordanate is used to indicate the direction and length of the cols of pixels. This is done by
constructing a line that connects the first and third coordinate. 
That is a col of pixels. parrallel and adjacent rows of pixels lie in the direction of the second coordinate, 
with the last col starting at the second pixel.
-
*/
pub struct ViewPort {
    ray_direction: Point, // a vector aimed in the direction the viewport is looking
    row_pixels: u32,
    col_pixels: u32,
    rays: Vec<Line> //the origin of every ray. starts at the end of the first row, and moves horizontally across it 
    //until it reaches the end of the row, then starts at the begining of the second row and so on.
}

impl ViewPort {
    pub fn new_from_window_dimentions(direction: Point, 
                                    width: u32, 
                                    height: u32,) -> ViewPort {

        //we know exactly how many items it will have, so no need for it to reallocate 
        //multiple times
        let mut rays: Vec<Line> = Vec::with_capacity(  (width *height) as usize );

        for row in 0..height { //every pixel in height corisponds to a row
            for pixel in 0..width { //every pixle in the row (width is the number of pixels in a row)
                let origin = Point::new(row as f32, pixel as f32, 0.0);
                let ray = Line::new(direction, origin);
                rays.push(ray)
            }   
        }



        ViewPort {
            ray_direction: direction,
            row_pixels: width,
            col_pixels: height,
            rays
        }

    }


}

pub fn render(viewport: &ViewPort, pixel_stream: *mut Vec<u8>, scene_geometry: &Vec<Tri>) {
    
    for (i, pixel_ray) in viewport.rays.iter().enumerate() {
        let (intersected, color) = color_of_cosest_tri(&pixel_ray, scene_geometry);
    unsafe {      
            if intersected {
                (*pixel_stream)[i * 3] = color[0];
                (*pixel_stream)[(i * 3) + 1] = color[1];
                (*pixel_stream)[(i * 3) + 2] = color[2];

            } // if it doesnt intersect, then the the color of that pixle is left unchanged
        }

    }
    
}


fn color_of_cosest_tri(ray: &Line, scene_geometry: &Vec<Tri>) -> (bool, [u8; 3] ){
    // distance_buffer.0 is the distance away
    // distance_buffer.1 is the color of that tri
    // distance_buffer.2 is flipped to 'true' if there is an intersection. then when
    //a value is returned, the black can be distinguished from not intersected (useful for a background color )
    let mut distance_buffer = (f32::INFINITY, [0,0,0], false); 
    
    for triangle in scene_geometry {

        let (is_in_tri, intersection_point) = triangle.intersects(&ray);
        if is_in_tri {

            let distance = Point::distance_between(&ray.origin, &intersection_point);
            //below would be prefered to above, as it could tell if something was behind the
            //viewport and thus shouldnt be displayed, but currently doesnt work
            //let distance = ray.t_from_point(intersection_point);
            if (distance < distance_buffer.0)  &&  (distance > 0.0 ) {
                distance_buffer.0 = distance;
                distance_buffer.1 = triangle.color; 
                distance_buffer.2 = true;
            }
        }

    }
    let intersected = distance_buffer.2;
    let color = distance_buffer.1;
    
    (intersected as bool, color)

}  





type z_color = (f32, [u8; 3]);

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

            let mut z_buffer: Vec<z_color> = Vec::new();

            //a line originating at the pixel, and moving straight out in the x direction
            let line = Line::new( 
                        Point::new(0.0, 0.0 , 1.0), Point::new(col as f32, row as f32, 0.0)
                    );

            /*previous 2 loops get us to the right pixel. 
            this itterator checks every Tri to see if the pixel is in it.
            will modify color to be the color of the last tri the pixel is within.*/
            for tri in triangles {
                let (is_in_tri, location) = tri.intersects(&line);
                if is_in_tri {
                    z_buffer.push( (location.z(), tri.color));
                }
            }

            if z_buffer.len() == 0 {
                //sets "color" to the current color of the pixel
                color = [ (*buffer)[counter],(*buffer)[counter +1], (*buffer)[counter +2] ];
            } else {
                let mut closest = z_buffer[0];
                for zb in z_buffer {
                    if zb.0 < closest.0 {
                        closest = zb; 
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