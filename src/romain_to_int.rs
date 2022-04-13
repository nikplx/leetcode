use std::str::FromStr;

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Numeral {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000
}

impl FromStr for Numeral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(Numeral::I),
            "V" => Ok(Numeral::V),
            "X" => Ok(Numeral::X),
            "L" => Ok(Numeral::L),
            "C" => Ok(Numeral::C),
            "D" => Ok(Numeral::D),
            "M" => Ok(Numeral::M),
            _ => Err(())
        }
    }
}

pub fn romain_to_int(s: String) -> i32 {
    let nums = s.chars()
        .map(|c| {
            Numeral::from_str(c.to_string().as_str()).unwrap()
        })
        .collect::<Vec<Numeral>>();

    nums.iter().enumerate()
        .fold(0, |s, (i, n)| {
            let mut val = Numeral::clone(n)  as i32;

            match nums.get(i+1) {
                None => s + val,
                Some(next) => {
                    if next > n {
                        val *= -1
                    }
                    s + val
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::romain_to_int::*;

    #[test]
    fn romain_to_int_test() {
        let inp = vec![
            "III", "VII", "IV", "IX", "XIII", "DM", "CM", "CML", "MMMLC",
            "MMMCCCXXXIII", "MMCMLCIII"
        ];
        let out = vec![
            3, 7, 4, 9, 13, 500, 900, 950, 3050,
            3333, 2953

        ];
        for (idx, input) in inp.iter().enumerate() {
            match out.get(idx) {
                None => continue,
                Some(output) => assert_eq!(romain_to_int(input.to_string()), *output)
            };
        }
    }
}
