fn main() {
    
    let max_a = 20;
    let max_b = 30;
    let max_c = 40;

    for i in 1..=max_a {
        for j in 1..=max_b{
            for k in 1..=max_c{
                if i * i + j * j == k * k{
                println!("a:{i} b:{j} c:{k}");
                }
            }
        }
    }
}
