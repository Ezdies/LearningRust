fn heaviside(x: f32) -> f32 {
    if x < 0.0{
        0.0
    }
    else if x > 0.0{
        1.0
    }
    else{
        0.5
    }
}

fn main() {
    let res = heaviside(1.2);
    println!("H(x) = {}", res);
}
