pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_is_of_age: bool = true;
    if age >= 21 && check_id || knows_person_is_of_age {
        println!("Bartender says \"What would you like to drink?\"");
    } else if age < 21 && check_id {
        println!("Sorry you have to leave");
    } else {
        println!("I'll need to see your id");
    }

    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}