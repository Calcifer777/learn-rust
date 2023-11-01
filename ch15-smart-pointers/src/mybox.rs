use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0  // accesses the first value in a tuple struct
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_mybox() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(*y, 5);
    }
}