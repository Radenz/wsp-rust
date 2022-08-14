use std::{env, fs, io::stdin, process::exit, time::Instant};

use io::parse_input;
use solver::{Solver, UNDEFINED_COORD};
use types::{Color, ColoredMatrix, PLAIN};

pub mod io;
pub mod solver;
pub mod types;

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args = raw_args[1..].to_vec();

    match args.len() {
        0 => dialog(),
        2 => check_optimization(args),
        1 => init(args.get(0).unwrap().to_owned(), false),
        _ => print_error_message(),
    }
}

fn dialog() {
    println!("Input file path: ");

    let mut path = String::new();
    stdin().read_line(&mut path).ok();
    path = path.trim_end().to_owned();

    let mut choice = String::new();
    let mut valid = false;
    let mut optimize = false;

    while !valid {
        println!("Use optimization ? (Y/n) ");
        stdin().read_line(&mut choice).ok();
        choice = choice.trim_end().to_owned();

        if choice == "Y" || choice == "y" {
            valid = true;
            optimize = true;
        } else if choice == "N" || choice == "n" {
            valid = true;
        }
    }

    init(path, optimize);
}

fn check_optimization(args: Vec<String>) {
    if args.get(1).unwrap() == "--optimize" {
        init(args.get(0).unwrap().to_owned(), true);
    } else {
        print_error_message();
    }
}

fn init(path: String, optimized: bool) {
    let raw_puzzle = fs::read_to_string(path.as_str()).expect("");
    let (puzzle, words) = parse_input(&raw_puzzle);

    let now = Instant::now();

    let mut solver = Solver::new(&puzzle);
    if optimized {
        solver.optimize()
    }

    let mut solution: ColoredMatrix<char> = ColoredMatrix::new(&puzzle);
    let mut comparisons = vec![0; words.len()];
    let mut colors = vec![PLAIN; words.len()];

    for i in 0..words.len() {
        let word = words.get(i).unwrap();
        let res = solver.search(&word.to_string());
        let color = Color::random();

        if res.found_at != UNDEFINED_COORD {
            solution.colorize(res.found_at, res.dir, word.len() as i32, color);
            colors[i] = color;
        }

        comparisons[i] = res.comparisons;
    }

    let exec_time = now.elapsed();

    println!("SOLUTION");
    println!("{}", solution);

    println!("SUMMARY");
    let mut total_comparisons = 0;
    for i in 0..words.len() {
        println!(
            "{} - {} comparisons{}",
            colors[i].wrap(&words[i].to_owned()),
            comparisons[i],
            if colors[i].is_plain() {
                "(not found)"
            } else {
                ""
            }
        );

        total_comparisons += comparisons[i];
    }

    println!("Total comparisons : {} comparisons", total_comparisons);
    println!("Execution time    : {} ms", exec_time.as_millis());
    println!(
        "                  : {} s",
        exec_time.as_millis() as f64 / 1e3
    );
}

fn print_error_message() -> ! {
    println!("\nUsage : [input-file-path [--optimize]]\n");
    println!("  --optimize\t\t\tOptimize solving algorithm using heuristic technique.");

    exit(1)
}
