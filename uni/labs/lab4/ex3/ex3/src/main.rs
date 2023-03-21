fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > b {
        swap(a, b);
    }
    if b > c {
        swap(b, c);
    }
    if a > b {
        swap(a, b);
    }
}

fn main() {
    let mut a = 5;
    let mut b = 4;
    let mut c = 3;
    println!("Wartości przed posortowaniem: {} {} {}", a, b, c);

    sort(&mut a, &mut b, &mut c);

    println!("Wartości po posortowaniu: {} {} {}:", a, b, c);
}
