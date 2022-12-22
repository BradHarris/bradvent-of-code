use std::time::Duration;

pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
    print!("\x1b[{};{}H", 0, 0);
}

#[derive(Debug)]
pub struct DayPerfMetric {
    pub day: usize,
    pub part1: Duration,
    pub part2: Duration,
}

pub fn print_time_results(results: Vec<DayPerfMetric>, runs: usize) {
    println!("");
    println!("+{:-^38}+", format!("averaged over {runs} runs"));
    println!("| {: <6} | {: <12} | {: <12} |", "day", "part 1", "part 2");
    let total = results
        .iter()
        .inspect(|DayPerfMetric { day, part1, part2 }| {
            println!(
                "| {day: <6} | {: <12} | {: <12} |",
                format!("{part1:?}"),
                format!("{part2:?}")
            )
        })
        .map(|m| m.part1 + m.part2)
        .sum::<Duration>();
    println!("+{:-^38}+", format!("total: {total:?}"));
    println!("");
}
