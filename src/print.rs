 pub fn run() {
    println!("Hello from the print.rs file!");

    // Basic formatting
    println!("{} is from {}", "Adora", "the underground");
    
    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Kat", "New York", "code");

    // Named Arguments
    println!("{name} likes to {activity}", name = "kat", activity = "code");

    // Placeholder Traits
    println!("Binary {:b}, Octal {:o}, Hex {:x}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true));
 }