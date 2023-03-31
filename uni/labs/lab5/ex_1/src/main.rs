fn liczba_wystapien(napis: &str, znak: char) -> u32 {
    let mut counter: u32 = 0;

    for x in napis.chars() {
        if x == znak {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let res = liczba_wystapien("hello world", 'o');
    println!("Liczba wystąpień: {res}");
}
