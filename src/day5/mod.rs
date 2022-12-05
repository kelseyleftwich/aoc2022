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
        // let lines = lines_from_file("src/day5/sample.txt");
        let lines = lines_from_file("src/day5/input.txt");

        let (diagram, raw_instructions) = match lines.split(|elem| elem == "").collect_tuple() {
            Some((d, i)) => (d, i),
            _ => panic!("whoops"),
        };

        let sub_len = 4;

        let mut rows = diagram
            .iter()
            .map(|line| {
                let mut chars = line.chars();
                let sub_string = (0..)
                    .map(|_| chars.by_ref().take(sub_len).collect::<String>())
                    .take_while(|s| !s.is_empty())
                    .map(|mut elem| {
                        elem.remove_matches("[");
                        elem.remove_matches("]");
                        elem.remove_matches(" ");
                        
                        elem})
                    .collect::<Vec<String>>();

                sub_string
            })
            .collect::<Vec<_>>();

        let labels = rows.pop();


        rows.reverse();

        let mut stacks: Vec<Vec<String>> = Vec::new();
        
         for _i in 0..(labels.unwrap().len()) {
            stacks.push(Vec::new());
        }
        
        rows.iter().for_each(| row| {
            row.iter().enumerate().for_each(|(index, item)| {
                let mut current_stack = match stacks.get(index) {
                    Some(stack) => stack.clone(),
                    None => {
                        let v:Vec<String> = Vec::new();
                        v
                    }
                };

                if item != "" {
                    current_stack.push(item.to_string());
                }
                
                stacks.splice(index..(index+1), [current_stack]);

            })
        });

        let instructions = raw_instructions.iter().map(|line| {
            let mut l = line.to_string();
            l.remove_matches("move ");
            l = l.replace(" from ", ",");
            l = l.replace(" to ", ",");

            let res = match l.split(",").map(|s| s.to_string().parse::<i32>().unwrap()).collect_tuple() {
                Some((qty, start, end)) => (qty, start, end),
                _ => panic!("uh oh")
            };

            res
        }).collect::<Vec<(i32, i32, i32)>>();
        

        instructions.iter().for_each(|(qty, start, end)| {
            let mut to_move = Vec::new();
            let from: usize = (*start - 1).try_into().unwrap();
            let mut start_stack = stacks.get(from).unwrap().clone();
            for _i in 0..*qty {
                let item = start_stack.pop().unwrap().clone();
                to_move.push(item);
            }

            stacks.splice(from..from+1, [start_stack]);
            let to: usize = (*end - 1).try_into().unwrap();

            let mut end_stack = stacks.get(to).unwrap().clone();

            to_move.reverse(); // part_2

            end_stack.append(&mut to_move);
            stacks.splice(to..to+1, [end_stack]);



        });


        println!("{:#?}", stacks);

        let code = stacks.iter().map(|stack| {
            stack.clone().pop().unwrap()
        }).join("");

        println!("{:#?}", code);



    }
}
