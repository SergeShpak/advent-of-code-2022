use std::io::{BufRead};

use anyhow::Context;

pub fn first_calories_count<'a>() -> anyhow::Result<()> {
    let filename = crate::utils::prompt_user("Enter the input file name: ")?;
    let file = std::fs::File::open(filename).context("failed to open the input file")?;
    let lines = std::io::BufReader::new(file).lines();

    println!(
        "Total calories carried by the Elf with the most calories is {}",
        get_maximum_calories(lines).context("failed to count the maximum calories carried by an elf")?,
    );
    Ok(())
}

fn get_maximum_calories(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> anyhow::Result<u32> {
    let mut maximum_calories: u32 = 0;
    let mut current_calories: u32 = 0;
    for line in lines {
        let l = line.context("failed to read a line from the input file")?;
        if l.trim().len() != 0 {
            let n = l.parse::<u32>().context("failed to parse the line from the input file as a number")?;
            current_calories += n;
            continue;
        }
        if current_calories > maximum_calories {
            maximum_calories = current_calories;
        }
        current_calories = 0;
    }

    if current_calories > maximum_calories {
        maximum_calories = current_calories;
    }

    Ok(maximum_calories)
}

