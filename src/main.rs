fn main(){
  
  i8 i16 i32 i64 i128 isize
  u8 u16 u32 u64 u128 usize

  i8 = 8 bits = 1 byte
  isize,usize 64 bits if possible
}

fn main() {
  let first_letter = 'A';
  let space = ' '; 
  let other_language_char = 'ก'; 
  let cat_face = '😺'; 
}

fn main() { 
    let my_number:u8 = 100; 

    println!("{}", my_number //as u8 as char); 
}


fn main() {
  println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
  println!("{}", "a".len()); 
  println!("{}", "ฮ".len());
  println!("{}", "国".len());
  println!("{}", "𓅱".len());
}


fn main() {
  let slice = "Hello!";
  println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
  let slice2 = "สวัสดี";
  println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}

---------------------------scalar data types
fn main() {

    let x = 10;
      u8 0 - (2^8-1) >>>>0-255
      i8 (-2^7) - (2^7-1) >>> -128-127

    let a = 5;
    let b: u8 = 1;

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = a+b;
    let minus =x -1.0;
    let multiply = a*4;
    let divided = 9/y;

    let boolean = false;

    let letter = 'a';
    
}
---------------------------compound data types
fn main() {
    
    let mut arr = [1,2,3,4,5];
    arr = [1,2,3,4,6];
    println!("{}",arr[4]);

    let mut tup = (1,true,'a');
    tup = (10,false,'z');
    print!("{}",tup.0);

}
