
fn main() {
   let number = 20;

   match number {
    1 => println!("one"),
    2 => println!("two"),
    3 | 4 => println!("three or four"),
    5..=10 => println!("five to ten"),
    _ => println!("It's doesn't match")
   }

   let name = "Hello";

   match name {
    "adam" => println!("This is Adam"),
    "jack" => println!("This is Jack"),

    _ => println!("It's doesn't match")
   }
}

fn main() {
    let my_number = 5;
    let my_number2 = match my_number {
        0 => 0,
        5 => 10,
        _ => 20,
    };
}
fn main() {
    let my_number = 10;
    let some_variable = match my_number {
        10 => 8,
        _ => "Not ten",
    };
}
fn main() {
    let my_number = 10;
 
    if my_number == 10 {
        let some_variable = 8;
    } else {
        let some_variable = "Something else";
    }
}
 
fn main() {
    let days = "Monday";
    let time = "Morning";
 
    match (days, time) {
        ("Tuesday", "Evening") => println!("dinner"),
        ("Monday", "Morning") => println!("going to work"),
        ("Sunday", "Afternoon") => println!("go to the beach"),
        _ => println!("sleep"),
    }
}
fn main() {
    let day = "Sunday";
    let rain = true;
 
    match (day, rain) {
        (day, rain) if rain == false => println!("go to work on {}", day),
        (day, rain) if day == "Monday" && rain == true => println!("stay home"),
        _ => println!("Today is {}. rain? {}.", day, rain),
    }
}
fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 20 => println!("Not much red"),
        (_, b, _) if b < 20 => println!("Not much blue"),
        (_, _, g) if g < 20 => println!("Not much green"),
        _ => println!("Unknown color"),
    }
}
 
fn main() {
    let first = (255, 0, 0);
    let second = (150, 150, 150);
    let third = (0, 0, 255);
 
    match_colours(first);
    match_colours(second);
    match_colours(third);
 
}
 
fn match_number(input: i32) {
    match input {
    number @ 8 => println!("{} is an unlucky number!", number),
    number @ 13 => println!("{} is unlucky number!", number),
    _ => println!("a normal number"),
    }
}
 
fn main() {
    match_number(8);
    match_number(13);
    match_number(20);
}
 
 
 
 
 
 
