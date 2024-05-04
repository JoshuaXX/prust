
pub fn demo_result() -> Result<i32, String> {
    Ok(100)
}

fn main() {
    
    let result = demo_result();
    println!("is the result ok? {}", result.is_ok());
    println!("is the result error? {}", result.is_err());
    
    // Result type -> Option type
    match result.as_ref().ok() {
        Some(result) => {
            println!("the result is Ok({})", result);
            println!("Result type: Ok({}) -> Option type: Some({})", result, result);
        },
        None => println!("None"),
    }

    match result.as_ref().err() {
        Some(result) => {
            println!("the result is Err({})", result);
            println!("Result type: Err({}) -> Option type: Some({})", result, result);
        },
        None => println!("None"),
    }
}