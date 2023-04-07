i
fn hamming(a: &str, b: &str) -> Option<f32>{
    let fragment: f32 = 0.0;
    let sum: f32 = 0.0;

    let mut counter1 = 0;
    let mut counter2 = 0;

    for c1 in a.chars(){
        for c2 in b.chars(){
            if c[counter1] == c[counter2]{
            }
        }
        }
    
}


fn main() {
    let res = hamming("mama", "tata").unwrap_or(0);
    println!("{}", res);
}
