fn main() {
    let input = include_str!("input1.txt");
    
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> u32 {
    //take nums out of str as arr of nums
    input
        .lines()
        .map(|line| {
            let nums = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

            let n: String = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
                            
            n.parse::<u32>().unwrap()
        })
        .sum()

    //0
}

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
