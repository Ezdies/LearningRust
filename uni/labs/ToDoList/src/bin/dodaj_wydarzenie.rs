use std::io;

fn get_user_input_description()-> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    format!("{}", input.trim())
}

fn main() {
    println!("Podaj opis\n");
    println!("Podana opis to: {:?}", get_user_input_description());
}
