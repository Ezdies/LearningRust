fn swap(a: &mut i32, b: &mut i32){
    let tmp = *a;
    *a = *b;
    *b = tmp;
}


fn main() {
    let mut a = 2;
    let mut b = 3;

    println!("Wartości przed swapem: {} {}", a, b);

    swap(&mut a, &mut b);

    println!("Wartość po swapie: {} {}", a, b);

}
