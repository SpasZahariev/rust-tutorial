// Structs -- sed to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String, 
    last_name: String,
}

// lets create some functions associated with the Person struct
impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
             first_name: first.to_string(),
             last_name: last.to_string()}
    }

    //self just references "this" or the object that it is being called on
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn name_to_tuple(&self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut my_color = Color { red: 255, green: 0, blue: 0};

    // my_color.red = 200;
    // my_color.green = 100;
    // println!("Color: {} {} {}", my_color.red, my_color.green, my_color.blue);
    
    // let mut tuple_color = Color(240, 240, 240);
    // println!("Color: {} {} {}", tuple_color.0, tuple_color.1, tuple_color.2);

    let mut my_person = Person::new("Hohn", "Doe");

    println!("Person: {} {}", my_person.first_name, my_person.last_name);
    my_person.set_last_name("Zahariev");
    println!("Person: {} ", my_person.full_name());
}