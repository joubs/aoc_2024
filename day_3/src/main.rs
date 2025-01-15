use nom::{
  branch::alt, bytes::complete::tag, character::complete::{anychar, u32 as parse_u32}, combinator::{self, value}, multi::{many1, many_till}, sequence::{delimited, separated_pair}, IResult
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instruction{
  Mul(u32,u32),
  Enable,
  Disable
}

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
  alt((parse_do, parse_dont, parse_mul_op))(input)
}

pub fn parse_do(input: &str) -> IResult<&str, Instruction> {
  value(Instruction::Enable, tag("do()"))(input)
}

pub fn parse_dont(input: &str) -> IResult<&str, Instruction> {
  value(Instruction::Disable, tag("don't()"))(input)
}

pub fn parse_mul_op(input: &str) -> IResult<&str, Instruction> {
  let (remain, num_pair) = delimited(
    tag("mul("), 
    separated_pair(parse_u32, tag(","), parse_u32),
    tag(")")
  )(input)?;
  Ok((remain, Instruction::Mul(num_pair.0, num_pair.1)))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(
      combinator::map(many_till(anychar, parse_instruction),|(_garbage, inst)| inst)
    )(input)

}

fn main() -> anyhow::Result<()> {
  let (_, instructions) = parse_input(include_str!("input.txt"))?;
  
  let result: u32 = instructions
    .iter()
    .map(|instruction| match instruction {
      Instruction::Mul(a,b ) => a * b,
      _ => 0
    })
    .sum();
  
  println!("Sum of mul operations: {}", result);

  // Part 2 
  let mut mul_enabled = true;
  let enabled_mul_sum: u32 = instructions
    .iter()
    .map(|instruction| match instruction {
      Instruction::Mul(a,b ) => match mul_enabled {
        true => a*b,
        false => 0 
      },
      Instruction::Enable => {
        mul_enabled = true;
        0
      },
      Instruction::Disable => {
        mul_enabled = false;
        0
      }

    })
    .sum();

  println!("Enabled mul sum: {}", enabled_mul_sum);

  Ok(())
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn check_mul_parser() {
    assert_eq!(parse_mul_op("mul(4,456)"), Ok(("", Instruction::Mul(4u32, 456u32))));

    assert_eq!(parse_do("do()abc"), Ok(("abc", Instruction::Enable)));
    assert_eq!(parse_instruction("mul(4,5)"), Ok(("", Instruction::Mul(4, 5))));
    assert_eq!(parse_instruction("don't()toto"), Ok(("toto", Instruction::Disable)));

    assert_eq!(parse_input("?select()@ )select()>,how()mul(627,742)<??$"), Ok(("<??$",vec![Instruction::Mul(627u32, 742u32)])));
  }
}
