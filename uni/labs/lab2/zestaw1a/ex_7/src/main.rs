fn main() {
    let mut n : u32 = 1234;
    while n > 0{
        print!("{} ", n % 10);
        n /= 10;
    }
}
