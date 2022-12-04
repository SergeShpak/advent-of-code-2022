use std::io::{BufRead, Write};

use anyhow::Context;

pub fn prompt_user<'a>(msg: &'a str) -> anyhow::Result<String> {
    print!("{}", msg);
    std::io::stdout().flush().context("failed to flush the stdout")?;

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).context("failed to read the user input")?;
    input.pop();
    Ok(input)
}

pub fn get_lines() -> anyhow::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let filename = crate::utils::prompt_user("Enter the input file name: ")?;
    let file = std::fs::File::open(filename).context("failed to open the input file")?;
    Ok(std::io::BufReader::new(file).lines())
}