mod args;

use std::{env, error::Error, fs};

pub fn run(config: SimpleGrepConfig) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.filepath)?;
    let results = search_cased(&config.query, &text, config.cased);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

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

fn search_cased<'a>(query: &str, contents: &'a str, cased: bool) -> Vec<&'a str> {
    let lquery = &query.to_lowercase();
    let pat = if cased {query} else {lquery};
    contents
        .split("\n")
        .filter_map(|line| {
            let lline = &line.to_lowercase();
            let text = if cased {line} else {lline};
            text.find(pat).map(|_| line)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_one_result() {
        let query = "duct";
        let contents = concat!("Rust:\n", "safe, fast, productive.\n", "Pick three.\n",);
        let exp = vec!["safe, fast, productive."];
        assert_eq!(search_cased(query, contents, false), exp);
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = concat!(
            "Rust:",
            "safe, fast, productive.",
            "Pick three.",
            "Duct tape.",
        );

        assert_eq!(
            vec!["safe, fast, productive."],
            search_cased(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = concat!(
            "Rust:",
            "safe, fast, productive.",
            "Pick three.",
            "Trust me.",
        );

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_cased(query, contents, false)
        );
    }
}
