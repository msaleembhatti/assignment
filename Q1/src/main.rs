mod movies{
    pub mod english{
        pub enum Name {GOT,PRISON_BREAK,GOD_FATHER,}
        fn search_movie(movie: Name){ //private function
           match movie {
               Name::GOT=> println!("Game of Throne"),
               Name::PRISON_BREAK=> println!("Its about breaking the prison"),
               Name::GOD_FATHER=>println!("Ever living"),
            }   
        }
        pub fn watch_movie(movie:Name){//public function
            search_movie(movie);
        }
    }
}
fn main() {
        movies::english::watch_movie(movies::english::Name::GOD_FATHER);
}
