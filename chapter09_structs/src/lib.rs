#[derive(Debug)]
pub struct PointTuple(f64, f64, f64);

impl PointTuple {
    pub fn new(x: f64, y: f64, z: f64) -> PointTuple {
        PointTuple(x, y, z)
    }

    pub fn distance(&self, p: &PointTuple) -> f64 {
        let x_square = (self.0 - p.0).powf(2.0);
        let y_square = (self.1 - p.1).powf(2.0);
        let z_square = (self.2 - p.2).powf(2.0);

        (x_square + y_square + z_square).sqrt()
    }
}


#[derive(Debug)]
pub struct PointStruct {
    x: f64,
    y: f64,
    z: f64,
}

impl PointStruct {
    pub fn new(x: f64, y: f64, z: f64) -> PointStruct {
        PointStruct {x, y, z}
    }

    pub fn distance(&self, p: &PointStruct) -> f64 {
        let x_square = (self.x - p.x).powf(2.0);
        let y_square = (self.y - p.y).powf(2.0);
        let z_square = (self.z - p.z).powf(2.0);

        (x_square + y_square + z_square).sqrt()
    }
}