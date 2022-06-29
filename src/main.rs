fn get_value(vec: Vec<i32>) -> i32 {
    vec[3]
}
 
fn main() {
    let new_vec = vec![1, 2];
    let index = get_value(new_vec);
    println!("{}",index);
}
   
fn get_value(vec: Vec<i32>) -> Option<i32> {
    if vec.len() < 5 {
        None
    } else {
        Some(value[3])
    }
}
 
fn main() {
    let vec1 = vec![1, 2];
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}",
        get_value(vec1).unwrap(), //.expect("abc")
        get_value(vec1).unwrap()
    );
}
 
fn get_value(vec: Vec<i32>) -> Option<i32> {
    if vec.len() < 5 {
        None
    } else {
        Some(vec[3])
    }
}
 
fn get_option(e_vec: Vec<Option<i32>>) {
  for item in e_vec {
    match item {
      Some(number) => println!("Found {}", number),
      None => println!("Found None"),
    }
  }
}
 
fn main() {
    let vec1 = vec![1, 2];
    let vec2 = vec![1, 2, 3, 4, 5];
    let mut enum_vec = Vec::new();
 
    enum_vec.push(get_value(vec1));
    enum_vec.push(get_value(vec2));
 
    get_option(enum_vec);
                               
}
 
fn get_value(vec: Vec<i32>) -> Option<i32> {
    if vec.len() < 5 {
        None
    } else {
        Some(vec[3])
    }
}
 
fn main() {
    let vec1 = vec![1, 2];
    let vec2 = vec![1, 2, 3, 4, 5];
    let vecs = vec![vec1, vec2];
    for vec in vecs {
        let index = get_value(vec);
        if index.is_some() {
            println!("Found {}", index.unwrap());
        } else {
            println!("Found None");
        }
    }
}
 
 fn get_foods(name:&str) -> Option<&str> {
  match name {
      "friuts" => Some("apple"),
      "meat" => Some("beef"),
      _ => None
  }
}

fn main() { 
  println!("{}",match get_foods("banana") {
      Some(x) => x,
      None => "No foods found" 
  });

  let name = String::from("Bangkok");
  println!("Charactor at this index: {}",match name.chars().nth(10) {
    Some(c) => c.to_string(),
    None => "No charactor".to_string()
      
  });
}
