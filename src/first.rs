use std::io::BufRead;

use anyhow::Context;

pub fn first_calories_count() -> anyhow::Result<()> {
    let calories = get_carried_calories(get_lines()?).context("failed to get the list of carried calories")?;

    println!(
        "Total calories carried by the Elf with the most calories is {}",
        get_maximum_calories(calories).context("failed to count the maximum calories carried by an elf")?,
    );
    Ok(())
}

pub fn second_calories_count() -> anyhow::Result<()> {
    let lines = get_carried_calories(get_lines()?).context("failed to get the list of carried calories")?;

    println!(
        "Three maximum weights carried total to {} calories",
        get_three_maximum_calories_in_total(lines).context("failed to count the total of three maximum calories carried")?
    );
    Ok(())
}

fn get_lines() -> anyhow::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let filename = crate::utils::prompt_user("Enter the input file name: ")?;
    let file = std::fs::File::open(filename).context("failed to open the input file")?;
    Ok(std::io::BufReader::new(file).lines())
}

fn get_maximum_calories(calories: Vec<u32>) -> anyhow::Result<u32> {
    let max_calories = find_n_elements(calories, 1, |current, best| {
        current > best
    }).context("failed to get the maximal element")?;
    Ok(max_calories[0])
}

fn get_three_maximum_calories_in_total(calories: Vec<u32>) -> anyhow::Result<u32> {
    let three_max_calories = find_n_elements(calories, 3, |current, best| {
        current > best
    }).context("failed to get the three maximum calories carried")?;
    Ok(
        three_max_calories.iter().fold(0, |acc, el| acc + el)
    )
}

fn get_carried_calories(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> anyhow::Result<Vec<u32>> {
    let mut current_calories: u32 = 0;
    let mut calories: Vec<u32> = Vec::new();
    for line in lines {
        let l = line.context("failed to read a line from the input file")?;
        if l.trim().len() != 0 {
            let n = l.parse::<u32>().context("failed to parse the line from the input file as a number")?;
            current_calories += n;
            continue;
        }
        calories.push(current_calories);
        current_calories = 0;
    }
    calories.push(current_calories);
    Ok(calories)
}

fn find_n_elements<T: Clone>(mut v: Vec<T>, n: usize, p: fn(current: &T, best: &T) -> bool) -> anyhow::Result<Vec<T>> {
    if n > v.len() {
        anyhow::bail!("cannot find {} elements: the vector size {} is smaller than {}", n, v.len(), n)
    }
    let v_len = v.len();
    for iteration in 0..n {
        let mut best_idx = 0;
        for i in 1..(v_len-iteration) {
            if p(&v[i], &v[best_idx]) {
                best_idx = i;
            }
        }
        v.swap(best_idx, v_len-1-iteration);
    }
    Ok(v[(v.len()-n)..].to_vec())
}