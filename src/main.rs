use std::fmt::Display;
use std::ops::Add;
#[derive(Clone, Copy)]
#[derive(Debug)]
struct Date {
    days: i32
}

impl Date {

    // create Date from year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        let mut leap_years: i32 = 0;
        let mut years: i32 = 0;
        let mut neg_or_pos: i32 = 1;
        for i in 0..year.abs() {
            if is_leap_year(i) == true {
                leap_years = leap_years + 1;
            } else {
                years = years + 1;
            }
        }
        if year < 0 {
            neg_or_pos = neg_or_pos * -1;
            let total = (((leap_years * 366) + (years * 365)) - days_in_month(year, month, day) - 1) * neg_or_pos;
            return Date { days: total };
        }

        let total = (leap_years * 366) + (years * 365) + days_in_month(year, month, day);
        return Date { days: total };
    }

    // convert back to year/month/day triple
    fn ymd(&self) -> (i32, i32, i32) {
        if self.days.abs() < 365 {
            if self.days < 0 {
                let left_over = 365 + self.days;
                let months = left_over / 30;
                let days = days_locate(months, left_over);
                return(-1, months, days);
            } else {
                let mut months = self.days / 30;
                if months == 0 {
                    months = 1;
                }
                let days = days_left(0,&mut months, self.days);

                return(0, months, days);
            }

        } else {
            let mut leap_years: i32 = 0;
            let mut norm_years: i32 = 0;
            let leap_calc: f32 = self.days as f32 / 4.0;
            let mut years: i32 = ((leap_calc / 365.0) * 3.0) as i32 + (leap_calc / 366.0) as i32;
            if years == 0 {
                years = 1;
            }

            for i in 0..years {
                if is_leap_year(i) == true {
                    leap_years = leap_years + 1;
                } else {
                    norm_years = norm_years + 1;
                }
            }
            let total = (leap_years * 366) + (norm_years * 365);
            let left_over_days = self.days - total;
            let mut months = left_over_days / 30;
            if months == 0 {
                months = 1;
            }

            let days = days_left(years,&mut months,left_over_days);


            return(years, months, days);
        }

    }

}


impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day) = self.ymd();
        if year <= 0 {
            let adjusted_year = (year * -1) + 1;
            write!(f, "{adjusted_year}/{month}/{day} BC")
        } else {
            write!(f, "{year}/{month}/{day}")
        }
    }
}

impl Add<i32> for Date {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            days: self.days + rhs,
        }
    }
}

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    } else if (year % 4 == 0) && (year % 100 != 0) {
        return true;
    } else {
        return false;
    }
}

fn days_in_month(year: i32, month:i32, day: i32) -> i32 {
    let days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut total: i32 = 0;
    if month - 1 == 0 {
        return day;
    } else {
        for i in 0..month - 1 {
            total = total + days_of_month[i as usize];
        }
    }
    if (is_leap_year(year) == true) && (month > 2) {
        total = total + 1;
    }
    total = total + day - 1;
    return total;
}

fn days_locate(month:i32, left_over:i32) -> i32 {
    let days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut total: i32 = left_over;
    if month - 1 == 0 {
        return left_over;
    } else {
        for i in 0..month - 1 {
            total = total - days_of_month[i as usize];
        }
    }
    if total < 0 {
        return (days_of_month[month as usize] + total) + 1;
    } else {
        return total + 1;
    }
}

fn days_left(year:i32, month: &mut i32, total_days:i32) -> i32 {
    let mut days_of_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut total = total_days;
    if is_leap_year(year) {
        days_of_month[1] = 29;
    }
    if total_days <= 31 {
        return total_days + 1;
    } else {
        for i in 0..*month {
            total = total - days_of_month[i as usize];
        }
        if total < 0 {
            return days_of_month[*month as usize] + total;
        } else if total > 0 {
            *month = *month + 1;
        }
        return total + 1;
    }
} 


fn main() {
    // testing from_ymd; should print Date { days: 738885 }
    println!("{:?}", Date::from_ymd(2022, 12, 31));

    // testing Add and Display
    let date = Date::from_ymd(-1, 12, 31);
    // increase date by 30 days, 60 days, etc.
    for i in 0..20 {
        // first iteration should print 2/12/31 BC
        println!("{}", date + i * 30);
    }
}

