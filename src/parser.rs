use crate::lexer::{Error, Kind, Lexer, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Parser {
    pub lexer: Lexer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub token: Token,
    pub children: Option<Vec<Node>>,
}

impl Parser {
    pub fn new(code: &str) -> Parser {
        Parser {
            lexer: Lexer::new(code.to_string()),
        }
    }

    pub fn parse_all(&mut self) -> Result<Node, Error> {
        self.lexer.tokenize_all()?;
        self.read_node()
    }

    pub fn read_node(&mut self) -> Result<Node, Error> {
        return self.read_child();
    }

    pub fn read_child(&mut self) -> Result<Node, Error> {
        let current = self.read_token()?;
        let mut children = vec![];

        match &current.kind {
            Kind::OpenTag(_) => {
                loop {
                    let next = self.peek_token(0)?;
                    if let Kind::CloseTag(_) = next.kind {
                        self.read_child()?;
                        break;
                    } else {
                        let node = self.read_child()?;
                        children.push(node);
                    }
                }
                return Ok(Node {
                    token: current,
                    children: Some(children),
                });
            }
            Kind::CloseTag(_) => {
                return Ok(Node {
                    token: current,
                    children: None,
                });
            }
            Kind::SelfCloseTag(_) => {
                return Ok(Node {
                    token: current,
                    children: None,
                });
            }
            Kind::Text(_) => {
                return Ok(Node {
                    token: current,
                    children: None,
                })
            }
            _ => Err(Error::END),
        }
    }
}

impl Parser {
    pub fn read_token(&mut self) -> Result<Token, Error> {
        if self.lexer.pos < self.lexer.buf.len() {
            let pos = self.lexer.pos;
            self.lexer.pos += 1;
            Ok(self.lexer.buf[pos].clone())
        } else {
            Err(Error::END)
        }
    }

    pub fn peek_token(&mut self, index: usize) -> Result<Token, Error> {
        let index_in_buf = self.lexer.pos + index;
        if index_in_buf < self.lexer.buf.len() {
            Ok(self.lexer.buf[index_in_buf].clone())
        } else {
            Err(Error::END)
        }
    }
}
