use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let content = fs::read_to_string(path)?;

    let mut sums = vec![0];
    for line in content.lines() {
        if line != "" {
            *sums.last_mut().unwrap() += line.parse::<i32>()?;
        } else {
            sums.push(0);
        }
    }

    sums.sort();
    let last_3_index = sums.len() - 3;
    let last_3 = &sums[last_3_index..];

    print!("{:?}", last_3.iter().sum::<i32>());
    Ok(())
}
