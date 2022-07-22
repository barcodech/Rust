/* struct Rat {
    name: String
  }
  
impl Rat {
      fn eating_food(&self) {
          println!("{} is eating rat foods",self.name)
      }
    }
 */



 fn closure_to_i32 (input: &str) -> impl FnMut(i32) -> i32 {
    match  input {
        "plus 5" => |mut number| {
            number += 5;
            println!("plus 5.Now it is {}",number);
            number
        },
        "plus 10" => |mut number| {
            number += 10;
            println!("plus 10.Now it is {}",number);
            number
        },
        _ => |mut number| {        
            println!("Do nothing with {}",number);
            number
        },
    }
}
 
fn main(){
    let mut closure1 = closure_to_i32("plus 5");
    let mut closure2 = closure_to_i32("plus 10");
    let mut closure3 = closure_to_i32("zxcasd");

    closure1(5);
    closure2(10);
    closure3(15);
}