use std::collections::HashMap;

use num_bigint::BigInt;


fn main() {
    let n: usize = 100;
    let fibo = fib(n);
    println!("Fibonacci {n} is {fibo}");
}


fn fibonacci(n: usize) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    fn fibo(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
        match cache.get(&n) {
            Some(i) => {
                *i
            },
            None => {
                if n == 1 || n == 2 {
                    1
                } else {
                    let fibo = fibo(n-1, cache) + fibo(n-2, cache);
                    cache.insert(n, fibo);
                    fibo
                }
            }
        }
    }
    fibo(n, &mut cache)
}


fn fib(n: usize) -> BigInt {
    if n <= 2 {
        BigInt::from(1)
    } else {
        let mut lag = BigInt::from(1);
        let mut llag = BigInt::from(1);
        let mut counter = n - 2;
        while counter > 0 {
            let tmp = lag.clone();
            lag = lag + llag;
            llag = tmp;
            counter -= 1;
        }
        lag
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {

 fn x(&self) -> &T {
   &self.x
 }

}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}