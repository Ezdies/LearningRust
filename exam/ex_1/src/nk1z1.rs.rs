fn fizz_buzz(n: u32) {
    for i in 0..=n {
        if i % 15 == 0 || i % 21 == 0 || i % 35 == 0 {
            if i % 3 == 0 && i % 5 == 0 && i % 7 == 0 {
                println!("PlimPlumPlam");
            }
            else if i % 15 == 0 {
                println!("PlimPlum");
            }
            else if i % 21 == 0 {
                println!("PlimPlam");
            }
            else if i % 35 == 0 {
                println!("PlumPlam");
            }
        } else if i % 3 == 0 {
            println!("Plim");
        } else if i % 5 == 0 {
            println!("Plum");
        } else if i % 7 == 0 {
            println!("Plam");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    fizz_buzz(15);
}
