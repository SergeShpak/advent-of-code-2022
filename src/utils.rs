use std::{io::Write};

use anyhow::Context;

pub fn prompt_user<'a>(msg: &'a str) -> anyhow::Result<String> {
    print!("{}", msg);
    std::io::stdout().flush().context("failed to flush the stdout")?;

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).context("failed to read the user input")?;
    input.pop();
    Ok(input)
}