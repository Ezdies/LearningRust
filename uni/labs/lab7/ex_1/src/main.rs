fn powtorki(t: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];

    let mut i = 1;

    while i < t.len() - 1 {
        if t[i] == t[i + 1] || t[i] == t[i - 1]{
            v.push(t[i]);
        }
        i += 1;
    }
    return v;
}

fn main() {
    let res = powtorki(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]); 
    println!("{:?}", res)
}
