pub fn run() {
    greet("hello", "kat");
    let get_sum = add(5, 7);
    println!("Sum: {}", get_sum);

    // Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(4, 8));
}

fn greet(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}