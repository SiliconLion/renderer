extern crate stl_handler;
use geometry::*; 
//use super::macros::*;

impl Point {
    
    fn from_stl_vert(vert: &stl_handler::Vertex) -> Point {
        let vert = *vert;
        point!(vert.x(), vert.y(), vert.z())
    }
    
}

impl Tri {

    fn from_stl( stl_tri: &stl_handler::Triangle) -> Tri {
        let normal = Point::from_stl_vert(&stl_tri.normal());
        let verts = stl_tri.verts();
        let points = [
            Point::from_stl_vert(&verts[0]),
            Point::from_stl_vert(&verts[1]),
            Point::from_stl_vert(&verts[2])
        ];

        //should actually decode the atribute bytes but whatever
        let color = [255, 0, 0];

        Tri { normal, points, color}
    }

    pub fn vec_from_stl(path: String) -> Vec<Tri> {
        let (_,_, triangles) = stl_handler::decode_stl(path).expect("bad file path probs");

        let mut buffer: Vec<Tri> = Vec::new();

        for t in triangles {
            buffer.push(Tri::from_stl(&t));
        }

        print!("{:?}", buffer);
        buffer
    }
}

