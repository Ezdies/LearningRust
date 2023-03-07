fn czy_przestepny(year : u32) -> bool{
   return (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0);  
}



fn main() {
    let month_number : u8 = 2;
    let year : u32 = 2001;
    let mut days_count :u32;

    if month_number % 2 == 0 {
        days_count = 30;
    } else {
        days_count = 31;
    }

    if month_number == 2 && czy_przestepny(year){
        days_count = 29;
    }

    if month_number == 2 && !czy_przestepny(year){
        days_count = 28;
    }

    println!("Days count {days_count}");
}
