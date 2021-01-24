// Enums ar types with a select few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(moving: Movement) {
    // Perform action based on Movement
    //similar to switch
    match moving {
        Movement::Up => println!("Going Up there"),
        Movement::Down => println!("Going Down there"),
        Movement::Left => println!("Going left there"),
        Movement::Right => println!("Going Right there")
    }
}

pub fn run() {
    let avater_one = Movement::Left;
    let avater_two = Movement::Up;
    let avater_three = Movement::Right;
    let avater_four = Movement::Down;

    move_avatar(avater_one);
    move_avatar(avater_two);
    move_avatar(avater_three);
    move_avatar(avater_four);

}