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