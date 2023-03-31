fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let mut result = imie[..1].to_uppercase();
    result.push('.');
    let tmp_nazwisko = nazwisko[..1].to_uppercase() + &nazwisko[1..].to_lowercase();

    format!("{} {}", result, tmp_nazwisko)
}

fn main() {
    let res = wizytowka("jan", "KOWALSKI");
    println!("{res}");
}
