fn main() {
    let true_or_false = true;
 
    match true_or_false {
        true => println!("It's true"),
        false => println!("It's false"),
        true => println!("It's true"),
    }
}
enum Foods {
    KFC,
    Pizza,
    Barbecue,
}
 
fn choose_food(foods: &Foods) {
    use Foods::*;
    match foods {
        KFC => println!("You eat KFC"),
        Pizza => println!("You eat pizza"),
        Barbecue => unreachable!(),
    }
}
 
fn main() {
    let input = Foods::Barbecue;
    choose_food(&input);
}
 
fn main() {
    println!("we are at column {} and line {} in the file {} and module {}",column!(),line!(),file!(),module_path!());
   
}
fn main() {
    let message = if cfg!(target_os = "linux") { "Yes" } else { "No" };
    println!("run on windows?",message);
}
 
 
