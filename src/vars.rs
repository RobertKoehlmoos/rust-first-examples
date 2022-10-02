pub fn run() {
    let name = "kat";
    let mut age = 25;

    println!("my name is {} and I am {}", name, age);
    age = 26;

    println!("my name is {} and I am {}", name, age);

    // Define constant

    const ID: i32 = 1;
    println!("ID:{}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("kate", 25);
    println!("{} is {}", my_name, my_age);
}