pub fn run() {
    println!("{}", std::i32::MAX);
    println!("{}", std::f32::MAX);

    let is_active = true;
    let is_greater = 10 > 5;

    let a1: char = 'a';
    let face: char = '\u{1f601}';
    println!("{:?}", (is_active, is_greater, a1, face));
}