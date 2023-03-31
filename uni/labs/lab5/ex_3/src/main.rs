fn odwroc(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
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

fn main() {
    let res = szyfruj("kaszanka", 3);
    println!("Zaszyfrowany wyraz to: {:?}", res);
}
