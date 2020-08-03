pub struct Solution;

impl Solution {
    pub fn to_str(num: i32, one: char, five: char, ten: char) -> String {
        match num {
            1 => vec![one].iter().collect(),
            2 => vec![one, one].iter().collect(),
            3 => vec![one, one, one].iter().collect(),
            4 => vec![one, five].iter().collect(),
            5 => vec![five].iter().collect(),
            6 => vec![five, one].iter().collect(),
            7 => vec![five, one, one].iter().collect(),
            8 => vec![five, one, one, one].iter().collect(),
            9 => vec![one, ten].iter().collect(),
            _ => String::from(""),
        }
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut ret = String::from("");

        if num >= 1000 {
            ret += &Solution::to_str(num / 1000, 'M', '_', '_')
        }

        if num >= 100 {
            ret += &Solution::to_str(num / 100 % 10, 'C', 'D', 'M')
        }

        if num >= 10 {
            ret += &Solution::to_str(num / 10 % 10, 'X', 'L', 'C')
        }

        if num >= 1 {
            ret += &Solution::to_str(num % 10, 'I', 'V', 'X')
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0012() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
