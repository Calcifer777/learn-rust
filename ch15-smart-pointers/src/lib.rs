use std::rc::Rc;

mod exps;
mod mybox;

enum List<'a> {
    Cons(i32, &'a List<'a>),
    Nil,
}

impl<'a> List<'a> {
    fn len(&self) -> i32 {
        match *self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
    fn sum(&self) -> i32 {
        match *self {
            List::Cons(i, tail) => i + tail.sum(),
            List::Nil => 0,
        }
    }
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_length() {
        let l = List::Cons(1, &List::Cons(2, &List::Cons(3, &List::Nil)));
        let out = l.len();
        let exp = 3;
        assert_eq!(out, exp);
    }

    #[test] fn test_sum() {
        let l = List::Cons(1, &List::Cons(2, &List::Cons(3, &List::Nil)));
        let out = l.sum();
        let exp = 6;
        assert_eq!(out, exp);
        let x = [Box::new((1, 2)), Box::new((3, 4))];
    }

    #[test]
    fn multiple_owners() {
        use RcList::{Cons, Nil};
        let rc_common = Rc::new(Cons(
            5, Rc::new(Cons(10, Rc::new(Nil)))
        ));
        assert_eq!(Rc::strong_count(&rc_common), 1);
        let rc_copy_a = Cons(3, Rc::clone(&rc_common));
        assert_eq!(Rc::strong_count(&rc_common), 2);
        let rc_copy_b = Cons(1, Rc::clone(&rc_common));
        assert_eq!(Rc::strong_count(&rc_common), 3);
        
    }
}
