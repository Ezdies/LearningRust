fn silnia(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    return n * silnia(n - 1);
}

fn main() {
    let res = silnia(5);
    println!("Silnia rek: {res}");
    let mut a: u32 = 1;
    let n: u32 = 5;

    for i in a..=n {
        a *= i;
    }

    println!("Silnia iter: {a}");

}
