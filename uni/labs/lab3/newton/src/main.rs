// dane: funkcja, <a, b>, epsilon, liczba iteracji,
// wyniki: miejsce zerowe funkcji
//f(x) = x^2 - 4
//f'(x) = 2x
//mz = +- 2

fn met_newt_while(mut x0: f64, eps: f64, n: u128) -> f64 {
    let mut i: u128 = 0;
    let mut x1: f64 = 0.0;

    while f64::abs(f(x1)) >= eps || f64::abs(x1 - x0) >= eps && i < n {
        x1 = x0 - f(x0) / fp(x0);
        x0 = x1;
        i += 1;
    }

    x1
}

fn met_newt_loop(mut x0: f64, eps: f64, n: u128) -> f64 {
    let mut i: u128 = 0;
    let mut x1: f64;

    loop {
        x1 = x0 - f(x0) / fp(x0);
        if f64::abs(f(x1)) < eps && f64::abs(x1 - x0) < eps || i >= n {
            break;
        }
        x0 = x1;
        i += 1;
    }

    x1
}

fn met_newt_for(mut x0: f64, eps: f64, n: u128) -> f64 {
    let mut x1: f64 = x0;

    for _i in 0..n {
        x1 = x0 - f(x0) / fp(x0);
        if f64::abs(f(x1)) < eps && f64::abs(x1 - x0) < eps {
            break;
        }
        x0 = x1;
    }

    x1
}

fn met_newt_rec(x0: f64, eps: f64, n: u128, i: u128) -> f64 {
    let x1 = x0 - f(x0) / fp(x0);
    if f64::abs(f(x1)) < eps && f64::abs(x1 - x0) < eps || (i >= n) {
        x1
    } else {
        met_newt_rec(x1, eps, n, i + 1)
    }
}

fn f(x: f64) -> f64 {
    f64::powf(x, 2.0) - 4.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn main() {
    let res_while = met_newt_while(3.0, 0.000001, 10000);

    println!("While loop version: {res_while}");

    let res_loop = met_newt_loop(3.0, 0.000001, 10000);

    println!("Rust loop version: {res_loop}");

    let res_for = met_newt_for(3.0, 0.000001, 10000);

    println!("For loop version: {res_for}");

    let res_rec = met_newt_rec(3.0, 0.000001, 10000, 0);

    println!("Recursive version: {res_rec}");
}
