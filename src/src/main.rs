
fn main() {
    let age = 10;
    let check_id = true;
    let check_member = true;

    if age >= 21 && check_id || check_member{
        println!("You can drink beer");
    } else if age < 21 && check_id || check_member{
        println!("You can not drink beer");        
    } else {
        println!("You should drink milk");
    }

    let age2 = if age >=21 {true} else {false};
    println!("Check:{}",age2)
  }