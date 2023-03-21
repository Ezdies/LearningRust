//Generator liczb pseudolosowych LCG
//z' = (a * z + c) % m
//obliczamy nowe ziarno, i zwracamy liczbę z tego przedziału
//a = 75 z = <a, b> m = 2^16 + 1 = 65537
//z' % (b - a + 1) + a <- return
//usize, bo taką wartość ma indeks tablicy

fn rand(seed: &mut usize, min_rand: usize, max_rand: usize) -> usize {
    let mut last_seed = *seed;
    let x = 75;
    let c = 74;
    let m = 65537;
    last_seed = (x * (*seed) + c) % m;
    return last_seed % (max_rand - min_rand + 1) + min_rand;
}

fn main() {
    for _i in 0..20 {
        let res = rand(&mut 132, 0, 100);
        println!("{res}");
    }
}
