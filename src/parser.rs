pub mod tokenizer {
    use core::fmt;
    use std::cmp::Ordering;

    use regex::{Match, Regex};

    pub type ASTNode = (Token, Box<AST>, Box<AST>);

    #[derive(Debug)]
    pub enum SyntaxError {
        InvalidFormula,
        InvalidFormulaBranch,
    }

    impl fmt::Display for SyntaxError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SyntaxError::InvalidFormula => write!(f, "Invalid formula"),
                SyntaxError::InvalidFormulaBranch => write!(f, "Invalid formula branch"),
            }
        }
    }

    impl std::error::Error for SyntaxError {}

    #[derive(Debug)]
    pub enum AST {
        ASTNode(ASTNode),
        Token(Token),
    }

    impl AST {
        pub fn to_ast(tokens: Vec<Token>) -> Result<AST, SyntaxError> {
            let mut stack: Vec<AST> = Vec::new();

            for token in tokens {
                if token.token_type.priority == 0 {
                    stack.push(AST::Token(token));
                    continue;
                }

                if stack.len() < 2 {
                    return Err(SyntaxError::InvalidFormulaBranch);
                }

                let node = (
                    token,
                    Box::new(stack.pop().unwrap()),
                    Box::new(stack.pop().unwrap()),
                );

                stack.push(AST::ASTNode(node));
            }

            if stack.len() > 1 {
                return Err(SyntaxError::InvalidFormula);
            }

            return Ok(stack.pop().unwrap());
        }

        pub fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
            let mut op_stack: Vec<Token> = Vec::new();
            let mut num_stack: Vec<Token> = Vec::new();

            for token in tokens {
                println!(
                    "num: {} op: {}",
                    AST::to_string(&num_stack),
                    AST::to_string(&op_stack)
                );

                if token.token_type.priority == 0 {
                    num_stack.push(token);
                    continue;
                }

                if token.token_type.is_context {
                    let context_match = token.token_type.context_match.clone();
                    while op_stack.len() > 0
                        && op_stack
                            .last()
                            .unwrap()
                            .token_type
                            .typename
                            .cmp(&context_match)
                            != Ordering::Equal
                    {
                        let t = op_stack.pop();
                        num_stack.push(t.unwrap().clone());
                    }

                    op_stack.pop();

                    continue;
                }

                if op_stack.last().is_some() && token.get_priority() > 1 {
                    if token.get_priority() < op_stack.last().unwrap().get_priority() {
                        while op_stack.len() > 0 {
                            let token = op_stack.pop();
                            num_stack.push(token.unwrap().clone());
                        }
                    }

                    if op_stack.len() > 0
                        && !token.is_right_assoc()
                        && token.get_priority() == op_stack.last().unwrap().get_priority()
                    {
                        while op_stack.len() > 0
                            && token.get_priority() == op_stack.last().unwrap().get_priority()
                        {
                            let t = op_stack.pop();
                            num_stack.push(t.unwrap().clone());
                        }
                    }
                }

                op_stack.push(token)
            }

            while op_stack.len() > 0 {
                let token = op_stack.pop();
                num_stack.push(token.unwrap().clone());
            }

            return num_stack;
        }

        pub fn to_string(tokens: &Vec<Token>) -> String {
            let mut str = String::new();

            for token in tokens {
                str.push_str(token.text.as_ref().unwrap().as_str());
            }

            str
        }
    }

    #[derive(Debug, Clone)]
    pub struct TokenType {
        pub typename: String,
        pub pattern: String,
        pub priority: i32,
        pub is_context: bool,
        pub is_function: bool,
        pub params: Vec<String>,
        pub context_match: String,
        pub assoc_right: bool,
    }

    impl TokenType {
        pub fn new(
            typename: String,
            pattern: String,
            priority: i32,
            assoc_right: bool,
            is_context: bool,
            context_match: Option<String>,
            is_function: bool,
            params: Option<Vec<String>>,
        ) -> TokenType {
            TokenType {
                typename: typename,
                pattern: pattern,
                priority: priority,
                assoc_right: assoc_right,
                is_context: is_context,
                is_function: is_function,
                params: params.unwrap_or(Vec::new()),
                context_match: context_match.unwrap_or(String::from("0")),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Token {
        pub token_type: TokenType,
        pub index: (usize, usize),
        pub text: Result<String, String>,
    }

    impl Token {
        pub fn new(token_type: TokenType, formula: String, m: Match) -> Token {
            let start: usize = m.start().try_into().unwrap();
            let end: usize = m.end().try_into().unwrap();
            let text = if start <= end
                && end <= formula.len()
                && formula.is_char_boundary(start)
                && formula.is_char_boundary(end)
            {
                Ok(String::from(&formula[start..end]))
            } else {
                Err(String::from("Error"))
            };

            return Token {
                token_type: token_type,
                index: (start, end),
                text: text,
            };
        }

        pub fn resolver(token_type: TokenType, formula: String) -> Vec<Token> {
            let r: Regex = Regex::new(token_type.pattern.as_str()).unwrap();
            return r
                .find_iter(&formula)
                .filter_map(|m| Some(Token::new(token_type.clone(), formula.clone(), m)))
                .collect();
        }

        pub fn get_priority(&self) -> i32 {
            return self.token_type.priority;
        }

        pub fn is_right_assoc(&self) -> bool {
            return self.token_type.assoc_right;
        }
    }
}
