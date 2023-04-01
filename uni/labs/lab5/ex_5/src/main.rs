fn roman_to_arabic(roman_literal: &str) -> Option<u32> {
    let numerals = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];

    for (numeral, value) in &numerals {
        if *numeral == roman_literal.chars().next().unwrap() {
            return Some(*value);
        }
    }
    None
}

fn main() {
    let res = roman_to_arabic("V");
    println!("V po arabsku to: {}", res.unwrap_or(0));
}
