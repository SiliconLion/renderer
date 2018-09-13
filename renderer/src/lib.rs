use std::ops;
extern crate rand;
use rand::Rand;
extern crate num;

//a point or vector centered at the origin
#[derive(Copy, Clone)]
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

    pub fn random(max_width: i32, max_height: i32) -> Point {
        use rand::random;
        let x = random::<f32>() * max_width as f32;
        let y = random::<f32>() * max_height as f32;
        let z = random::<f32>() * max_height as f32;
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


impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { coords: [
            self.x() - other.x(), 
            self.y() - other.y(), 
            self.z() - other.z()
        ]}
    }
}

impl Point { 

    fn dot_product(&self, p: Point) -> f32 {
        let mut sum = 0.0;
        for c in 0..self.coords.len() {
            sum += self.coords[c] * p.coords[c];
        }
        sum
    }


    // using this method to find cross product
    //
    //          |i  j  k  | i  j
    //      <a> |ax ay az | ax ay
    //      <b> |bx by bz | bx by

    fn cross_product(&self, p: Point) {

    }

}





// pub struct Tri {
//     //in the order  A, B, C
//     points: [Point; 3],
//     normal: Point,
//     pub color: [u8; 3]
// }

// impl Tri {
//     pub fn new(A: Point, B: Point, C: Point, color: [u8; 3]) -> Tri {
//         let AB = math_line::new(&A, &B);
//         let BC = math_line::new(&B, &C);
//         let CA = math_line::new(&C, &A);
//         //print!("AB = {:?}\nBC = {:?}\nCA = {:?}\n", AB, BC, CA);
//         Tri{ points:[A,B,C], lines :[AB,BC,CA], color }
//     }

//     pub fn is_inside(&self, P:&Point) -> bool {
//         let mut P_is_inside = true;
//         for (i,line) in self.lines.into_iter().enumerate() {
//             //the vertex in the triangle not used in the line
//             let point_not_used_in_line = &self.points[(i + 2) % 3];
//             let needs_to_be_above: bool = line.is_above(point_not_used_in_line);
//             let is_P_above = line.is_above(P);
            
//             if is_P_above != needs_to_be_above {
//                 P_is_inside = false;
//             }
//         }
//         P_is_inside

        
//     }

//     pub fn random(width: u32, height: u32) ->  Tri {
//         Tri::new(
//             Point::random(width as i32,height as i32),
//             Point::random(width as i32,height as i32),
//             Point::random(width as i32,height as i32),
//             [
//                 rand::random::<u8>(),
//                 rand::random::<u8>(),
//                 rand::random::<u8>()
//             ]
//         )
//     }
// }



// #[derive(PartialEq,Debug)]
// struct math_line {
//     //in the event it is a vertical line
//     is_vertical: bool,
//     /*line in the form y = mx + b, except when it is vertical.
//     when it is vertical, b represents the x value */ 
//     m: f64,
//     b: f64
    
// }


// impl math_line {
//     fn new(A: &Point, B: &Point) -> math_line {
        
//         let rise = B.y() - A.y();
//         let run = B.x() - A.x();
//         if run == 0.0 {
//             math_line { 
//                 is_vertical: true,
//                 m: 0.0,
//                 b: A.x() as f64,
//             }
//         } else {
//             let m = rise as f64 / run as f64;
//             let b = A.y() as f64 - (m * A.x() as f64);
//             math_line {m, b, is_vertical: false}
//         }
        
//     }

//     // fn from_m_and_b(m: f64, b: f64) -> math_line {
//     //     math_line {m, b}
//     // }

//     fn y_from_x(&self, x: f32) -> f64 {
//         (self.m * x as f64) + self.b
//     }

//     /*returns true if the point is above the line, or, if the line is
//     vertical, returns true if it is to the right*/
//     fn is_above(&self, P: &Point) -> bool {
//         if self.is_vertical {
//             let to_the_right: bool = P.x() as f64 > self.b;
//             return to_the_right;
//         } else {
//             let fx = self.y_from_x( P.x() );
//             let is_above = P.y() as f64 > fx;
//             //print!("P.x: {}\nP.y: {}\nfx: {}\nis_above: {}\n",P.x, P.y, fx, is_above);
//             is_above
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
    
//     use Point;
//     use math_line;
//     use Tri;

//     #[test]
//     fn correct_line() {
//         let A = Point::new(200,200);
//         let B = Point::new(400,400);
//         let R = Point::new(200,200);
//         let S = Point::new(400,300);
//         let AB = math_line::new(&A,&B);
//         let RS = math_line::new(&R,&S);
//         print!("AB: {:?}\nRS: {:?}", AB, RS);

//         assert_eq!(AB.m, 1.0);
//         assert_eq!(AB.b, 0.0);
//         assert_eq!(RS.m, 0.5);
//         //assert_eq!(RS.b, 0.0);
//     }


//     #[test]
//     // fn correct_y_from_x_from_m_and_b() {
//     //     let line = math_line::from_m_and_b(3.2, 2.1);
//     //     let output = line.y_from_x(0);
//     //     assert_eq!(output, 2.1);
//     // }

//     // #[test]
//     // fn correct_line_from_points() {
//     //     let A = Point::new(12,5);
//     //     let B = Point::new(30,6);
//     //     let output = math_line::new(&A, &B);
//     //     let correct = math_line::from_m_and_b(0.05555555555555555,4.333333333333333);
//     //     print!("{:?}", output);
//     //     //assert_eq!(output, correct);
//     // }

//     #[test]
//     fn is_above_is_correct() {
//         let P = Point::new(100,100);
//         let A = Point::new(12,5);
//         let B = Point::new(30,6);
//         let line = math_line::new(&A,&B);
//         let output = line.is_above(&P);
        
//         assert_eq!(output, true);
        
//     }
//     #[test]
//     fn is_below_is_correct() {
//         let P = Point::new(12,1);
//         let A = Point::new(12,5);
//         let B = Point::new(30,6);
//         let line = math_line::new(&A,&B);
//         print!("{:?}\n", line);
//         let output = line.is_above(&P);

//         assert_eq!(output, false);
//     }

//     #[test]
//     fn is_in_triangle() {
//         let A = Point::new(1,10);
//         let B = Point::new(34,8);
//         let C = Point::new(20,0);
//         let triangle = Tri::new(A,B,C,[0,0,0]);
//         let P = Point::new(20, 5);
//         let output = triangle.is_inside(&P);
//         assert_eq!(output, true);
//     }
//     #[test]
//     fn is_in_vertical_tri() {
//         let triangle = Tri::new(
//             Point::new(100,100),
//             Point::new(0,100),
//             Point::new(100,0),
//             [100,100,255]
//         );
//         let P = Point::new(75,75);
//         let output = triangle.is_inside(&P);
//         assert_eq!(output, true);
//     }
    
// }

