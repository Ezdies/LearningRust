fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a > b {
        swap(a, b);
    }
    if b > c {
        swap(b, c);
    }
    if a > b {
        swap(a, b);
    }
}

fn rand(seed: u64, max_rand: u64) -> u64 {
    let mut res = 0;
    for _i in 0..100 {
        let mut tmp_seed = seed;
        tmp_seed *= tmp_seed;
        res = tmp_seed % max_rand;
    }
    res
}

fn rand_array(array: &mut[i32]){
    let len = array.len() as u64;
    println!("{}", array[0]);
}


fn main() {
    println!("Hello, world!");
    let mut a = 2;
    let mut b = 4;

    swap(&mut a, &mut b);

    println!("{a}{b}");

    println!("Random value: {} {} {} {}", rand(72, 15), rand(62, 10), rand(182, 23), rand(56, 8));

    let mut array = [1,2,3,4,5];

    rand_array(&mut array);

}
