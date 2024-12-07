use std::fs;

fn main() {
    // Read the input file
    let content = fs::read_to_string("input.txt").expect("Unable to read the file");

    // Initialize the two vectors
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    // Process each line
    for line in content.lines() {
        let mut split = line.split_whitespace();
        if let (Some(first), Some(second)) = (split.next(), split.next()) {
            first_column.push(
                first
                    .parse::<i32>()
                    .expect("Invalid number in first column"),
            );
            second_column.push(
                second
                    .parse::<i32>()
                    .expect("Invalid number in second column"),
            );
        }
    }

    // Sort the two vectors
    first_column.sort();
    second_column.sort();

    let mut total_dist = 0;

    for i in 0..first_column.len() {
        total_dist += (first_column[i] - second_column[i]).abs();
    }

    println!("Total distance: {}", total_dist);

    let mut simularity = 0;
    let mut local_simularity = 0;
    for num in first_column.iter() {
        for num2 in second_column.iter() {
            if num == num2 {
                local_simularity += num;
            }
        }
        simularity += local_simularity;
        local_simularity = 0;
    }

    println!("Simularity: {}", simularity);
}
