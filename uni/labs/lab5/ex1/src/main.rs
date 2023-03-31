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
        if index % 2 == 1 {
            res.push(znak);
        }
    }
    res
}


fn szyfruj(s: &str, klucz: usize) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < s.len() {
        let chunk = s
            .chars()
            .skip(i)
            .take(klucz)
            .collect::<String>();
        let reversed_chunk = odwroc(&chunk);
        result.push_str(&reversed_chunk);
        i += klucz;
    }
    result
}

fn odwrocony_kawalek(s: &str, beginning_index: usize, end_index: usize) -> String {
    let my_fragment = &s[beginning_index..=end_index];
    let reversed_fragment = my_fragment.chars().rev().collect();

    reversed_fragment
}

fn odwroc(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    let res1 = liczba_wystapien("hello world", 'o');
    println!("Liczba wystąpień: {res1}");

    let res2 = co_drugi_znak("hello world");
    println!("Co drugi znak: {res2}");

    let res3 = odwrocony_kawalek("siema", 0, 4);
    println!("Odwrócony kawałek siema to: {res3}");

    let res4 = szyfruj("kaszanka", 3);
    println!("Zaszyfrowany wyraz to: {:?}", res4);
}
