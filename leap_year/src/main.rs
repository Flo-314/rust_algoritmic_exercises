fn main() {
    let is_leap = is_leap_year(151);
    println!("{}!", is_leap);
}
pub fn is_leap_year(year: u64) -> bool {
    let is_divisible_four: bool = if (year % 4 == 0) { true } else { false };
    let is_divisible_hundred: bool = if (year % 100 == 0) { true } else { false };
    let is_divisible_four_hundred: bool = if (year % 400 == 0) { true } else { false };

    let is_leap: bool;
    if (is_divisible_four && !is_divisible_hundred) {
        is_leap = true
    } else if (is_divisible_four_hundred) {
        is_leap = true
    } else {
        is_leap = false
    }
    is_leap
}

//on every year that is evenly divisible by 4
//except every year that is evenly divisible by 100
//unless the year is also evenly divisible by 400
