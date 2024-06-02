use parser::tokenizer::{Token, TokenType, AST};

pub mod parser;

fn main() {
    let formula = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let token_types = [
        TokenType::new(
            String::from("number"),
            String::from(r"-?\d+(\.\d+)?"),
            0,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("sum_operator"),
            String::from(r"\+"),
            2,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("sub_operator"),
            String::from(r"\-"),
            2,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("mul_operator"),
            String::from(r"\*"),
            3,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("div_operator"),
            String::from(r"\/"),
            3,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("power_operator"),
            String::from(r"\^"),
            4,
            true,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("parenthesis_left"),
            String::from(r"\("),
            1,
            false,
            false,
            None,
            false,
            None,
        ),
        TokenType::new(
            String::from("parenthesis_right"),
            String::from(r"\)"),
            1,
            false,
            true,
            Some(String::from("parenthesis_left")),
            false,
            None,
        ),
        TokenType::new(
            String::from("max_fn"),
            String::from(r"max"),
            1,
            false,
            false,
            None,
            true,
            vec![String::from("number"), String::from("number")].into(),
        ),
    ];

    let mut tokens: Vec<Token> = token_types
        .into_iter()
        .map(|token_type| {
            return Token::resolver(token_type, String::from(formula));
        })
        .flatten()
        .collect();

    tokens.sort_by(|a: &Token, b: &Token| a.index.0.cmp(&b.index.0));

    // print!("Numbers: {:#?}", tokens);

    let rpn = AST::to_rpn(tokens);

    print!("\n Formula: {} RPN: {:#?}\n", formula, AST::to_string(&rpn));

    // let ast = AST::to_ast(rpn);

    // match ast {
    //     Ok(ast) => println!("AST #{:#?}", ast),
    //     Err(err) => println!("{}", err),
    // }
}
