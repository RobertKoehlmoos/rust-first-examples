// Enums are types with a few definite values
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(direction: Movement) {
    // Perform action depending on info
    match direction {
        Movement::Up => println!("Avatar moving up!"),
        Movement::Down => println!("Avatar moving down!"),
        Movement::Left => println!("Avatar moving left!"),
        Movement::Right => println!("Avatar moving right!")
    }
}

pub fn run() {
    let avatar1: Movement = Movement::Left;
    let avatar2: Movement = Movement::Right;
    let avatar3: Movement = Movement::Up;
    let avatar4: Movement = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}