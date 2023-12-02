fn main() {
    let input = include_str!("input1.txt");

    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) ->String {
    input.into()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = part1("test");
        assert_eq!(res, "test");
    }
}