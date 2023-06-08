use std::fs;

pub fn main() {
    let _my_name = "Jos";
    let file_path = "/home/simon/rust-projects/test/assets/advent1.txt";
    let file = fs::read_to_string(file_path);
    match file {
        Err(err) => {
            panic!("Failed to read file @ {:?}: {:?}", file_path, err)
        }
        Ok(file) => {
            let splitted = file.split_terminator('\n');
            let mut totals: Vec<i32> = vec![];
            let mut sum = 0;
            for line in splitted {
                if line.is_empty() {
                    totals.push(sum);
                    sum = 0;
                } else {
                    let i = line
                        .parse::<i32>()
                        .expect(&format!("Failed to parse {} to i32", line));
                    sum = sum + i;
                }
            }
            totals.push(sum);

            totals.sort();
            match totals[..] {
                [.., third, second, first] => println!("Total: {:?}", first + second + third),
                _ => panic!("Less than 3 elves"),
            }
        }
    }
}
