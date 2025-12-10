fn age(present_date: i32, present_month: i32, present_year: i32, birth_date: i32, birth_month: i32, birth_year: i32) {
    let month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    let (mut calc_date, mut calc_month, mut calc_year) = 
        (present_date, present_month, present_year);
    
    if birth_date > calc_date {
        calc_date += month[(birth_month - 1) as usize];
        calc_month -= 1;
    }
    
    if birth_month > calc_month {
        calc_year -= 1;
        calc_month += 12;
    }
    
    let final_date = calc_date - birth_date;
    let final_month = calc_month - birth_month;
    let final_year = calc_year - birth_year;
    
    print!("Present Age Years: {} Months: {} Days: {}", 
             final_year, final_month, final_date);
}

fn main() {
    let present_date = 21;
    let present_month = 9;
    let present_year = 2019;
    let birth_date = 25;
    let birth_month = 9;
    let birth_year = 1996;
    
    age(present_date, present_month, present_year, birth_date, birth_month, birth_year);
}