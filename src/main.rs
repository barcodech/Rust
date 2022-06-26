/* struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Color2(u8,u8,u8); */

struct  Person {
    first_name: String,
    last_name: String,
}

impl  Person {
    fn new(first:&str,last:&str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }

    fn set_last_name(& mut self,last:&str){
        self.last_name = last.to_string()
;    }
    
    fn to_tuple(self) -> (String,String){
        (self.first_name,self.last_name)
    }
}


fn main() {


    let mut p = Person::new("Adam","Roger");
    println!("Person {} {}", p.first_name,p.last_name);
    println!("Person {}",p.full_name());
    p.set_last_name("Jacob");
    println!("Person {}",p.full_name());
    println!("Person {:?}", p.to_tuple());

    /* let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 0;
    c.blue = 255;

    println!("Color: {} {} {}",c.red,c.green,c.blue);

    let mut b = Color2 (255,0,0);
    b.0 = 0;
    b.2 = 255;
    println!("Color2: {} {} {}",b.0,b.1,b.2); */
}


#[derive(Debug)]
struct Country{
    population:u32,
    city: String,
    foods: String,
}
fn main(){
    let population = 80_000;
    let city = String::from("Melaka");
    let foods = String::from("Cendol");

    let Malaysia = Country{
        population,
        city,
        foods,
    };
    println!("{:?}",Malaysia);
}