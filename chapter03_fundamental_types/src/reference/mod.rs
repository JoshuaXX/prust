pub fn display_reference() {
    let mut x = 100;
    let x_ref = &mut x;
    println!("{:p}", x_ref);

    let raw_p = x_ref as *mut i32;
    unsafe {
        *raw_p = 200;
    }
    println!("{}", x);
}