#[repr(C)]
pub struct MyDate {
    pub day_of_month: u16,
    pub month: u16,
    pub year: i16,
}

extern "C" {
    pub fn convert(date: MyDate) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use julianday::JulianDay;

    #[test]
    fn converts_correctly() {
        fn test_conversion(y: i16, m: u16, d: u16) {
            let my_jd = unsafe {
                convert(MyDate {
                    day_of_month: d,
                    month: m,
                    year: y,
                })
            };

            let nd = NaiveDate::from_ymd(y as i32, m as u32, d as u32);
            let jd = JulianDay::from(nd);
            let jdi: i32 = jd.into();

            assert_eq!(jdi as u32, my_jd);
        }

        let days_in_month: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let leap_days_in_month: [u16; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        for y in 1i16..4000 {
            for m in 1u16..12 {
                let dim = if (y % 4 == 0) && (!(y % 100 == 0) || (y % 400 == 0)) {
                    leap_days_in_month[(m - 1) as usize]
                } else {
                    days_in_month[(m - 1) as usize]
                };

                for d in 1u16..dim {
                    test_conversion(y, m, d);
                }
            }
        }
    }
}
