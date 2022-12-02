pub mod solve {
    extern crate nom;
    use std::env;
    use std::fs;

    // fn max_calories(input: Vec<Vec<i32>>) -> Option<i32> {
    //     input.iter().map(|x| x.iter().sum()).max()
    // }

    fn top_calories(input: Vec<Vec<i32>>) -> Vec<i32> {
        input
            .iter()
            .map(|x| x.iter().sum::<i32>())
            .fold(vec![], |mut acc, x| {
                if acc.len() < 3 {
                    acc.push(x);
                    acc.sort();
                    acc
                } else {
                    match acc.first() {
                        None => acc,
                        Some(y) => {
                            if y < &x {
                                acc.splice(0..1, [x]);
                                acc.sort();
                            }

                            acc
                        }
                    }
                }
            })
    }


    pub fn part_2() {
        let args: Vec<String> = env::args().collect();

        let file_path = format!("src/day1/{}", &args[1]);

        println!("File path: {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let lines: Vec<Vec<i32>> = contents
            .split("\n\n")
            .map(|x| x.split("\n").map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

        let result = top_calories(lines);

        println!("{:?}", result.iter().sum::<i32>());
    }
}
