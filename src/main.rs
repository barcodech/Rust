fn main() {
    let vector1 = vec![1, 2, 3];  
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b = vector1.into_iter().map(|x| x * 2).collect::<Vec<i32>>();
   
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x +=100);
   
    println!("{:?}", vector1_a);
    println!("{:?}", vector2);
    println!("{:?}", vector1_b);
  }
   
   
  fn main() {
    let my_vec = vec!['a', 'b', 'c', 'd'];
   
    let mut my_vec_iter = my_vec.iter();
   
    assert_eq!(my_vec_iter.next(), Some(&'a'));  
    assert_eq!(my_vec_iter.next(), Some(&'b'));  
    assert_eq!(my_vec_iter.next(), Some(&'c'));
    assert_eq!(my_vec_iter.next(), Some(&'d'));
    assert_eq!(my_vec_iter.next(), None);        
           
  }
   
   
  #[derive(Debug, Clone)]
  struct Library {
      library_type: LibraryType,
      books: Vec<String>,
  }
   
  #[derive(Debug, Clone)]
  enum LibraryType {
      City,
      Country,
  }
   
  impl Library {
      fn add_book(&mut self, book: &str) {
          self.books.push(book.to_string());
      }
   
      fn new() -> Self {
          Self {
              library_type: LibraryType::City,
              books: Vec::new(),
          }
      }
  }
   
  impl Iterator for Library {
      type Item = String;
   
      fn next(&mut self) -> Option<String> {
          match self.books.pop() {
              Some(book) => Some(book + " is found!"),
              None => None,
          }
      }
  }
   
  fn main() {
      let mut my_library = Library::new();
      my_library.add_book("dark night");
      my_library.add_book("roller car");
      my_library.add_book("gardening");
      my_library.add_book("best foods");
   
      for item in my_library.clone() {
          println!("{}", item);
      }
  }
   
   
   
  