use Karim::BookStore::Book;
use Karim::BookStore::english;
fn main() {
    let java = Book::new("java".to_string(),"Dietel&Dietel".to_string() );    
    
    english::print_book(java);
    
    
    println!("changed FIle");
}
