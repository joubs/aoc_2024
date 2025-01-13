use std::collections::HashMap;



fn main() -> anyhow::Result<()> {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = include_str!("input.txt")
      .lines()
      .map(|line| {
        let mut nums = line
          .split_whitespace()
          .map(str::parse::<u32>)
          .map(Result::unwrap);
        
        (nums.next().unwrap(), nums.next().unwrap()) 
      })
      .unzip();
    
    left.sort();
    right.sort();

    let result: u32 = left
      .iter()
      .zip(&right)
      .map(|(a,b)| a.abs_diff(*b))
      .sum();
    
    println!("Sum of distances: {}", result);

    // Part 2 

    let frequencies: HashMap<_,_> = right
      .chunk_by(|a,b| a == b)
      .map(|group| (group[0], group.len() as u32))
      .collect();

    let similarity_score: u32 = left
      .iter()
      .filter_map(|a| frequencies.get(a)
        .map(|freq| a * freq))
      .sum();
    println!("Similarity score: {}", similarity_score);

    Ok(())

}
