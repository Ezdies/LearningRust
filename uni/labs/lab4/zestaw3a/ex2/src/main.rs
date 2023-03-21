fn print_arr(arr: &[usize]) {
    for elem in arr {
        println!("{elem}");
    }
}

fn swap_arr(arr: &mut[usize], a: usize, b: usize){
 arr.swap(a, b);  
}

fn main() {
    let mut tab = [1, 2, 3, 4];
    println!("Tablica przed swapowaniem:");
    print_arr(&tab);
    let a = 2;
    let b = 3;
    swap_arr(&mut tab, a, b);
    println!("Tablica po swapowaniu:");
    print_arr(&tab);
}
