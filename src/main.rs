type Mystring = String;
 
fn main() {
    let x = String::from("this is string");
    let y = Mystring::from("this is string");
 
    println!("{}",x==y);
 }
 
type Myvec = Vec<Vec<Vec<[i32;3]>>>;
 
fn return_vec() -> Myvec {
    vec![vec![vec![[6,5,4]]]]
}
fn main() {
    println!("{:?}",return_vec())
 }
 
#[derive(Debug)]
 struct Mystring(String);
 fn main(){
    let x = Mystring(String::from("this is string"));
    println!("{:?}",x);
 }
