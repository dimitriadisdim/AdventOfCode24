use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    // Replace this with the path to your input file
    let input_file = "input.txt";

    // Read the input data
    let reports = read_lines(input_file).expect("Failed to read input file");

    // Collect and count the safe reports
    let safe_reports_count = reports
        .into_iter()
        .filter_map(|line| line.ok())
        .filter(|line| is_safe(line).0)
        .count();

    println!("Number of safe reports: {}", safe_reports_count);
}

fn is_safe(report: &str) -> (bool, String) {
    // Parse the levels into a vector of integers
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();

    if levels.len() < 2 {
        return (false, "Report has less than 2 levels".to_string());
    }

    // Check if the levels are either all increasing or all decreasing
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..levels.len() {
        let diff = (levels[i] - levels[i - 1]).abs();

        if diff < 1 || diff > 3 {
            return (false, format!("Difference {} is not between 1 and 3", diff));
        }
        
        // We do it again without abs to check if we decrease or increase
        let diff = levels[i] - levels[i - 1];
        if diff < 0 {
            increasing = false;
        } else if diff > 0 {
            decreasing = false;
        }
    }

    if increasing || decreasing {
        (true, String::new())
    } else {
        (false, "Levels are neither all increasing nor all decreasing".to_string())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}