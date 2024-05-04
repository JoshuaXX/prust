/**
 * LISP programmers know the value of everything, but the cost of nothing.
 *                     -- Alan Perlis
 *
 * Expressions
 *
 */



fn demo() {
    println!("function type fn() -> ()");
}


fn main() {
    let function_demo: fn() -> () = demo;
    function_demo();


    // if expression
    let num = if true { 10 } else { 20 };
    println!("{}", num);

    // match expression
    let num = 120u32;
    match num {
        0..=9 => println!("one digit"),
        10..=99 => println!("two digit"),
        x => println!("large than 100, it is {}", x),
    }

    // block expression
    let num = {
        println!("block expression");
        10
    };
    println!("num is: {}", num);

    let mut count = 0;
    while count < 5 {
        println!("while count: {}", count);
        count += 1;
    }

    for i in 0..5 {
        println!("for count: {}", i);
    }

    count = 0;
    loop {
        if count == 5 {
            break;
        }
        println!("loop count: {}", count);
        count += 1;
    }


}
