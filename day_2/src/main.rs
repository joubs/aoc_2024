
fn gradual_diff(left: u32, right: u32) -> bool {
  let diff = left.abs_diff(right);
  1 <= diff && diff <= 3
}

fn is_safe(report: &Vec<u32>) -> bool {
  // Todo: what if len < 2? 
  let gradually_increasing = report.windows(2).all(|pair| pair[0] < pair[1] && gradual_diff(pair[0], pair[1]));
  let gradually_decreasing = report.windows(2).all(|pair| pair[0] > pair[1] && gradual_diff(pair[0], pair[1]));
  gradually_decreasing || gradually_increasing
}

fn main() -> anyhow::Result<()>{
    
    let num_safe_reports = include_str!("input.txt")
      .lines()
      .map(|line| line
          .split_whitespace()
          .map(str::parse::<u32>)
          .map(Result::unwrap)
          .collect::<Vec<u32>>()
        )
      .map(|report: Vec<u32>| {
        if is_safe(&report) {
          true
        } else {
          // Very inefficient, but convenient.. 
          for skip_index in 0..report.len() {
            let modified_report: Vec<u32> = report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_index)
                .map(|(_, &v)| v)
                .collect();
            if is_safe(&modified_report) {
                return true;
            }
          }
          false
        }

      })
      .filter(|&x| x)
      .count();
    println!("Number of safe report: {}", num_safe_reports);

    Ok(())
}
