#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Cards {
    Trefl = 1,
    Karo = 2,
    Kier = 3,
    Pik = 4,
}

fn main() {
    let card = Cards::Pik;
    println!("{:?}", card);
    
}
