use LastNotKolos::Towar::Towar;

fn main(){
    let towar = Towar::new("chleb".to_string(), 2.99);
    println!("Cena {}, to {}", towar.zwroc_nazwe(), towar.zwroc_cene());
}
