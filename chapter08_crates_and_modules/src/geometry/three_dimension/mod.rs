#[derive(Debug)]
pub enum Quadrant {
    FIRST,
    SECOND,
    THIRD,
    FORTH,
    FIFTH,
    SIXTH,
    SEVENTH,
    EIGHTH,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x,
            y,
            z,
        }
    }

    pub fn quadrant(&self) -> Quadrant {
        if self.x >= 0f64 && self.y >= 0f64 && self.z >= 0f64 {
            return Quadrant::FIRST;
        } 

        if self.x <= 0f64 && self.y >= 0f64 && self.z >= 0f64 {
            return Quadrant::SECOND;
        } 

        if self.x <= 0f64 && self.y <= 0f64 && self.z >= 0f64 {
            return Quadrant::THIRD;
        } 

        if self.x >= 0f64 && self.y <= 0f64 && self.z >= 0f64 {
            return Quadrant::FORTH;
        } 

        if self.x >= 0f64 && self.y >= 0f64 && self.z <= 0f64 {
            return Quadrant::FIFTH;
        } 

        if self.x <= 0f64 && self.y >= 0f64 && self.z <= 0f64 {
            return Quadrant::SIXTH;
        } 

        if self.x <= 0f64 && self.y <= 0f64 && self.z <= 0f64 {
            return Quadrant::SEVENTH;
        } 

        if self.x >= 0f64 && self.y <= 0f64 && self.z <= 0f64 {
            return Quadrant::EIGHTH;
        }  

        Quadrant::UNKNOWN
    }
}




#[derive(Debug)]
pub struct Sphere {
    pub diameter: f64,
    pub origin: Point,
}