pub fn display_array() {
    let integer_arr = [1, 2, 3, 4, 5];
    println!("integer array: {:?}", integer_arr);

    let integer_arr = [5;5];
    println!("integer array: {:?}", integer_arr);
}


pub fn display_vector() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    println!("v is: {:?}", v);
}