fn main() {
    let mut sum : u32 = 0;
    let mut n : u32 = 1234;

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }

    println!("{sum}")

}
