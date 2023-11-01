use std::thread;

fn closure_check1() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    println!("Before calling closure: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}


fn closure_check2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(|| println!("From thread: {:?}", list))
        .join()
        .unwrap();
}