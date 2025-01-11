

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
      .zip(right)
      .map(|(a,b)| a.abs_diff(b))
      .sum();
    
    println!("Sum of distances: {}", result);
    Ok(())
}
