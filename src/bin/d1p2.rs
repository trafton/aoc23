fn main() {
    let input = include_str!("input1.txt");

    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = find_nums_in_str(line);
            let f = nums.first();
            let last = nums.last();

            f.unwrap().value * 10 + last.unwrap().value
        })
        .sum()
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone, Copy)]
struct Num {
    loc: usize,
    value: u32,
}

fn find_nums_in_str(num_str: &str) -> Vec<Num> {
    let num_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine","0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];    

    let mut res: Vec<Num> = Vec::new();

    for word in num_words.iter().enumerate() {
        for (i, _) in num_str.match_indices(word.1) {
            res.push(Num {
                loc: i,
                value: (word.0 as u32) % 10,
            });
        }
    }

    res.sort();
  
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_2() {
        // two1nine - 29
        // eightwothree -83
        // abcone2threexyz - 13
        // xtwone3four - 24
        // 4nineeightseven2 - 42
        // zoneight234 - 14
        // 7pqrstsixteen - 76
        let res = part1("two1nine\r\neightwothree\r\nabcone2threexyz\r\nxtwone3four\r\n4nineeightseven2\r\nzoneight234\r\n7pqrstsixteen");
        assert_eq!(res, 281);
    }

    #[test]
    fn finds_word_num_in_str() {
        let test_str = "two1nine";

        let res = find_nums_in_str(test_str);
        let expected = vec![
            Num { loc: 0, value: 2 },
            Num { loc: 3, value: 1 },
            Num { loc: 4, value: 9 },
        ];

        assert_eq!(res, expected);
    }

    #[test]
    fn finds_all_instances_of_word() {
        let test_str = "two1ninetwo";

        let res = find_nums_in_str(test_str);
        let expected = vec![
            Num { loc: 0, value: 2 },
            Num { loc: 3, value: 1 },
            Num { loc: 4, value: 9 },
            Num { loc: 8, value: 2 },
        ];

        assert_eq!(res, expected);
    }
}
