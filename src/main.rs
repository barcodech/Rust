
#[derive(Debug)]
struct Book {
    name: String,
    year: u32,
}

impl Book {
    fn print_name(&self) {
        println!("The book's name is {}",self.name)
    }

    fn change<F> (&mut self,f:F)
    where
    F: Fn(&mut Book),
    {
        f(self);
    }
}

fn main(){
    let mut my_book = Book{
        name: "Domino".to_string(),
        year: 2005,
    };
    my_book.print_name();

    my_book.change(|book| {
        book.name = "Lego".to_string();
        book.year = 1999;
    });
    println!("{:?}",my_book);
}