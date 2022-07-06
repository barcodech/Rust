fn main() {
    let number1 = 4;
    let number2 = 5;
 
    let my_closure = |x: i32| println!("{}", number1 + number2 + x);
    my_closure(5);
}

fn closures1(num1:i32,num2:i32) -> i32 {
  let sum = |x| {
    x + 2
  };

  if num1>num2 {
    sum(num1)
  } else {
    sum(num2)
  }
}

fn closures2(){
  let printout = |x| {
    x
  };
  
  println!("{}",printout(5));
}
fn main(){
  let x = closures1(1, 5);
  println!("{}",x);

  closures2();
}