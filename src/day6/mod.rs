pub mod solve {
    // extern crate nom;
    // use itertools::Itertools;
    use std::{
        fs::File,
        io::{prelude::*, BufReader},
        path::Path
    };

    use itertools::Itertools;

    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }

    pub fn part_1() {
        let lines = lines_from_file("src/day6/input.txt");

        let result = lines.iter().map(|line| {
            let inter = line.chars().collect::<Vec<char>>();
            let mut windows = inter.windows(4);

            let mut counter = 3;

            while let Some(w) = windows.next() {
                let uniq_chars = w.into_iter().unique().count();

                counter += 1;
                
                if uniq_chars == 4 {
                    break;
                }
                
            }

            counter


        }).collect::<Vec<usize>>();

             dbg!(result);


    } 
pub fn part_2() {
        let lines = lines_from_file("src/day6/input.txt");

        let result = lines.iter().map(|line| {
            let inter = line.chars().collect::<Vec<char>>();
            let mut windows = inter.windows(14);

            let mut counter = 13;

            while let Some(w) = windows.next() {
                let uniq_chars = w.into_iter().unique().count();

                counter += 1;
                
                if uniq_chars == 14 {
                    break;
                }
                
            }

            counter


        }).collect::<Vec<usize>>();

             dbg!(result);


    } 
}
