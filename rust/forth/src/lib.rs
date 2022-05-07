use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type ParseTree = Vec<Node>;
type ParseResult = std::result::Result<ParseTree, Error>;
type PopResult = std::result::Result<Value, Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Rc<ParseTree>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
enum Token {
    Number(Value),
    Word(String),
    DefinitionBegin,
    DefinitionEnd,
}

#[derive(Debug, Clone)]
enum Node {
    Number(Value),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Compound(Rc<ParseTree>),
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            words: [
                ("+".to_string(), Rc::new(vec![Node::Add])),
                ("-".to_string(), Rc::new(vec![Node::Sub])),
                ("*".to_string(), Rc::new(vec![Node::Mul])),
                ("/".to_string(), Rc::new(vec![Node::Div])),
                ("DUP".to_string(), Rc::new(vec![Node::Dup])),
                ("DROP".to_string(), Rc::new(vec![Node::Drop])),
                ("SWAP".to_string(), Rc::new(vec![Node::Swap])),
                ("OVER".to_string(), Rc::new(vec![Node::Over])),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let tokens =
            input
                .split_whitespace()
                .map(|token| match token.to_ascii_uppercase().as_str() {
                    ":" => Token::DefinitionBegin,
                    ";" => Token::DefinitionEnd,
                    token => token
                        .parse::<Value>()
                        .map_or_else(|_| Token::Word(token.to_string()), Token::Number),
                });

        let parse_tree = self.parse(tokens)?;
        self.interpret(&parse_tree)
    }

    fn parse(&mut self, mut tokens: impl Iterator<Item = Token>) -> ParseResult {
        let mut tree = Vec::new();
        while let Some(token) = tokens.next() {
            match token {
                Token::DefinitionBegin => self.parse_definition(&mut tokens)?,
                Token::DefinitionEnd => return Err(Error::InvalidWord),
                Token::Number(value) => tree.push(Node::Number(value)),
                Token::Word(word) => {
                    let subtree = self.words.get(&word).ok_or(Error::UnknownWord)?;
                    tree.push(Node::Compound(subtree.clone()));
                }
            }
        }
        Ok(tree)
    }

    fn parse_definition(&mut self, tokens: &mut impl Iterator<Item = Token>) -> Result {
        let mut definition_complete = false;
        if let Some(Token::Word(new_word)) = tokens.next() {
            let mut tree = Vec::new();
            while let Some(token) = tokens.next() {
                match token {
                    Token::DefinitionBegin => return Err(Error::InvalidWord),
                    Token::DefinitionEnd => {
                        self.words.insert(new_word, Rc::new(tree));
                        definition_complete = true;
                        break;
                    }
                    Token::Number(value) => tree.push(Node::Number(value)),
                    Token::Word(word) => {
                        let subtree = self.words.get(&word).ok_or(Error::UnknownWord)?;
                        tree.push(Node::Compound(subtree.clone()));
                    }
                }
            }
        }
        if !definition_complete {
            return Err(Error::InvalidWord);
        }
        return Ok(());
    }

    fn interpret(&mut self, parse_tree: &[Node]) -> Result {
        let mut stack = Vec::new();
        for node in parse_tree {
            stack.push(node.clone());
            while let Some(node) = stack.pop() {
                match node {
                    Node::Number(value) => self.push(value),
                    Node::Add => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        self.push(value1 + value2);
                    }
                    Node::Sub => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        self.push(value1 - value2);
                    }
                    Node::Mul => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        self.push(value1 * value2);
                    }
                    Node::Div => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        if value2 == 0 {
                            return Err(Error::DivisionByZero);
                        }
                        self.push(value1 / value2);
                    }
                    Node::Dup => {
                        let value = self.pop()?;
                        self.push(value);
                        self.push(value);
                    }
                    Node::Drop => {
                        self.pop()?;
                    }
                    Node::Swap => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        self.push(value2);
                        self.push(value1);
                    }
                    Node::Over => {
                        let value2 = self.pop()?;
                        let value1 = self.pop()?;
                        self.push(value1);
                        self.push(value2);
                        self.push(value1);
                    }
                    Node::Compound(subtree) => {
                        for node in subtree.iter().rev() {
                            stack.push(node.clone());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> PopResult {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
}
