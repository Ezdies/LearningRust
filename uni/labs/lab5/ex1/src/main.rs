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

fn odwrocony_kawalek(s: &str, beginning_index: usize, end_index: usize) -> String {
    let my_fragment = &s[beginning_index..=end_index];
    let reversed_fragment = my_fragment.chars().rev().collect();

    reversed_fragment
}

//fn szyfruj(napis: &str, klucz: u32) -> String {

//}

fn main() {
    let res1 = liczba_wystapien("hello world", 'o');
    println!("Liczba wystąpień: {res1}");

    let res2 = co_drugi_znak("hello world");
    println!("Co drugi znak: {res2}");

    let res3 = odwrocony_kawalek("siema", 0, 4);
    println!("Odwrócony kawałek siema to: {res3}");
}
