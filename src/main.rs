  enum Movement {
    Up,
    Down,
    Left,
    Right,
  }

  fn walking(m: Movement){
    match m {
        Movement::Up => println!("moving up"),
        Movement::Down => println!("moving down"),
        Movement::Left => println!("moving left"),
        Movement::Right => println!("moving right"),
    }
  }

  fn main() {
        let go_left = Movement::Left;
        let go_up = Movement::Up;
        let go_right = Movement::Right;
        let go_down = Movement::Down;

        walking(go_left);
        walking(go_up);
        walking(go_right);
        walking(go_down);

  }

  enum Meals {
    Lunch,
    Dinner,
    Nomeal
}
 
fn check_meal(time: i32) -> Meals {
    match time {
        11..=13 => Meals::Lunch,
        17..=19 => Meals::Dinner,
        _ => Meals::Nomeal,
    }
}
 
fn check_state(state: &Meals) {
    match state {
        Meals::Lunch => println!("I am eating Lunch!"),
        Meals::Dinner => println!("I am eating Dinner!"),
        Meals::Nomeal => println!("Not time to eat"),
    }
}
 
fn main() {
    let time = 22;
    let mymeal = check_meal(time);
    check_state(&mymeal);
}
 
enum Meals {
    Lunch(String),
    Dinner(String),
}
fn check_meal(time: i32) -> Meals {
   /*  let string1 = String::from("I am eating Lunch!");
    let string1 = String::from("I am eating Dinner!");
    let string1 = String::from("Not time to eat"); */
    match time {
        11..=13 => Meals::Lunch(String::from("I am eating Lunch!")),
        17..=19 => Meals::Dinner(String::from("I am eating Dinner!")),
        _ => Meals::Dinner(String::from("Not time to eat")),
    }
}
fn check_state(state: &Meals) {
    match state {
        Meals::Lunch(d) => println!("{}", d),
        Meals::Dinner(d) => println!("{}", d),
    }
}
fn main() {
    let time = 22;
    let mymeal = check_meal(time);
    check_state(&mymeal);
}
enum Chilis {
    Two,
    Four,
    Five,
    Ten,
}
fn check_level(chili: &Chilis) -> i32 {
    use Chilis::*;
 
    let spicy_level = match chili {
        Chilis::Two => 2,
        Chilis::Four => 4,
        Chilis::Five => 5,
        Chilis::Ten => 10,
    };
    spicy_level
}
 
fn main() {
    let my_chilis = Chilis::Five;
    let spicy_level = check_level(&my_chilis);
    println!("my spicy level is {}", spicy_level);
}
 enum Chilis {
    Two = 2,
    Four =4,
    Five =5,
    Ten =10,
    Eleven,
}
 
fn main() {
    use Chilis::*;
    let chilivec = vec![Two, Four, Five, Ten];
    for chili in chilivec {
        match chili as u32 {
            level if level <= 4 => println!("normal spicy"),
            level if level >= 5 => println!("very spicy"),
            _ => println!("Unknown"),
        }
    }
    println!("level of Eleven is {}", Eleven as u32);
}
 
enum Number {
    U32(u32),
    I32(i32),
}
 
fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}
 
 
fn main() {
    let my_vec = vec![get_number(-10), get_number(10), get_number(20)];
 
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's u32 with the value {}", number),
            Number::I32(number) => println!("It's i32 with the value {}", number),
        }
    }
}
 
 
 
