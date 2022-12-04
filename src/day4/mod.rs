pub mod solve {
    extern crate nom;
    use itertools::Itertools;
    use std::env;
    use std::fs;

    fn get_contents() -> String {
        let args: Vec<String> = env::args().collect();

        let file_path = format!("src/day4/{}", &args[1]);

        println!("File path: {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        contents
    }

    fn get_terminators(assignment: &str) -> (i32, i32) {
        assignment
            .splitn(2, "-")
            .map(|s| s.parse().unwrap())
            .collect_tuple::<(i32, i32)>()
            .unwrap()
    }

    pub fn part_1() {
        let answer = get_contents()
            .split("\n")
            .map(|pair| {
                let (first, second) = pair.split(",").collect_tuple::<(&str, &str)>().unwrap();

                let (first_start, first_end): (i32, i32) = get_terminators(first);
                let (second_start, second_end): (i32, i32) = get_terminators(second);

                if first_start <= second_start && first_end >= second_end
                    || second_start <= first_start && second_end >= first_end
                {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>();

        println!("{:#?}", answer)
    }

  pub fn part_2() {
        let answer = get_contents()
            .split("\n")
            .map(|pair| {
                let (first, second) = pair.split(",").collect_tuple::<(&str, &str)>().unwrap();

                let (first_start, first_end): (i32, i32) = get_terminators(first);
                let (second_start, second_end): (i32, i32) = get_terminators(second);

                if first_start <= second_start {
                    if second_start <= first_end {
                        1
                    } else {
                        0
                    }
                } else {
                    if first_start <= second_end {
                        1
                    } else {
                        0
                    }
                }

            })
            .sum::<i32>();

        println!("{:#?}", answer)
    }
}
