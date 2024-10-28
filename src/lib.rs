mod error;
pub use self::error::{Result, Error};

pub mod http {
    #[derive(Debug)]
    pub struct HTTP {
        pub request: String,
        pub headers: Vec<String>,
    } 

    impl HTTP {
        pub fn new() -> HTTP {
            HTTP {
                request: String::from(""),
                headers: Vec::<String>::new(),
            }
        }
    }
}

pub mod proxy;
