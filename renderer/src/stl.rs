use std::fs::OpenOptions;
use stl_io;
use geometry::*;


fn point_from_stl(v: &stl_io::Vertex) -> Point {
    let v = *v;
    //Point{ coords: v}
    Point::new(
        (v[0] * 10.0) + 100.0,
        (v[1] * 10.0) + 100.0,
        (v[2] * 10.0) + 100.0
        )
}


fn tri_from_indexed_triangle(
            vert_locations: [usize; 3],  
            n: stl_io::Normal,
            verts: &Vec<stl_io::Vertex>) -> Tri {
    let mut p = Vec::new();
    for i in 0..3 {
        let vert = &verts[vert_locations[i]];
        p.push(point_from_stl(vert));
    }

    let normal = point_from_stl(&n);
    Tri {points: [p[0], p[1], p[2]], normal, color: [
        (n[0] * 255.0) as u8,
        (n[1] * 255.0) as u8,
        (n[2] * 255.0) as u8
    ] }
}


pub fn vec_from_stl(path: &String) -> Vec<Tri> {
    let mut file = OpenOptions::new()
                .read(true)
                .open(path)
                .unwrap();
                
    let stl_io::IndexedMesh {vertices: master_list, faces} = stl_io::read_stl(&mut file).unwrap();

    let mut tris: Vec<Tri> = Vec::new();
    for i in 0..faces.len() {
        let t = &faces[i];
        tris.push(tri_from_indexed_triangle(
            t.vertices, t.normal, &master_list
        ));
    }

    tris
}

