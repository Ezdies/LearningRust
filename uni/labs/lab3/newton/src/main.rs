// dane: funkcja, <a, b>, epsilon, liczba iteracji,
// wyniki: miejsce zerowe funkcji
//f(x) = x^2 - 4
//f'(x) = 2x
//mz = +- 2

fn met_newt_while(x0: f64, eps: f64, n: u128) -> f64 {
    let mut i: u128 = 0;
    let mut x1: f64 = 0.0;

    while f64::abs(f(x1)) < eps && f64::abs(x1 - x0) < eps || (i < n) {
        x1 = x0 - f(x0) / fp(x0);
        i += 1;
    }

    x1
}

fn met_newt_loop(x0: f64, eps: f64, n: u128) -> f64 {
    let mut i: u128 = 0;
    let mut x1: f64 = 0.0;

    loop {
        if f64::abs(f(x1)) < eps && f64::abs(x1 - x0) < eps || (i < n) {
            x1 = x0 - f(x0) / fp(x0);
        } else {
            break;
        }
        i += 1;
    }

    x1
}


fn f(x: f64) -> f64 {
    f64::powf(x, 2.0) - 4.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn main() {
    let res_while = met_newt_while(3.0, 0.0001, 100);

    println!("{res_while}");

    let res_loop = met_newt_loop(3.0, 0.0001, 100);

    println!("{res_loop}");
}
