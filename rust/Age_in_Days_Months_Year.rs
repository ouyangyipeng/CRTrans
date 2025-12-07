fn age(present_date: i32, present_month: i32, present_year: i32, birth_date: i32, birth_month: i32, birth_year: i32) -> (i32, i32, i32) {
    let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    let mut adjusted_date = present_date;
    let mut adjusted_month = present_month;
    let mut adjusted_year = present_year;
    
    // Adjust if birth day is greater than present day
    if birth_date > adjusted_date {
        let month_idx = (adjusted_month - 1) as usize;
        if month_idx < month_days.len() {
            adjusted_date += month_days[month_idx];
        }
        adjusted_month -= 1;
    }
    
    // Adjust if birth month is greater than present month
    if birth_month > adjusted_month {
        adjusted_year -= 1;
        adjusted_month += 12;
    }
    
    // Calculate final age
    let final_date = adjusted_date - birth_date;
    let final_month = adjusted_month - birth_month;
    let final_year = adjusted_year - birth_year;
    
    (final_year, final_month, final_date)
}

fn main() {
    let present_date = 21;
    let present_month = 9;
    let present_year = 2019;
    let birth_date = 25;
    let birth_month = 9;
    let birth_year = 1996;
    
    let (years, months, days) = age(present_date, present_month, present_year, birth_date, birth_month, birth_year);
    
    print!("Present Age Years: {} Months: {} Days: {}", years, months, days);
}