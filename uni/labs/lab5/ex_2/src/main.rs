fn co_drugi_znak(napis: &str) -> String {
    let mut res: String = String::new();

    for (index, znak) in napis.char_indices() {
        if index % 2 == 1 {
            res.push(znak);
        }
    }
    res
}

fn main() {
    let res = co_drugi_znak("hello world");
    println!("Co drugi znak: {res}");
}
