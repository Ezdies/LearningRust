fn czy_przestepny(year : u32) -> bool{
   return (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0);  
}



fn main() {
    let month_number : u8 = 20;
    let year : u32 = 2001;
    let days_count :u32;

    if !czy_przestepny(year){
        if month_number == 1 {
            days_count = 31;
            println!("{days_count}");
        }
        if month_number == 2 {
            days_count = 28;
            println!("{days_count}");
        }
        if month_number == 3 {
            days_count = 30;
            println!("{days_count}");
        }

    }

    println!("Hello, world!");
}
