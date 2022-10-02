// traditional struct
// struct Color {
//     red: u8,
//     blue: u8,
//     green: u8
// }

// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 255,
    //     blue: 0
    // };
    // c.red = 200;
    // println!("Red {}, blue {}, green {}", c.red, c.blue, c.green);

    // let mut c = Color(255, 0, 0);
    // c.1 = 42;
    // println!("Colors: {} {} {}", c.0, c.1, c.2);

    let mut p: Person = Person::new("Kat", "Smith");
    p.first_name.push('a');
    // println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("Windsor");
    println!("{}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}