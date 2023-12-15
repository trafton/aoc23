use std::num::ParseIntError;

fn main() {
    let input = include_str!("d2input.txt");

    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> u32 {
    todo!()
}

fn parse_game(game: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let g: Vec<&str> = game.split(':').collect();
    let gameidstr = g.first().unwrap();
    let gameid = parse_gameid(&gameidstr)?;

    //split semicolons
    //each is a round

    Ok(Game {
        id: gameid,
        rounds: Vec::new(),
    })
}

fn parse_rounds(r_str: &str) -> Vec<Round> {
    // let mut rounds: Vec<Round> = Vec::new();
    // for round in r_str.split(';') {
    //     dbg!(round);
    // }

    // rounds

    r_str.split(';').map(|r| {
        dbg!(r);

        Round::default()
    })
    .collect::<Vec<Round>>()
}

fn parse_gameid(gameidstr: &str) -> Result<u32, ParseIntError> {
    let idstr = gameidstr.split_whitespace().last().unwrap();
    u32::from_str_radix(idstr, 10)
}

#[derive(PartialEq, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new(red: u32, green: u32, blue: u32) -> Round {
        Round { red, green, blue }
    }
}

impl Default for Round {
    fn default() -> Self {
        Self { red: Default::default(), green: Default::default(), blue: Default::default() }
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[cfg(test)]
mod tests {
    use super::*;

    //  #[test]
    fn _it_works() {
        todo!()
    }
    #[test]
    fn parses_round() {
        let roundstr = "4 green, 10 red, 9 blue";

        let expected = Round::new(10, 4, 9);

        let res = parse_rounds(roundstr);

        assert_eq!(res, vec![expected]);
    }

    #[test]
    fn parses_gameid() {
        let gameidstr = "Game 12";

        let expected = 12;

        let res = parse_gameid(gameidstr);

        assert_eq!(res, Ok(expected));
    }

      #[test]
    fn parses_a_game() {
        //let game = "Game 38: 4 green, 10 red, 9 blue; 12 green, 2 blue, 2 red; 6 red, 6 blue, 9 green; 1 blue, 1 green, 6 red; 3 blue, 1 red, 5 green; 5 blue, 2 red, 12 green";
        let game = "Game 12: 1 green, 1 red, 1 blue; 2 green, 2 red, 2 blue; 1 red";

        let expected = Game {
            id: 12,
            rounds: vec![
                Round::new(1, 1, 1),
                Round::new(2, 2, 2),
                Round::new(1, 0, 0),
            ],
        };

        match parse_game(game){
            Ok(g) =>  assert_eq!(g, expected),
            Err(_) => assert!(true),
        }
       
    }
}
