pub fn display_character() {
    let c = 'a';
    println!("\'a\': {}", c);
    let c = '\x64';
    println!("\\x64: {}", c);
    let c = '\u{5640}';
    println!("\\u{{5640}}: {}", c);
    let c = b'\x64';
    println!("b'\\x64': {}", c);
}