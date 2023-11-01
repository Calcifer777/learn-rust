
trait Expr {
    fn exec(&self, i: i32) -> i32;
}

struct Sum {
    a: i32,
}

impl Expr for Sum {
    fn exec(&self, i: i32) -> i32 {
        self.a + i
    }
}

struct Mul {
    a: i32,
}

impl Expr for Mul {
    fn exec(&self, i: i32) -> i32 {
        self.a * i
    }
}

struct Combo {
    exprs: Vec<Box<dyn Expr>>,
}

impl Expr for Combo {
    fn exec(&self, i: i32) -> i32 {
        self.exprs.iter().fold(i, |acc, x| x.exec(acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combo() {
        let combo = Combo {
            exprs: vec![
                Box::new(Sum { a: 1 }), 
                Box::new(Mul { a: 3 }),
            ],
        };
        assert_eq!(combo.exec(3), 12);
    }
}
