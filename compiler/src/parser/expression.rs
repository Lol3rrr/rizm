use std::iter::Peekable;

use super::call_params;
use crate::{
    ir,
    lexer::{Token, Value},
};

fn parse_single<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::Constant(const_val)) => {
            iter.next().unwrap();
            match const_val {
                Value::Integer(value) => Some(ir::Expression::Constant(ir::Value::I32(*value))),
                Value::UInteger(value) => Some(ir::Expression::Constant(ir::Value::U32(*value))),
            }
        }
        Some(Token::Identifier(name)) => {
            iter.next().unwrap();
            match iter.peek() {
                Some(Token::OpenParan) => {
                    iter.next();

                    let params = call_params::parse(iter)?;
                    Some(ir::Expression::Call(name.to_owned(), params))
                }
                _ => Some(ir::Expression::Variable(name.to_owned())),
            }
        }
        _ => None,
    }
}

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<ir::Expression>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek() {
        Some(Token::OpenParan) => {
            iter.next();
            let inner = parse(iter);

            match iter.next() {
                Some(Token::CloseParan) => {}
                _ => return None,
            };

            inner
        }
        Some(Token::Constant(_)) | Some(Token::Identifier(_)) => {
            let left_side = parse_single(iter)?;

            match iter.peek() {
                Some(Token::Plus)
                | Some(Token::Minus)
                | Some(Token::Asterisk)
                | Some(Token::Slash) => {
                    let operation = match iter.next() {
                        Some(Token::Plus) => ir::OP::Add,
                        Some(Token::Minus) => ir::OP::Substract,
                        Some(Token::Asterisk) => ir::OP::Multiply,
                        Some(Token::Slash) => ir::OP::Divide,
                        _ => return None,
                    };

                    let right_side = match parse(iter) {
                        Some(r) => r,
                        None => return None,
                    };

                    match (&operation, right_side) {
                        (ir::OP::Multiply, ir::Expression::Operation(other_op, mut additions))
                        | (ir::OP::Divide, ir::Expression::Operation(other_op, mut additions)) => {
                            let right_left = additions.remove(0);

                            let exp =
                                ir::Expression::Operation(operation, vec![left_side, right_left]);

                            Some(ir::Expression::Operation(
                                other_op,
                                vec![exp, additions.remove(0)],
                            ))
                        }
                        (_, right) => {
                            Some(ir::Expression::Operation(operation, vec![left_side, right]))
                        }
                    }
                }
                _ => Some(left_side),
            }
        }
        Some(Token::And) => {
            iter.next().unwrap();

            let var_name = match iter.peek() {
                Some(Token::Identifier(name)) => {
                    iter.next();
                    name.to_string()
                }
                _ => return None,
            };

            Some(ir::Expression::Reference(var_name))
        }
        Some(Token::Asterisk) => {
            iter.next().unwrap();

            let inner = parse(iter).unwrap();

            Some(ir::Expression::Dereference(Box::new(inner)))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant() {
        let tokens = &[Token::Constant(Value::Integer(2))];

        let expected = Some(ir::Expression::Constant(ir::Value::I32(2)));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable() {
        let tokens = &[Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Variable("test".to_string()));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn constant_plus_constant() {
        let tokens = &[
            Token::Constant(Value::Integer(2)),
            Token::Plus,
            Token::Constant(Value::Integer(3)),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Constant(ir::Value::I32(2)),
                ir::Expression::Constant(ir::Value::I32(3)),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_plus_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Plus,
            Token::Identifier("test_2".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Variable("test_1".to_string()),
                ir::Expression::Variable("test_2".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_multiply_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Asterisk,
            Token::Identifier("test_2".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Multiply,
            vec![
                ir::Expression::Variable("test_1".to_string()),
                ir::Expression::Variable("test_2".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_multiply_variable_plus_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Asterisk,
            Token::Identifier("test_2".to_string()),
            Token::Plus,
            Token::Identifier("test_3".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Operation(
                    ir::OP::Multiply,
                    vec![
                        ir::Expression::Variable("test_1".to_string()),
                        ir::Expression::Variable("test_2".to_string()),
                    ],
                ),
                ir::Expression::Variable("test_3".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_divide_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Slash,
            Token::Identifier("test_2".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Divide,
            vec![
                ir::Expression::Variable("test_1".to_string()),
                ir::Expression::Variable("test_2".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn variable_divide_variable_plus_variable() {
        let tokens = &[
            Token::Identifier("test_1".to_string()),
            Token::Slash,
            Token::Identifier("test_2".to_string()),
            Token::Plus,
            Token::Identifier("test_3".to_string()),
        ];

        let expected = Some(ir::Expression::Operation(
            ir::OP::Add,
            vec![
                ir::Expression::Operation(
                    ir::OP::Divide,
                    vec![
                        ir::Expression::Variable("test_1".to_string()),
                        ir::Expression::Variable("test_2".to_string()),
                    ],
                ),
                ir::Expression::Variable("test_3".to_string()),
            ],
        ));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn reference_to_variable() {
        let tokens = &[Token::And, Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Reference("test".to_string()));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn dereference_variable() {
        let tokens = &[Token::Asterisk, Token::Identifier("test".to_string())];

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Variable("test".to_string()),
        )));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn dereference_constant() {
        let tokens = &[Token::Asterisk, Token::Constant(Value::Integer(0))];

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Constant(ir::Value::I32(0)),
        )));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
    #[test]
    fn dereference_calc() {
        let tokens = &[
            Token::Asterisk,
            Token::OpenParan,
            Token::Constant(Value::Integer(1)),
            Token::Plus,
            Token::Constant(Value::Integer(2)),
            Token::CloseParan,
        ];

        let expected = Some(ir::Expression::Dereference(Box::new(
            ir::Expression::Operation(
                ir::OP::Add,
                vec![
                    ir::Expression::Constant(ir::Value::I32(1)),
                    ir::Expression::Constant(ir::Value::I32(2)),
                ],
            ),
        )));

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}