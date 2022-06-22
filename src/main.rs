---------------------------String Methods
fn main() {
  {
    let my_string = String::from("Today is Monday");
    println!("after replace: {}",my_string.replace("Monday", "Sunday"));
  }

  {
    let my_string = String::from("coconut\nbanana\napple");
    println!("item:{}",my_string);
      for line in my_string.lines(){
        println!("item:{}",line);
      }
  }

  {
  let my_string = String::from("KFC+donut+noodles");
  let tokens:Vec<&str> = my_string.split("+").collect();
  println!("index 1:{}",tokens[1]);
  }

  {
    let my_string = String::from("   I like pizza\n");
    println!("Before trim: {}",my_string);
    println!("After trim: {}",my_string.trim());
  }

  {
    let my_string = String::from("Bangkok");
    println!("{}",my_string);
      match my_string.chars().nth(4) {
        Some(c) => println!("Charactor at index 4: {}",c),
        None => println!("No charactor at index 4")
      }
  }
}