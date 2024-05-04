

fn main() {
    // panic
    // 1. Out-of-bounds array access
    // 2. Integer division by zero
	// 3. Calling .expect() on a Result that happens to be Err
	// 4. Assertion failure
    // 5. panic!()

    // 1. Out-of-bounds array access
    let numbs = [1, 2, 3, 4];
    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("read error");
    let index: usize = index.trim().parse().expect("not a int");
    println!("numbs 10: {}", numbs[index]);

    // 2. Integer division by zero
    let mut divident = String::new();
    match std::io::stdin().read_line(&mut divident) {
        Ok(size) => println!("read {} characters.", size),
        Err(err) => {
            println!("the error is: {}", err.to_string());
            return;
        }
    }
    let divident: isize = match divident.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("{}", err.to_string());
            return;
        },
    };
    println!("10/{} = {}", divident, 10/divident);


    //  Rust can either unwind the stack when a panic happens 
    //  or abort the process. Unwinding is the default
    //
    //  

}