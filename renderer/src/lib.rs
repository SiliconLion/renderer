use std::ops;
extern crate rand;
use rand::Rand;
extern crate num;
extern crate stl_handler;





//a point or vector centered at the origin
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    //x,y,z 
    coords: [f32; 3]
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x as f32, $y as f32, $z as f32)
    };
}


impl Point {

    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point{ coords: [x,y,z] }
    }

    pub fn x(&self) -> f32 {
        self.coords[0]
    }

    pub fn y(&self) -> f32 {
        self.coords[1]
    }

    pub fn z(&self) -> f32 {
        self.coords[2]
    }

    pub fn random(max_width: i32, max_height: i32, max_depth: i32) -> Point {
        use rand::random;
        let x = (random::<f32>() * max_width as f32).abs();
        let y = (random::<f32>() * max_height as f32).abs();
        let z = (random::<f32>() * max_depth as f32).abs();
        Point{ coords: [x,y,z] }
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { coords: [
            self.x() + other.x(), 
            self.y() + other.y(), 
            self.z() + other.z()
        ]}
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        // *self = Point { coords: [
        //     self.x() + other.x(), 
        //     self.y() + other.y(), 
        //     self.z() + other.z()
        // ]};
        *self = *self + other;
    }
}


impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        // other = *other;
        Point { coords: [
            self.x() - other.x(), 
            self.y() - other.y(), 
            self.z() - other.z()
        ]}
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        // *self = Point { coords: [
        //     self.x() - other.x(), 
        //     self.y() - other.y(), 
        //     self.z() - other.z()
        // ]};
        *self = *self - other;
    }
}

impl Point { 

    fn dot_product(&self, p: &Point) -> f32 {
        let mut sum = 0.0;
        for c in 0..self.coords.len() {
            sum += self.coords[c] * p.coords[c];
        }
        sum
    }


    // using this method to find cross product
    //
    //          |i  j  k  | i  j
    //   <self> |ax ay az | ax ay
    //  <other> |bx by bz | bx by

    fn cross_product(&self, other: &Point) -> Point {
        let positive = Point::new(
                self.coords[1] * other.coords[2],
                self.coords[2] * other.coords[0],
                self.coords[0] * other.coords[1]
            );
        let negative = Point::new(
                self.coords[2] * other.coords[1],
                self.coords[0] * other.coords[2],
                self.coords[1] * other.coords[0]
            );

        positive - negative

    }

    fn distance_from_origin(&self) -> f32 {
        let components = (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z());

        components.sqrt()
    }

    // angle in the form ∠abc
    fn angle_between(a: &Point, b: &Point, c: &Point ) -> f32 {
        let a = *a; 
        let b = *b;
        let c = *c;

        //recenters angle at center
        let ab = a - b;
        let cb = c - b;

    

        let cos_theta = ab.dot_product(&cb) / (ab.distance_from_origin() * cb.distance_from_origin());

        let theta = cos_theta.acos();

        theta
    }

    fn normal_from(a: &Point, b: &Point, c: &Point ) -> Point {
        let a = *a; 
        let b = *b;
        let c = *c;

        let p = a - b;
        let r = c - b;
        r.cross_product(&p)
    }

}

impl Point {
    
    fn from_stl_vert(vert: &stl_handler::Vertex) -> Point {
        let vert = *vert;
        point!(vert.x(), vert.y(), vert.z())
    }
    
}














#[derive(Clone, Debug)]
pub struct Line {
    //  t * <vector> + (origin)     where t is distance along the line
    vector: Point,
    origin: Point
    
}

impl Line {
    pub   fn new(vector: Point, origin: Point) -> Line {
        Line { vector, origin}
    }
}


impl Line {

    fn point_from_t(&self, t: f32) -> Point {
        let mut p = Point::new(
            self.vector.x() * t,
            self.vector.y() * t,
            self.vector.z() * t
        );

        p += self.origin;
        
        p
        
    }
}














#[derive(Debug)]
pub struct Tri {
    //in the order  A, B, C
    points: [Point; 3],
    normal: Point,
    pub color: [u8; 3]
}

impl Tri {
    pub fn new(A: Point, B: Point, C: Point, color: [u8; 3]) -> Tri {
        Tri {
            points: [A, B, C],
            normal: Point::normal_from(&A, &B, &C),
            color
        }
    }

    


    //the bool is if the point lies inside the triangle, and the Point 
    //is where the line intersects the plane regardless of it is inside the triangle
    pub fn intersects(&self, line: &Line) -> (bool, Point) {
        //see https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection for whats being calculated

        let p0 = (*self).points[0].clone();
        let l0 = line.origin.clone();
        //nermerator and denomerator respectively
        let numer = (p0 - l0).dot_product( &self.normal);
        let denom = line.vector.dot_product( &self.normal);

        let d = numer / denom ;
        //where line intersects plane
        let intersection_point = line.point_from_t(d);

        //finding if it it lies withing the triangle (this is where we leave wikipedia)
        let mut inside = true;

        for i in 0..3 {
            let theta = Point::angle_between(
                    &self.points[i],
                    &self.points[ (i + 1) % 3 ],
                    &self.points[ (i + 2) % 3 ],
                );
            let rho = Point::angle_between(
                    &self.points[i],
                    &self.points[ (i + 1) % 3 ],
                    &intersection_point
                );

            if rho > theta { inside = false }
        }

        (inside, intersection_point)
        
    }

    pub fn random(width: u32, height: u32, depth: u32) ->  Tri {
        Tri::new(
            Point::random(width as i32,height as i32, depth as i32),
            Point::random(width as i32,height as i32, depth as i32),
            Point::random(width as i32,height as i32, depth as i32),
            [
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>()
            ]
        )
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













#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn point_addition() {
        let a = Point::new(5.0,5.0,5.0);
        let b = Point::new(5.0,5.0,5.0);
        let c = a + b;
        assert_eq!(c, Point::new(10.0,10.0,10.0));
    }

    #[test]
    fn point_subtraction() {
        let a = Point::new(5.0,5.0,5.0);
        let b = Point::new(3.0,3.0,3.0);
        let c = a - b;
        assert_eq!(c, Point::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn point_add_assign_and_subtract() {
        let mut a = Point::new(5.0,5.0,5.0);
        let mut b = Point::new(5.0,5.0,5.0);
        let c = Point::new(1.0, 1.0, 1.0);
        a += c;
        b -= c;
        assert_eq!(a, Point::new(6.0,6.0,6.0));
        assert_eq!(b, Point::new(4.0,4.0,4.0));
    }

    #[test]
    fn point_dot_product() {
        let a = Point::new(1.0, 1.0, 1.0);
        let b = Point::new(2.0, 2.0, 2.0);
        assert_eq!(a.dot_product(&b), 6.0);
    }

    #[test]
    fn point_cross_product() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(4.0, 5.0, 6.0);
        assert_eq!(a.cross_product(&b), Point::new(-3.0, 6.0, -3.0));

        let p = Point::new(3.0, -2.0, 7.0);
        let q = Point::new(2.0, 3.0, -5.0);
        assert_eq!(p.cross_product(&q) , Point::new(-11.0, 29.0, 13.0) ); 
    }

    #[test]
    fn point_distance_from_origin() {
        let a = Point::new(3.0, 4.0, 5.0);
        assert_eq!(a.distance_from_origin(), 5.0 * 2.0_f32.sqrt());
    }

    #[test]
    fn point_angle_betwee() {
        let a = Point::new(1.0, 2.0, 3.0);
        let b = Point::new(0.0, 0.0, 0.0);
        let c = Point::new(4.0, 5.0, 6.0);
        
        //i belive the more accurate number would be 0.2257261285521736383, but close enough 
        //i think
        assert_eq!(Point::angle_between(&a, &b, &c), 0.225726);
        
    }

    #[test]
    fn point_normal_from() {
        let a = Point::new(1.0, -2.0, 0.0);
        let b = Point::new(3.0, 1.0, 4.0);
        let c = Point::new(0.0, -1.0, 2.0);

        let n = Point::normal_from(&a, &b, &c);
        assert_eq!(n, Point::new(2.0, -8.0, 5.0));
        print!("\n\n\nn: {:?}", n);
    }

    // fn line_point_from_t() {
       
    // }

    #[test]
    fn tri_intersects() {
        let a = point!(-2, -2, 4);
        let b = point!(2, -2, 4);
        let c = point!(0, 2, 4);
        let abc = Tri::new(a,b,c, [0,0,0]);

        let line = Line::new(point!(0,0,1), point!(100,100,100));

        let (b,p) = abc.intersects(&line);
        assert_eq!(b, false);
        //assert_eq!(p, point!(0,0,4));
    }
}