use geometry::*;
use math::*;


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