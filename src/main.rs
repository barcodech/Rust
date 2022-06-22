fn main() {
    let mut say = String::from("Dog");
    say.push_str(" and Cat");
  
    let say2 = say;
  
    println!("{}",say)
  
  }

///---------------------------borrowing
fn main() {
    let say = String::from("Hello");
    borrow_say(&say);
    println!("{}",say);
  
    let mut arr = vec![1,2,3];
    println!("{:?}",arr);
    borrow_arr(&mut arr);
    println!("{:?}",arr);
  
  }
  
  fn borrow_say(_say:&String){
    println!("{}",_say);
  }
  
  fn borrow_arr(_arr:&mut Vec<i32>){
    _arr.push(4);
  }

 
