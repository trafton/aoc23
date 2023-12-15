fn main() {
    let input = include_str!("d1input1.txt");
    
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> u32 {    
    input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter(|c| c.is_numeric());
            let mut n: String = "".into();
            if let Some(f) = nums.next() {
                let l = nums.last().unwrap_or(f);
                n = format!("{}{}", f, l);
            }
                 
            n.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 1abc2 -> 12
        // pqr3stu8vwx -> 38
        // a1b2c3d4e5f -> 15
        // treb7uchet -> 77
        let res = part1("1abc2\r\npqr3stu8vwx\r\na1b2c3d4e5f\r\ntreb7uchet");
        assert_eq!(res, 142);
    }
}
