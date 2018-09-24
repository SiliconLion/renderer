extern crate rand;
use rand::random;

// ########   #######  #### ##    ## ######## 
// ##     ## ##     ##  ##  ###   ##    ##    
// ##     ## ##     ##  ##  ####  ##    ##    
// ########  ##     ##  ##  ## ## ##    ##    
// ##        ##     ##  ##  ##  ####    ##    
// ##        ##     ##  ##  ##   ###    ##    
// ##         #######  #### ##    ##    ##   


//a point or vector centered at the origin
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    //x,y,z 
    pub coords: [f32; 3]
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
        let x = (random::<f32>() * max_width as f32).abs();
        let y = (random::<f32>() * max_height as f32).abs();
        let z = (random::<f32>() * max_depth as f32).abs();
        Point{ coords: [x,y,z] }
    }
}



// ##       #### ##    ## ######## 
// ##        ##  ###   ## ##       
// ##        ##  ####  ## ##       
// ##        ##  ## ## ## ######   
// ##        ##  ##  #### ##       
// ##        ##  ##   ### ##       
// ######## #### ##    ## ######## 


#[derive(Clone, Debug)]
pub struct Line {
    //  t * <vector> + (origin)     where t is distance along the line
    pub vector: Point,
    pub origin: Point
    
}

impl Line {
    pub   fn new(vector: Point, origin: Point) -> Line {
        Line { vector, origin}
    }
}



// ######## ########  #### 
//    ##    ##     ##  ##  
//    ##    ##     ##  ##  
//    ##    ########   ##  
//    ##    ##   ##    ##  
//    ##    ##    ##   ##  
//    ##    ##     ## #### 

#[derive(Debug)]
pub struct Tri {
    //in the order  A, B, C
    pub points: [Point; 3],
    pub normal: Point,
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