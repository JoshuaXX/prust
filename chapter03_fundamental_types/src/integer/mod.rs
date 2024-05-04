pub fn display_integer() {
    println!("std::u8::MAX: {}", std::u8::MAX);
    println!("std::i8::MAX: {}", std::i8::MAX);
    println!("std::u16::MAX: {}", std::u16::MAX);
    println!("std::i16::MAX: {}", std::i16::MAX);
    println!("std::u32::MA: {}", std::u32::MAX);
    println!("std::i32::MAX: {}", std::i32::MAX);
    println!("std::u64::MAX: {}", std::u64::MAX);
    println!("std::i64::MAX: {}", std::i64::MAX);
    println!("std::u128::MAX: {}", std::u128::MAX);
    println!("std::i128::MAX: {}", std::i128::MAX);
    println!("std::usize::MAX: {}", std::usize::MAX);
    println!("std::isize::MAX: {}", std::isize::MAX);

    assert_eq!(0xffffffffu32 as i32, -1i32);
}