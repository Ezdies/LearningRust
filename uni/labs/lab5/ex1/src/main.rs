fn liczba_wystapien(napis: &str, znak: char) -> u32 {
    let mut counter: u32 = 0;

    for x in napis.chars() {
        if x == znak {
            counter += 1;
        }
    }
    counter
}

fn co_drugi_znak(napis: &str) -> String {
    let mut res: String = String::new();

    for (index, znak) in napis.char_indices() {
        if index as usize % 2 == 1 {
            res.push(znak);
        }
    }
    res
}

fn odwrocony_kawalek(s: &str, beg: usize, en: usize){
    match str {
        be => {},
    }

}

fn szyfruj(napis: &str, klucz: u32) -> String {

}

fn main() {
    let res1 = liczba_wystapien("hello world", 'o');
    println!("Liczba wystąpień: {res1}");

    let res2 = co_drugi_znak("hello world");
    println!("Co drugi znak: {res2}");
}
