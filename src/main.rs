/* struct Book{
    name: "Hello",
    year: 2005
}
 */
macro_rules! create_struct {
    ($name: tt,$field: ident,$type_of: ty) => {
        #[derive(Debug)]
        struct $name {
            $field: $type_of
        }
    };
    ($name: tt,$($field: ident,$type_of: ty), *) => {
        #[derive(Debug)]
        struct $name {
            $($field: $type_of),*
        }
    };
}
    create_struct!(Book,name,String);
    create_struct!(Superbook,name,String,year,u32,writer,String);

fn main(){
    let my_book = Book {
        name: "My book".to_string()
    };
    println!("{:?}",my_book);

    let my_seceond_book = Superbook {
        name: "My book".to_string(),
        year: 2005,
        writer: "adam".to_string()

    };
    println!("{:?}",my_seceond_book);
}