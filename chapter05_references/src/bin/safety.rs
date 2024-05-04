/**
 * We say that the variable’s lifetime must contain or enclose 
 * that of the reference borrowed from it
 * 
 * 
 * 
 * 
 * Similarly, if you store a reference in some data structure, 
 * its lifetime must enclose that of the data structure. 
 * 
 * For example, if you build a vector of references, 
 * all of them must have lifetimes enclosing that of 
 * the variable that owns the vector.
 * 
 */
fn main() {
    // let r;
    // {
    //     let x = 1;
    //     r = &x;
    // }
    // assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy

    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
    unsafe { f(STASH) };
    g(&1);

    /*
     * From smallest’s signature, we can see that its argument 
     * and return value must have the same lifetime, 'a. 
     * 
     * In our call, the argument &v must not outlive v itself, 
     * yet smallest’s return value must live at least as long as min
     */
    // let min;
    // {
    //     let v = vec![5, 4, 9 ,1, 3];
    //     min = smallest(&v);
    // }
    // println!("{}", min);

    let min;
    let v = vec![5, 4, 9 ,1, 3];
    min = smallest(&v);
    println!("{}", min);
}

const PI: f64 = 3.1415926;

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe { STASH = p };
    println!("{}", PI);
    unsafe { println!("{}", STASH) };
}


/**
 * 
 *  This could be written more briefly: 
 *      fn g(p: &i32),
*   but let's write out the lifetimes for now.
 * 
 *  **Important**
 *    the lifetime of p’s reference is some 'a, 
 *    which could be anything, as long as it encloses 
 *    the call to f
 */
fn g<'a>(p: &'a i32) { 
    println!("{}", p);
}



/**
 *  We’ve omitted lifetimes from that function’s signature in the usual way. 
 *  When a function takes a single reference as an argument and 
 *  returns a single reference, Rust assumes that the two must 
 *  have the same lifetime. Writing this out explicitly would give us:
 *       fn smallest<'a>(v: &'a [i32]) -> &'a i32 { ... }
 */
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}