pub mod first;
pub mod utils;

fn main() {
    match run() {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{:#}", e);
            std::process::exit(1);
        },
    }
}

type PuzzleSolution = fn() -> anyhow::Result<()>;

fn run<'a>() -> anyhow::Result<()> {
    let puzzle_index = crate::utils::prompt_user("Enter the puzzle index: ")?;
    let puzzles = get_puzzles_map();
    match puzzles.get(puzzle_index.as_str()) {
        Some(solution) => solution(),
        None => {
            anyhow::bail!(get_unknown_puzzle_index_err_msg(puzzle_index.as_str(), puzzles))
        }
    }
}

fn get_puzzles_map() -> std::collections::HashMap<&'static str, PuzzleSolution> {
    let puzzles = [
        ("1.1", crate::first::first_calories_count),
    ];
    let mut puzzles_map: std::collections::HashMap<&'static str, PuzzleSolution>  = std::collections::HashMap::new();
    for (name, solution) in puzzles {
        puzzles_map.insert(name, solution);
    }
    puzzles_map
}

fn get_unknown_puzzle_index_err_msg<'a>(bad_idx: &'a str, puzzles: std::collections::HashMap<&'static str, PuzzleSolution>) -> std::string::String {
    let msg = format!("Unknown index \"{}\" passed. Try an element from this list instead:", bad_idx);
    puzzles.keys().fold(std::string::String::from(msg), |f, s| f + "\n" + format!("    - {}", s).as_str())
}