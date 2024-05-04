use chapter09_structs::*;

fn main() {
    let origin = PointTuple::new(0.0, 0.0, 0.0);
    let p = PointTuple::new(2.0, 3.0, 4.0);
    println!("distance of {:?} to {:?} is {}", origin, p, p.distance(&origin));

    let origin = PointStruct::new(0.0, 0.0, 0.0);
    let p = PointStruct::new(2.0, 3.0, 4.0);
    println!("distance of {:?} to {:?} is {}", origin, p, p.distance(&origin));
}
