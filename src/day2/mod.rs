pub mod solve {
    extern crate nom;
    use std::env;
    use std::fs;

    #[derive(Debug)]
    enum Play {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Debug)]
    enum Outcome {
        Lose,
        Draw,
        Win,
    }

    pub fn part_1() {
        let args: Vec<String> = env::args().collect();

        let file_path = format!("src/day2/{}", &args[1]);

        println!("File path: {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines: i32 = contents
            .split("\n")
            .map(|x| {
                
                let values = x.split(" ").collect::<Vec<&str>>();
                let (first, second): (&str, &str) = (values[0], values[1]);
                    // .map(|item| 
                        
                        let opponent_play =  match first {
                         "A" => Play::Rock,
                         "B" => Play::Paper,
                         "C" => Play::Scissors,
                        _ => panic!("crash and burn"),
                        } ;

                        let outcome: Outcome = match second {
                        "X" => Outcome::Lose,
                        "Y" => Outcome::Draw,
                        "Z" => Outcome::Win,
                        _ => panic!("crash and burn"),
                        };

                        (opponent_play, outcome)
                    
                    // .collect::(Play , Outcome)()
            })
            .map(|round| {
                // let player2 = &round[0];
                // let player1 = &round[1];
                let (player2, outcome) = round;

                let choice = match outcome {
                    Outcome::Draw =>
                        &player2,
                    Outcome::Lose =>
                        match &player2 {
                            &Play::Paper => &Play::Rock,
                            &Play::Rock => &Play::Scissors,
                            &Play::Scissors => &Play::Paper
                        }
                    Outcome::Win =>
                        match &player2 {
                            &Play::Paper => &Play::Scissors,
                            &Play::Rock => &Play::Paper,
                            &Play::Scissors => &Play::Rock
                        }
                };

                let shape = match choice {
                    Play::Rock => 1,
                    Play::Paper => 2,
                    Play::Scissors => 3,
                };

                let outcome = match (choice, &player2) {
                    (Play::Rock, Play::Scissors) => 6,
                    (Play::Rock, Play::Paper) => 0,
                    (Play::Paper, Play::Rock) => 6,
                    (Play::Paper, Play::Scissors)=>0,
                    (Play::Scissors, Play::Paper) => 6,
                    (Play::Scissors, Play::Rock) => 0 ,
                    _ => 3
                };
                // println!("{:#?} vs {:#?} = {}", choice, player2, outcome);

                shape + outcome
            })
            .sum();

        println!("{:#?}", lines);
    }
}
