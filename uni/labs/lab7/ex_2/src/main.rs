use std::iter::Map;

fn unikalne(t: Vec<i32>) -> Vec<i32>{
    let mut v = t.iter().map(|x|(x, t.iter().filter(|y| *y == x).count())).map(|x| x.0).collect();
    return v;
}

fn main() {
    println!("Hello, world!");
}

