use std::cell::Cell;
 
struct PhoneModel {
    model_name: String,
    date_issued: u32,
    on_sale: Cell<bool>,
}
 
fn main() {
    let iphone = PhoneModel {
        model_name: "iphone".to_string(),
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
 
    iphone.on_sale.set(false);
    println!("{}",iphone.on_sale.get());
 
}
 
use std::cell::Cell;
 
#[derive(Debug)]
struct Book {
    name: String,
    number: Cell<u32>
}
 
impl Book {
    fn return_number(&self) -> u32 {
        let new_number = self.number.get() + 10;
        self.number.set(new_number);
        self.number.get()
    }
}
 
fn main(){
    let my_book = Book{
        name: "gardening".to_string(),
        number: Cell::new(10),
    };
 
    dbg!(&my_book);
    let next_number = my_book.return_number();
    print!("{}",next_number);
    dbg!(my_book);
}
 
 
