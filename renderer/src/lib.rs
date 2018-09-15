use std::ops;
extern crate rand;
use rand::Rand;
extern crate num;





//a point or vector centered at the origin
#[derive(Copy, Clone, Debug)]
pub struct Point {
    //x,y,z 
    coords: [f32; 3]
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
        let x = random::<f32>() * max_width as f32;
        let y = random::<f32>() * max_height as f32;
        let z = random::<f32>() * max_depth as f32;
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
        *self = Point { coords: [
            self.x() + other.x(), 
            self.y() + other.y(), 
            self.z() + other.z()
        ]};
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
        *self = Point { coords: [
            self.x() - other.x(), 
            self.y() - other.y(), 
            self.z() - other.z()
        ]};
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

        // ‖ab ✕ bc‖ = ‖ab‖ ‖cb‖ sin θ 
        // using this common formula we solve for θ
        let cross = ab.cross_product(&cb);
        let cross_len = cross.distance_from_origin();

        let sin_theta = cross_len / (ab.distance_from_origin() * cb.distance_from_origin() );
        let theta = sin_theta.asin();

        theta
    }


}

impl Point {
    
    fn normal_from(a: &Point, b: &Point, c: &Point ) -> Point {
        let a = *a; 
        let b = *b;
        let c = *c;

        let p = a - b;
        let r = c - b;
        p.cross_product(&r)
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
                    &intersection_point,
                    &self.points[i],
                    &self.points[ (i + 1) % 3 ]
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



