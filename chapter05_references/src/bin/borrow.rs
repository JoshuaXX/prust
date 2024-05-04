/**
 * In Rust：
 *      - references are created explicitly with the & operator；
 *      - dereferenced explicitly with the * operator
 * 
 * Since references are so widely used in Rust, 
 *      - the "." operator implicitly dereferences its left operand, 
 *        if needed.
 *      - The "." operator can also implicitly borrow a reference 
 *        to its left operand.
 * 
 * The "." operator, which borrows and dereferences implicitly.
 * 
 * Rust permits references to references:
 *       let r: i32 = 100;
 *       let rr: &i32 = &r;
 *       let rrr: &&i32 = &rr;
 * The "." operator follows as many references as it takes to find its target
 * 
 * 
 * 
 * 
 * 
 *         let x = 10;
 *         let y = 10;
 *         
 *         let rx = &x;
 *         let ry = &y;
 *         
 *         let rrx = &rx;
 *         let rry = &ry;
 *         
 *         assert!(rrx <= rry);
 *         assert!(rrx == rry);
 * 
 * the == operator follows all the references and performs the comparison 
 * on their final targets
 * 
 * If you actually want to know whether two references point to the same memory, 
 * you can use std::ptr::eq, which compares them as addresses
 *          assert!(rx == ry);              // their referents are equal
 *          assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses
 * 
 * 
 * 
 * Note that the operands of a comparison must have exactly the same type, 
 * including the references：
 *           assert!(rx == rrx);    // error: type mismatch: `&i32` vs `&&i32`
 *           assert!(rx == *rrx);   // this is okay
 */


use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}


fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    show(&table);


    let mut num = 32;
    let m = &mut num;
    *m = 100;
    println!("num changed to: {}", num);


    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[0..=3];
    println!("{:?}", arr_slice);
}
