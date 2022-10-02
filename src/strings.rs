pub fn run() {
    let mut hello = String::from("Hello ");
    
    // Get length
    println!("Length {}", hello.len());

    hello.push('w');

    hello.push_str("orld!");

    // capacity in bytes
    println!("{}", hello.capacity());

    println!("Does it contain World? {}", hello.contains("world"));

    println!("{}", hello.replace("world", "there"));

    for word in hello.split_ascii_whitespace() {
        println!("{}", word);
    }
    
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(s.len(), 2);
    assert_eq!(s.capacity(), 10);
}