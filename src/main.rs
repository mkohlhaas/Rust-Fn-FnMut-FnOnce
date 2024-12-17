#![allow(unused)]

fn main() {
    let mut v = vec![1, 2, 3];
    let c1 = || {
        print!("Fn ");
        println!("closure");
        v[0]
    };
    c1();
    dbg!(&v);
    let mut c2 = || {
        print!("FnMut ");
        println!("closure");
        v[0] = 0;
        v[0]
    };
    c2();
    dbg!(&v);
    let c3 = || {
        print!("FnOnce ");
        println!("closure");
        // play with into_iter() and iter()
        v.into_iter().take(2).collect::<Vec<i32>>()[0]
    };
    println!("{:?}", c3());
}
