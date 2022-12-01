use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let dataset = fs::read_to_string("datasets/day1.txt")?;

    let mut inventories = dataset
        .split("\n\n")
        .map(|inv| {
            inv.split('\n')
                .filter_map(|line| line.trim().parse::<u64>().ok())
                .sum()
        })
        .collect::<Vec<_>>();

    inventories.sort();

    let top1 = inventories.last().unwrap();
    let top3: u64 = inventories.iter().rev().take(3).sum();

    println!("Top 1: {top1}");
    println!("Top 3: {top3}");

    Ok(())
}
