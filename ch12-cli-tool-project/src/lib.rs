pub mod args;

use std::{error::Error, fs};

use args::SimpleGrepConfig;


pub fn run(config: SimpleGrepConfig) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.filepath)?;
    let results = search(&config.query, &text, config.cased);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, cased: bool) -> Vec<&'a str> {
    // https://users.rust-lang.org/t/search-case-insensitive-from-ch-12-in-the-book-using-existing-search/35433/4
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
        let contents = concat!(
            "Rust:\n", 
            "safe, fast, productive.\n", 
            "Pick three.\n",
        );
        let exp = vec!["safe, fast, productive."];
        assert_eq!(search(query, contents, false), exp);
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = concat!(
            "Rust:\n",
            "safe, fast, productive.\n",
            "Pick three.\n",
            "Duct tape.\n",
        );

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = concat!(
            "Rust:\n",
            "safe, fast, productive.\n",
            "Pick three.\n",
            "Trust me.\n",
        );

        assert_eq!(
            vec!["Rust:", "Trust me.",],
            search(query, contents, false)
        );
    }
}
