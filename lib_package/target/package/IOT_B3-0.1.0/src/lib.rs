#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
///outer module: this is for testing
/// 
pub mod outer { 
    ///inner module: this is inside outer
    pub mod inner {
        ///a simple function
        /// # Example 
        /// ```
        /// outer::inner::simplefunction()
        /// ```
        pub fn simplefunction(){ 
            println!("Simple function!");
        }
    }
}
