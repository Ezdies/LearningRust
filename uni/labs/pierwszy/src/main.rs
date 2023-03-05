fn szlaczek() {
    println!("========================================");
}

fn main() {
    let a = 33;
    let mut b = 0;
    if b != 0 {
        let x = a / b;
        println!("{} / {} = {}", a, b, x);
    } else {
        println!("Nie dziel przez zero");
    }

    b = 10;
    println!("{}", b);


    szlaczek();
    szlaczek();
}
