fn main(){
 
    let x:&'a str = "Hello";
}
 
fn print_str(my_str: &String){
    println!("{}",my_str);
}
 
fn main(){
    let my_string = String::from("I am a string");
    print_str(&my_string);
}
 
fn return_str<'a>(input1: &'a str,input2: &'a str) -> &'a str{
    input2
   
}
 
#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    point: u32,
}
 
 
fn main() {}
 
 
#[derive(Debug)]
struct City<'a,'b> {
    name: &'a str,
    name2: &'b str,
    points: u32,
}
 
impl City<'_,'_> {
    fn take_points(&mut self) {
        self.points += 20;
        println!("{} has {} points", self.name, self.points);
    }
}
 
 
fn main() {}
 
 
 
 
