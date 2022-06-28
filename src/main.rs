#[derive(Debug)]
struct Mango {
    price: u8,
    color: Pickercolor,
}

#[derive(Debug)]
enum Pickercolor {
    Green,
    Yellow,
}

impl Mango {
    fn new() -> Self {
        Self { 
            price: 50,
            color: Pickercolor::Green,
         }
    }
    fn change_color(&mut self){
        println!("Changing color to yellow");
        self.color = Pickercolor::Yellow;
    }
    fn check_color(&self) {
        match self.color {
            Pickercolor::Green => println!("The mango is green"),
            Pickercolor::Yellow => println!("The mango is yellow"),
        }
    }
}

fn main(){
    let mut new_mango = Mango::new();
    new_mango.check_color();
    new_mango.change_color();
    new_mango.check_color();
    println!("{:?}",new_mango);
}