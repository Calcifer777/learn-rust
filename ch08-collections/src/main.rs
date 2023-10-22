use std::{marker::Copy, collections::HashMap};


fn main() {

    // Vec<T>
    let mut v = vec![1, 2];
    let _ = &v[1];
    if let Some(i) = v.get(2) {
        println!("{}", i);
    }
    for i in &mut v {
        *i *= 10;
        println!("{}", i)
    }

    let vv1 = vec![1,2,3];
    let vv2 = vec![4,5,6];
    let vv_concat = concatenate_vectors(&vv1, &vv2);
    println!("{:?}", vv_concat);


    // String
    let s1  = "hello".to_string();
    let s2  = " world".to_string();
    let mut s3 = s1.chars().chain(s2.chars()).collect::<String>();
    
    let s3b = format!("{s1} {s2}");

    // HaspMap
    let v = vec![11,2,3,4,5,62,3,5,62,3,1,1,2];
    let v: Vec<i16> = Vec::new();
    match mode(&v) {
        None => println!("Mode for {:?} can't be computed!", v),
        Some(i) => println!("Mode for {:?} is {}", v, i),
    }
    match mean(&v) {
        None => println!("Mean for {:?} can't be computed!", v),
        Some(f) => println!("Mean for {:?} is {:.2}", v, f),
    }
    
}


fn concatenate_vectors<T: Copy>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    v1.iter().copied().chain(v2.iter().copied()).collect()
}

fn mean(v: &Vec<i16>) -> Option<f32> {
    v.iter().copied()
    .reduce(|acc , i| acc + i)
    .map(|i| i as f32 / v.len() as f32)
}

fn mode(v: &Vec<i16>) -> Option<i16> {
    let mut counter = HashMap::<i16, i16>::new();
    for i in v {
        let count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    let most_frequent = counter.iter()
        .fold((None, -1), |acc, (&k, &v)| 
            if v > acc.1 { (Some(k), v) } else { acc }
        );
    match most_frequent {
        (None, -1) => None,
        (i, _) => i,
    }
}