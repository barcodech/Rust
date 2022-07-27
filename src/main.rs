struct Item {
    number: u8,
}
fn main() {
    let item = Item {
        number: 8,
    };
    let reference_number = &item.number;
    println!("{}", reference_number == 8);
}
 
struct Item {
    number: u8,
}
impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);          
    }
}
fn main() {
    let item = Item {
        number: 8,
    };
    let reference_1 = &item;
    let reference_2 = &reference_1;
    item.compare_number(8);
    reference_1.compare_number(8);
    reference_2.compare_number(8);
}
 
 
struct Thing {
    number:u32,
}
impl Thing {
    fn print_number(self){
        println!("{}",self.number);
    }
}
fn main(){
    let my_box = Box::new(Thing{number:50});
    my_box.print_number();
}
