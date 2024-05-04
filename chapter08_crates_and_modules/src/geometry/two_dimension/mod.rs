///
/// document for [`Point`]
/// 
/// ```
/// use chapter08_crates_and_modules::geometry::two_dimension::Point;
/// 
/// let p = Point {x: 0.1, y: 0.1};
/// println!("p is: {:?}", p);
/// ```
/// 
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}



///
/// 
/// 
#[derive(Debug)]
pub struct Circle {
    pub diameter: f64,
    pub origin: Point,
}