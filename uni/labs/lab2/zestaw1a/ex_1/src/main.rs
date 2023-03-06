fn main() {
    let year: u32 = 2000;

    if (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0) {
        println!("Rok: {year} jest przestępny");
    } else {
        println!("Rok: {year} nie jest przestępny");
    }
}
