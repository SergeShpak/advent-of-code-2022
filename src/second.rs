use anyhow::Context;

pub fn second_theoretical_score_count() -> anyhow::Result<()> {
    println!(
        "Expected score is: {}",
        calculate_score(GameRules::calculate_round_score_first)?,
    );
    Ok(())
}

pub fn second_real_score_count() -> anyhow::Result<()> {
    println!(
        "Expected score is: {}",
        calculate_score(GameRules::calculate_round_score_second)?,
    );
    Ok(())
}

fn calculate_score<F>(calculate_round_score: F) -> anyhow::Result<u32> where
    F: Fn(&GameRules, String) -> anyhow::Result<u32> {
        let game = crate::utils::get_lines()?;
        let game_rules = GameRules::new();
        let mut game_results: u32 = 0;
        for round in game {
            let r = round.context("failed to read a line from the game file")?;
            game_results += match calculate_round_score(&game_rules, r) {
                Ok(s) => s,
                Err(e) => {
                    log::warn!("failed to calculate the score: {:#}", e);
                    0
                },
            };
        }
        Ok(game_results) 
    }

struct GameRules {
    rules: std::collections::HashMap<char, (char, char, u32)>,
}

impl GameRules {
    pub fn new() -> GameRules {
        GameRules {
            rules: std::collections::HashMap::from([
            ('A', ('C', 'B', 1)),  // rock
            ('B', ('A', 'C', 2)),  // paper
            ('C', ('B', 'A', 3)),  // scissors
        ])}
    }

    fn calculate_round_score_first(&self, round: String) -> anyhow::Result<u32> {
        let move_transformation = |c: char| {
            match c {
                'Y' => Ok('B'),
                'X' => Ok('A'),
                'Z' => Ok('C'),
                c => anyhow::bail!("an unexpected third character {} found in the round string", c),
            }
        };
        let round_info = Self::parse_game_round(round, move_transformation).context("failed to parse the passed game round")?;
        let generate_none_err = |entry: char| -> String { format!("cannot find a rule for the entry {}", entry) };
        let op_move = match self.rules.get(&round_info.0) {
            None => anyhow::bail!(generate_none_err(round_info.0)),
            Some(m) => *m, 
        };
        let our_move = round_info.1;
        let our_move_score = match self.rules.get(&round_info.1) {
            None => anyhow::bail!(generate_none_err(round_info.1)),
            Some(m) => m.2, 
        };
        let mut result:u32 = 0;
        if our_move == op_move.1 {
            result = 6
        } else if our_move != op_move.0 {
            result = 3
        }
        result += our_move_score;
        log::info!("({}, {}) -> {}", round_info.0, our_move, result);
        Ok(result)
    }

    fn calculate_round_score_second(&self, round: String) -> anyhow::Result<u32> {
        let move_transformation= |c: char| {
            match c {
                'Y' => Ok(2 as u32),
                'X' => Ok(0 as u32),
                'Z' => Ok(1 as u32),
                c => anyhow::bail!("an unexpected third character {} found in the round string", c),
            }
        };
        let round_info = Self::parse_game_round(round, move_transformation).context("failed to parse the passed game round")?;
        let generate_none_err = |entry: char| -> String { format!("cannot find a rule for the entry {}", entry) };
        match self.rules.get(&round_info.0) {
            None => anyhow::bail!("failed to find a rule for the move {}", round_info.0),
            Some((win, lose, score)) => {
                match round_info.1 {
                    0 => match self.rules.get(win) {
                        Some((_, _, score)) => Ok(*score),
                        None => anyhow::bail!(generate_none_err(*win))
                    },
                    1 => match self.rules.get(lose) {
                        Some((_, _, score)) => Ok(*score + 6),
                        None => anyhow::bail!(generate_none_err(*win))
                    },
                    2 => Ok(*score + 3),
                    _ => anyhow::bail!("an unknown round result {} passed", round_info.1),
                }
            }
        }
    }

    fn parse_game_round<T, F>(round: String, our_move_transformation: F) -> anyhow::Result<(char, T)> where
            F: Fn(char) -> anyhow::Result<T> {
        if round.len() != 3 {
            anyhow::bail!("passed game round string {} has an unexpected format", round);
        }
        let mut round_info = round.chars();
        let none_err_msg = format!("failed to read the opponent move from the round string {}", round);
        let op_move = match round_info.next() {
            Some(op_move) => op_move,
            None => {
                anyhow::bail!(none_err_msg);
            },
        };
        match round_info.next() {
            Some(' ') => (),
            Some(c) => anyhow::bail!("passed game round string {} has an unexpected second character {}", round, c),
            None => anyhow::bail!(none_err_msg),
        };
        let our_move = match round_info.next() {
            Some(c) => our_move_transformation(c).context(format!("failed to perform our move transformation for the round string {}", round))?,
            None => anyhow::bail!(none_err_msg),
        };
        Ok((op_move, our_move))
    }
}
