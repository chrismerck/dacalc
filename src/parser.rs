use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::iterators::Pairs;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;  

fn new() -> PrattParser<Rule> {
  PrattParser::new()
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
        .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
        .op(Op::infix(Rule::pow, Assoc::Right))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::fac))
}

fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i32 {
  pratt
      .map_primary(|primary| match primary.as_rule() {
          Rule::int  => primary.as_str().parse().unwrap(),
          Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
          _          => unreachable!(),
      })
      .map_prefix(|op, rhs| match op.as_rule() {
          Rule::neg  => -rhs,
          _          => unreachable!(),
      })
      .map_postfix(|lhs, op| match op.as_rule() {
          Rule::fac  => (1..lhs+1).product(),
          _          => unreachable!(),
      })
      .map_infix(|lhs, op, rhs| match op.as_rule() {
          Rule::add  => lhs + rhs,
          Rule::sub  => lhs - rhs,
          Rule::mul  => lhs * rhs,
          Rule::div  => lhs / rhs,
          Rule::pow  => (1..rhs+1).map(|_| lhs).product(),
          _          => unreachable!(),
      })
      .parse(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pratt = new();
        let expr = ExprParser::parse(Rule::expr, "4 + 3 * 2 ^ 3!").unwrap();
        assert_eq!(parse_expr(expr, &pratt), 4 + 3 * 2i32.pow(2 * 3));
    }
}