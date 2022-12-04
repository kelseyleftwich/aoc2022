
pub mod solve {
    extern crate nom;
    use std::env;
    use std::fs;
    use std::collections::HashSet;

    fn share_char(a: &str, b: &str, c: &str) -> Vec<char> {
        // get which one is shorter
        let mut triple = [a, b, c];
     

         triple.sort_by(|x, y| {
             x.len().cmp(&y.len())
        });

        let [short, medium, long] = triple;

        
    
        // fill the set with the characters from the shorter string
        let set1: HashSet<char> = short.chars().collect();
        let set2: HashSet<char> = medium.chars().collect();
        long.chars().filter(|c| set1.contains(&c) && set2.contains(&c)).collect()
    }

    pub fn part_1() {
        let letterbank = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let args: Vec<String> = env::args().collect();

        let file_path = format!("src/day3/{}", &args[1]);

        println!("File path: {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
            
        let sacks= contents.split("\n").map(|line| line.split_at(line.len()/2));

        let res: Vec<HashSet<usize>>  = sacks.map(|(comp1, comp2)| {
            let set: HashSet<char> = comp1.chars().collect();

            let dupes: HashSet<usize> = comp2.chars().filter(move |c| { set.contains(&c)}).
            map(|c| {
                match letterbank.find(c) {
                    Some(x) => x + 1,
                    None => 0
                }
            }).
            collect();
            dupes
            
        }).collect();

        let answer = res.iter().fold(0, |acc, x| {
            acc + x.iter().fold(0, |acc2, y| { acc2 + y})

        });

        println!("{:#?}", answer)
    }

    pub fn part_2() {
        let args: Vec<String> = env::args().collect();

        let file_path = format!("src/day3/{}", &args[1]);

        println!("File path: {}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
            
        let sacks: Vec<&str>= contents.split("\n").collect();

        let  iter = sacks.rchunks(3);

        let letterbank = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";


        println!("{:#?}", iter.map(|x| {
            match x {
                [first, second, third] => {

                    let m = share_char(first, second, third);
                    
                    let mcopy = m.first().copied();
                    match mcopy {
                        Some(mval) => 
                match letterbank.find(mval) {
                    Some(x) => x + 1,
                    None => 0
                },
                _ => 0
                    }
                    
                    
                },
                _ => panic!("woops!")
            }
         }).sum::<usize>())
        
    }
}


