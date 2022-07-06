fn tuple() {
  let tup = (1,2,String::from("Adam"));
  println!("{}",tup.0);
  println!("{}",tup.1);
  println!("{}",tup.2);

  let mut tup = (1,2,String::from("Adam"));
  tup.2 = String::from("Jack");
  println!("{}",tup.0);
  println!("{}",tup.1);
  println!("{}",tup.2);

}

fn arrays(){
  let mut arr = [6,7,8];
  arr[0] = 10;
  arr[1] = 9;
  for x in &arr {
    println!("{}",x);
  }

  let exp_array =[1;3];
  for x in &exp_array {
    println!("{}",x);
  }

  let [adam, jack] = ["adam".to_string(), "jack".to_string()];
  print!("{}",adam);
  print!("{}", jack);
}

fn vectors() {
  let vector = vec![1,3,5,7];
  let mut vector: Vec<i32> = (0..10).collect();
  
  vector.push(10);
  for x in &vector {
    println!("{}",x);
  }

  println!("{}",vector[vector.len() -1]);
  print!("{:?}",vector.pop());
}

fn iteration(){

  let arr = [1;5];
  /* for x in arr.iter() {
    println!("{}",x);
  } */

  for x in arr.iter().enumerate() {
    println!("{}",x.1);
  }

  for item in arr.iter().enumerate() {
    let (i,x) = item;
    println!("Index: {}, Value: {}", i,x)
  }

  let vector: Vec<i32> = (0..5).collect();
  for x in vector.iter() {
    println!("{}",x);
  }

  let mut vector2: Vec<i32> = (0..5).collect();
  for x in vector2.iter_mut(){
    *x += 2;
  }
  println!("{:?}",vector2);
}

fn main(){
  iteration()
}