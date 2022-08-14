use serde_derive::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Serialize,Deserialize)]
struct Person {
  name:String,
  age:u8,
  is_male:bool
}

fn main(){
  let json_str = r#"
  {
    "name": "Adam",
    "age": 30,
    "is_male": true
  }
  "#;

  let res = serde_json::from_str(json_str);
  if res.is_ok(){
    let p:JsonValue = res.unwrap();
    println!("The name is: {}", p["name"].as_str().unwrap());
    println!("The age is: {}", p["age"]);
    println!("The is_male is: {}", p["is_male"]);

/*let res = serde_json::from_str(json_str);
  if res.is_ok(){
    let p:Person = res.unwrap();
    println!("The name is: {}", p.name);
    println!("The name is: {}", p.age);
    println!("The name is: {}", p.is_male);
*/
  } else {
      println!("Soory could not parse JOSN");
  }
}



use serde_derive::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
struct User {
  name: String,
  id: u32,
  active: bool
}
#[derive(Debug,Serialize, Deserialize)]
struct NewUserRequest {
  name: String,
  id: u32,
}

fn make_user (request: NewUserRequest) -> User {
  User { 
    name: request.name, 
    id: request.id, 
    active: true }
}

fn handle_request(json_str: &str) {
  match serde_json::from_str(json_str) {
      Ok(good_json) => {
        let new_user = make_user(good_json);
        println!("Made a new user \n {new_user:#?}");
      }

      Err(e) => {
        println!("Got an error from {json_str}\n {e}");
      }
  }
}


fn main(){
  let good_json = 
  r#"
  {
    "name": "Adam",
    "id": 2001
  }
  "#;

  let bad_json = 
  r#"
  {
    "namee": "Adam",
    "idd": 2001
  }
  "#;

  handle_request(good_json);
  handle_request(bad_json);

}