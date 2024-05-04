fn main() {
    let origin = chapter09_structs::PointTuple::new(0.0, 0.0, 0.0);
    let p = chapter09_structs::PointTuple::new(2.0, 3.0, 4.0);
    println!("distance of {:?} to {:?} is {}", origin, p, p.distance(&origin));

    let origin = chapter09_structs::PointStruct::new(0.0, 0.0, 0.0);
    let p = chapter09_structs::PointStruct::new(2.0, 3.0, 4.0);
    println!("distance of {:?} to {:?} is {}", origin, p, p.distance(&origin));
}
