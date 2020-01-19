pub mod BookStore {
    #[derive(Debug)]
    pub struct Book {
     title: String,
     publisher: String,
    }
    impl Book {
        pub fn new(title:String, publisher:String)->Book{
            Book{
                title,
                publisher,
            }
        }
    }
    pub mod english {
        use crate::BookStore::Book;
        pub fn print_book(book_title:Book){
        println!("Title:\t  {}\nPublisher:{}\nBookStore::English",book_title.title,book_title.publisher);
        }
    }
}