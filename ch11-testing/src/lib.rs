

pub fn adder(i: i32) -> i32 {
    i + 2
}

fn private_fn(s: &str) -> String {
    String::from(format!("hello, {s}"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_private_fn() -> Result<(), String> {
        let exp = "hello, marco";
        let out = private_fn("marco");
        if out == exp {
            Ok(())
        } else {
            Err(format!("Expected {} but got {}", exp, out))
        }
    }
}