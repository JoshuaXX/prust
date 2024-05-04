/**
 * 1. Once an item is marked pub(crate), 
 *    meaning that it is available anywhere inside this crate.
 * 2. If you want an item in a nested module to be visible to other crates, 
 *    be sure to mark it and all enclosing modules as public. 
 * 3. It’s also possible to specify pub(super), 
 *    making an item visible to the parent module only, and pub(in <path>), 
 *    which makes it visible in a specific parent module and its descendants. 
 */

fn main() {
    println!("Hello, world!");

    let p = chapter08_crates_and_modules::geometry::three_dimension::Point::new(-1.0, -1.0, -1.0);
    println!("point {:?} is in {:?} quadrant.", p, p.quadrant());

}
