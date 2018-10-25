use geometry::Point;

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        Point::new($x as f32, $y as f32, $z as f32)
    };
}