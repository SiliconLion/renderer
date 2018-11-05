use geometry::*;
use std::ops;



// ########   #######  #### ##    ## ######## 
// ##     ## ##     ##  ##  ###   ##    ##    
// ##     ## ##     ##  ##  ####  ##    ##    
// ########  ##     ##  ##  ## ## ##    ##    
// ##        ##     ##  ##  ##  ####    ##    
// ##        ##     ##  ##  ##   ###    ##    
// ##         #######  #### ##    ##    ##    

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

impl ops::Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point { coords: [
            self.x() * other.x(), 
            self.y() * other.y(), 
            self.z() * other.z()
        ]}
    }
}

impl ops::MulAssign for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = *self * other;
    }
}


impl ops::Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point { coords: [
            self.x() / other.x(), 
            self.y() / other.y(), 
            self.z() / other.z()
        ]}
    }
}

impl ops::DivAssign for Point {
    fn div_assign(&mut self, other: Point) {
        *self = *self / other;
    }
}

impl Point { 

    pub fn dot_product(&self, p: &Point) -> f32 {
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

    pub fn cross_product(&self, other: &Point) -> Point {
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

    pub fn distance_from_origin(&self) -> f32 {
        let components = (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z());

        components.sqrt()
    }


    pub fn normal_from(a: &Point, b: &Point, c: &Point ) -> Point {
        let a = *a; 
        let b = *b;
        let c = *c;

        let p = a - b;
        let r = c - b;
        r.cross_product(&p)
    }

}







impl Point {
    // angle in the form ∠abc
    pub fn angle_between(a: &Point, b: &Point, c: &Point ) -> f32 {
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

    pub fn distance_between(a: &Point, b: &Point) -> f32 {
        let delta_x = a.x() - b.x();
        let delta_y = a.y() - b.y();
        let delta_z = a.z() - b.z();

        let sum_of_squares = (delta_x * delta_x) + (delta_y * delta_y) + (delta_z * delta_z);
        let distance = sum_of_squares.sqrt();
        
        distance
    }

}


// ##       #### ##    ## ######## 
// ##        ##  ###   ## ##       
// ##        ##  ####  ## ##       
// ##        ##  ## ## ## ######   
// ##        ##  ##  #### ##       
// ##        ##  ##   ### ##       
// ######## #### ##    ## ######## 

impl Line {

    pub fn point_from_t(&self, t: f32) -> Point {
        let mut p = Point::new(
            self.vector.x() * t,
            self.vector.y() * t,
            self.vector.z() * t
        );

        p += self.origin;
        
        p
        
    }

//seems to be wrong. validate correctness before using
//assumes point is on line. answer is meaningless if it isnt
    pub fn t_from_point(&self, p: Point) -> f32 {
        let offset = p.x() - self.origin.x();
        let t = offset / self.vector.x();

        t
    }
}

// ######## ########  #### 
//    ##    ##     ##  ##  
//    ##    ##     ##  ##  
//    ##    ########   ##  
//    ##    ##   ##    ##  
//    ##    ##    ##   ##  
//    ##    ##     ## #### 


impl Tri {

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
                ).abs();
            let rho = Point::angle_between(
                    &self.points[i],
                    &self.points[ (i + 1) % 3 ],
                    &intersection_point
                ).abs();

            if rho + 0.0004 > theta  { inside = false }
            
        }

        (inside, intersection_point)
        
    }

    
    
}