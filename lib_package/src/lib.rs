#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod outer {
    pub mod inner {
        pub fn simplefunction(){
            println!("Simple function!");
        }
    }
}
