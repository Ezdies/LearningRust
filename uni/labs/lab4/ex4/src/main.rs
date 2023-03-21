fn main() {
    let a: u8 = 33;
    let b: u8 = 126;

    for i in a..=b {
        print!("{}", i as char);
    }
}
