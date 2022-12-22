use std::{fmt::Display, slice::Iter};

pub enum RegularExpression {
    Epsilon,
    Symbol(char),
    Group(Box<RegularExpression>),
    Union(Box<RegularExpression>, Box<RegularExpression>),
    Concat(Box<RegularExpression>, Box<RegularExpression>),
    Closure(Box<RegularExpression>),
}

impl Display for RegularExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegularExpression::Epsilon => write!(f, "ε"),
            RegularExpression::Symbol(c) => write!(f, "{}", c),
            RegularExpression::Group(a) => write!(f, "({})", a),
            RegularExpression::Union(a, b) => write!(f, "[{}]+[{}]", *a, *b),
            RegularExpression::Concat(a, b) => write!(f, "[{}].[{}]", *a, *b),
            RegularExpression::Closure(a) => write!(f, "<{}>", *a),
        }
    }
}

enum Token {
    Symbol(char),
    Group(TokenGroup),
}

struct TokenGroup(Vec<Token>);

impl Display for TokenGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut first = true;
        for a in &self.0 {
            write!(f, "{}{}", if first { "" } else { "," }, a)?;
            first = false;
        }
        write!(f, ")")
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Symbol(c) => write!(f, "{}", c),
            Token::Group(v) => write!(f, "{}", v),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Self::Symbol(arg0) => Self::Symbol(arg0.clone()),
            Self::Group(arg0) => Self::Group(TokenGroup(arg0.0.to_vec())),
        }
    }
}

fn parse_group(iter: &mut std::str::Chars) -> TokenGroup {
    let mut tokens = TokenGroup(vec![]);
    while let Some(c) = iter.next() {
        tokens.0.push(match c {
            '(' => Token::Group(parse_group(iter)),
            ')' => return tokens,
            token => Token::Symbol(token),
        })
    }
    tokens
}

fn optimize_group(group: &mut TokenGroup) {
    for token in &mut group.0 {
        if let Token::Group(other) = token {
            optimize_group(other)
        }
    }
    if group.0.len() == 1 {
        match group.0.get(0) {
            Some(first) => match first {
                Token::Group(inner) => group.0 = inner.0.to_vec(),
                _ => {}
            },
            None => todo!(),
        }
    }
}

fn build_expression(iter: &mut Iter<Token>) -> RegularExpression {
    let mut exp = RegularExpression::Epsilon;
    while let Some(t) = iter.next() {
        match t {
            Token::Symbol(c) => match c {
                '+' => {
                    exp = RegularExpression::Union(Box::new(exp), Box::new(build_expression(iter)));
                }
                '*' => {
                    exp = match exp {
                        RegularExpression::Union(left, right) => RegularExpression::Union(
                            left,
                            Box::new(RegularExpression::Closure(right)),
                        ),
                        RegularExpression::Concat(left, right) => RegularExpression::Concat(
                            left,
                            Box::new(RegularExpression::Closure(right)),
                        ),
                        _ => RegularExpression::Closure(Box::new(exp)),
                    }
                }
                symbol => {
                    exp = match exp {
                        RegularExpression::Epsilon => RegularExpression::Symbol(*symbol),
                        _ => RegularExpression::Concat(
                            Box::new(exp),
                            Box::new(RegularExpression::Symbol(*symbol)),
                        ),
                    }
                }
            },
            Token::Group(g) => {
                exp = match exp {
                    RegularExpression::Epsilon => {
                        RegularExpression::Group(Box::new(build_expression(&mut g.0.iter())))
                    }
                    _ => RegularExpression::Concat(
                        Box::new(exp),
                        Box::new(RegularExpression::Group(Box::new(build_expression(
                            &mut g.0.iter(),
                        )))),
                    ),
                }
            }
        }
    }
    exp
}

pub fn parse(input: &String) -> RegularExpression {
    let mut tokens = parse_group(&mut input.chars());
    optimize_group(&mut tokens);
    let mut iter = tokens.0.iter();
    build_expression(&mut iter)
}
