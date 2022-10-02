use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command: String = args[1].clone();
    let name = "kat";
    let status = "100%";
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Command not recognized");
    }

    // println!("Command {:?}", command);
}