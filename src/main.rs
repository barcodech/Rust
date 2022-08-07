use std::fmt::Display;

fn func_1<T:Display> (input: T) {
    println!("This {} for book_a",input);
}

fn func_2 <T:Display> (input: T) {
    println!("This {} for book_b",input);
}

#[derive(Debug)]
struct Book <T:Display> {
    name: String,
    get_year: fn(T)
}

fn main() {
    let book_a = Book{
        name: "book A".to_string(),
        get_year: func_1
    };

    let book_b = Book{
        name: "book B".to_string(),
        get_year: func_2
    };

    (book_a.get_year)("Helo");
    (book_b.get_year)("world");

    println!("{:?} {:?}",book_a,book_b);

}
