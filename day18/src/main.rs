use std::fs;

fn main() {
    let contents = fs::read_to_string(
        "/mnt/c/Users/predrag/Dropbox/Documents/Code/advent-of-code-2020/day18/input.txt",
    )
    .unwrap();

    let lines: Vec<_> = contents.trim().split("\n").collect();

    println!("{}", solve_part1(&lines));
    println!("{}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<&str>) -> i64 {
    lines
        .iter()
        .cloned()
        .map(parse_expression_part1)
        .map(|x| evaluate_expression(&x))
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Numeric(i64),
    Operator(Operator),
    OpenParen,
    CloseParen,
}

#[derive(Clone, Debug, PartialEq)]
enum Expression {
    Numeric(i64),
    Compound(Box<Expression>, Operator, Box<Expression>),
    Paren(Box<Expression>),
}

fn tokenize_expression(expr: &str) -> Vec<Token> {
    expr.split_ascii_whitespace()
        .flat_map(|x| -> Vec<&str> {
            let mut result: Vec<&str> = Vec::new();
            let mut rest_x = x;
            while rest_x.starts_with("(") {
                let (part, rest) = rest_x.split_at(1);
                result.push(part);
                rest_x = rest;
            }
            result.push(rest_x);
            result
        })
        .flat_map(|x| -> Vec<&str> {
            let mut result: Vec<&str> = Vec::new();
            let mut rest_x = x;
            while rest_x.ends_with(")") {
                let (rest, part) = rest_x.split_at(rest_x.len() - 1);
                result.push(part);
                rest_x = rest;
            }
            result.push(rest_x);
            result.reverse();
            result
        })
        .map(str::trim)
        .map(|x| match x {
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            "+" => Token::Operator(Operator::Add),
            "*" => Token::Operator(Operator::Multiply),
            x => Token::Numeric(x.parse().unwrap()),
        })
        .collect()
}

fn parse_expression_part1(expr: &str) -> Expression {
    parse_tokens_part1(tokenize_expression(expr).as_slice())
}

fn parse_tokens_part1(tokens: &[Token]) -> Expression {
    let (mut result, mut remainder_tokens) = parse_from_expression_start_tokens_part1(tokens);

    while !remainder_tokens.is_empty() {
        let operator = match remainder_tokens.first().unwrap() {
            Token::Operator(op) => op.clone(),
            _ => unreachable!(),
        };
        remainder_tokens = &remainder_tokens[1..remainder_tokens.len()];

        let (next_expression, post_expression_tokens) =
            parse_from_expression_start_tokens_part1(remainder_tokens);
        remainder_tokens = post_expression_tokens;
        result = Expression::Compound(Box::from(result), operator, Box::from(next_expression));
    }

    result
}

fn parse_from_expression_start_tokens_part1(tokens: &[Token]) -> (Expression, &[Token]) {
    match tokens.first().unwrap() {
        Token::OpenParen => {
            let (contents_tokens, remainder_tokens) = find_paren_expr(tokens);
            (
                Expression::Paren(Box::from(parse_tokens_part1(contents_tokens))),
                remainder_tokens,
            )
        }
        Token::Numeric(value) => {
            let remainder_tokens = &tokens[1..tokens.len()];
            (Expression::Numeric(*value), remainder_tokens)
        }
        _ => unreachable!(),
    }
}

fn parse_expression_part2(expr: &str) -> Expression {
    parse_tokens_part2(tokenize_expression(expr).as_slice())
}

fn parse_tokens_part2(tokens: &[Token]) -> Expression {
    let (mut result, mut remainder_tokens) = parse_from_expression_start_tokens_part2(tokens);

    while !remainder_tokens.is_empty() {
        let operator = match remainder_tokens.first().unwrap() {
            Token::Operator(op) => op.clone(),
            _ => unreachable!(),
        };
        remainder_tokens = &remainder_tokens[1..remainder_tokens.len()];

        let (next_result, next_remainder) =
            handle_operator_precedence(result, operator, remainder_tokens);
        result = next_result;
        remainder_tokens = next_remainder;
    }

    result
}

fn handle_operator_precedence(
    pre_operator_expr: Expression,
    op: Operator,
    post_operator_tokens: &[Token],
) -> (Expression, &[Token]) {
    let (mut post_operator_expr, mut remaining_tokens) =
        parse_from_expression_start_tokens_part2(post_operator_tokens);
    let (next_expr, next_tokens) = match op {
        Operator::Add => (post_operator_expr, remaining_tokens),
        Operator::Multiply => {
            // coalesce right first, if possible
            match remaining_tokens.first() {
                None => (post_operator_expr, remaining_tokens),
                Some(token) => {
                    let mut op_token = token;
                    loop {
                        let next_operator = match op_token {
                            Token::Operator(oper) => oper.clone(),
                            _ => unreachable!(),
                        };

                        let (next_expression, rest_tokens) = handle_operator_precedence(
                            post_operator_expr,
                            next_operator,
                            &remaining_tokens[1..remaining_tokens.len()],
                        );
                        post_operator_expr = next_expression;
                        remaining_tokens = rest_tokens;
                        if remaining_tokens
                            .first()
                            .map_or(true, |&x| x != Token::Operator(Operator::Add))
                        {
                            break;
                        }
                        op_token = remaining_tokens.first().unwrap();
                    }

                    (post_operator_expr, remaining_tokens)
                }
            }
        }
    };

    (
        Expression::Compound(Box::from(pre_operator_expr), op, Box::from(next_expr)),
        next_tokens,
    )
}

fn parse_from_expression_start_tokens_part2(tokens: &[Token]) -> (Expression, &[Token]) {
    match tokens.first().unwrap() {
        Token::OpenParen => {
            let (contents_tokens, remainder_tokens) = find_paren_expr(tokens);
            (
                Expression::Paren(Box::from(parse_tokens_part2(contents_tokens))),
                remainder_tokens,
            )
        }
        Token::Numeric(value) => {
            let remainder_tokens = &tokens[1..tokens.len()];
            (Expression::Numeric(*value), remainder_tokens)
        }
        _ => unreachable!(),
    }
}

fn find_paren_expr(tokens: &[Token]) -> (&[Token], &[Token]) {
    let mut in_parens = 1;
    let tokens_minus_paren = &tokens[1..tokens.len()];
    for (sub_index, &token) in tokens_minus_paren.iter().enumerate() {
        in_parens += match token {
            Token::OpenParen => 1,
            Token::CloseParen => -1,
            _ => 0,
        };
        if in_parens == 0 {
            let contents_tokens = &tokens_minus_paren[0..sub_index];
            let remainder_tokens = &tokens_minus_paren[sub_index + 1..tokens_minus_paren.len()];

            return (contents_tokens, remainder_tokens);
        }
    }

    unreachable!();
}

fn evaluate_expression(expr: &Expression) -> i64 {
    match expr {
        Expression::Numeric(value) => *value,
        Expression::Paren(inner) => evaluate_expression(inner),
        Expression::Compound(left, operator, right) => {
            let left_value = evaluate_expression(left);
            let right_value = evaluate_expression(right);
            match operator {
                Operator::Add => left_value + right_value,
                Operator::Multiply => left_value * right_value,
            }
        }
    }
}

fn solve_part2(lines: &Vec<&str>) -> i64 {
    lines
        .iter()
        .cloned()
        .map(parse_expression_part2)
        .map(|x| evaluate_expression(&x))
        .sum()
}
