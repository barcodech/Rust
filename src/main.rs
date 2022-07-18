struct Book {}
 
enum BookType {
    HardCover,
    SoftCover,
}
 
fn get_book(book: &Book) -> Option<String> {
    todo!()
}
 
fn delete_book(book: Book) -> Result<(), String> {
    todo!()
}
 
fn check_book_type(book_type: &BookType) {
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}
 
fn main() {
    let book_type = BookType::HardCover;
    check_book_type(&book_type);
}
