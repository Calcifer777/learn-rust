use std::env;


pub struct SimpleGrepConfig {
    pub filepath: String,
    pub query: String,
    pub cased: bool,
}

impl SimpleGrepConfig {
    pub fn build(args: &[String]) -> Result<SimpleGrepConfig, String> {
        if args.len() != 3 {
            Err(format!("Expected 2 arguments, found {}", args.len()))
        } else {
            Ok(SimpleGrepConfig {
                filepath: args[1].clone(),
                query: args[2].clone(),
                cased: env::var("CASED").is_ok(),
            })
        }
    }
}
