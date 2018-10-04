use geometry::*;
use math::*;

//  ######   ######  ######## ##    ## ######## 
// ##    ## ##    ## ##       ###   ## ##       
// ##       ##       ##       ####  ## ##       
//  ######  ##       ######   ## ## ## ######   
//       ## ##       ##       ##  #### ##       
// ##    ## ##    ## ##       ##   ### ##       
//  ######   ######  ######## ##    ## ########


fn translate_point(delta_x: f32, delta_y: f32, delta_z: f32, p: &mut Point) {
    p.coords[0] += delta_x;
    p.coords[1] += delta_y;
    p.coords[2] += delta_z;
}

pub fn translate_triangles(delta_x: f32, delta_y: f32, delta_z: f32, triangles: &mut Vec<Tri>) {
    for tri in triangles {
        for point in &mut tri.points {
            translate_point(delta_x, delta_y, delta_z, point);
        }
    }

}

//if the point is thought of as a vector, it will be 'scaller' times longer
fn scale_point(scaler: f32, p: &mut Point) {
    *p *= Point {coords: [scaler, scaler, scaler]};
}

pub fn scale(scaler: f32, triangles: &mut Vec<Tri>) {
    for tri in triangles {
        for p in tri.points.iter_mut() {
            scale_point(scaler, p);
        }
    }
}


fn flip_point_z(p: &mut Point) {
    p.coords[2] = p.coords[2] * -1.0;
    
}

pub fn flip_z(triangles: &mut Vec<Tri>) {
    for tri in triangles {
        for p in tri.points.iter_mut() {
            flip_point_z(p);
        }
        tri.normal *= Point{coords: [-1.0, -1.0, -1.0]};
    }
}



