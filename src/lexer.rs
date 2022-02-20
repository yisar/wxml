use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub code: String,
    pub loc: Loc,
    pub buf: VecDeque<Token>,
    pub states: Vec<usize>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    END,
    ERR,
    Expect(Loc, String),
    UnexpectedToken(Loc, String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    OpenTag(String),
    CloseTag(String),
    Text(String),
    END,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
    pub pos: usize,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code,
            loc: Loc::default(),
            buf: VecDeque::new(),
            states: vec![],
        }
    }

    pub fn tokenize_all(&mut self) -> Result<(), Error> {
        loop {
            match self.tokenize() {
                Ok(tok) => self.buf.push_back(tok),
                Err(Error::END) => break,
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}

impl Lexer {
    pub fn tokenize(&mut self) -> Result<Token, Error> {
        let current = self.peek_char()?;
        match current {
            c if c !== '<' => self.read_text(),
            c if c.is_whitespace() => {
                self.skip_whitespace()?;
                return self.tokenize();
            }
            '\n' => self.read_end(),
            _ => self.read_tag(),
        }
    }

    fn read_tag(&mut self) -> Result<Token, Error> {
        return Token{
            kind:OpenTag('div'),
            Loc:loc::new(0,0,0)
        }
    }

    fn read_text(&mut self) -> Result<Token, Error> {
        return Token{
            kind:Text('123'),
            Loc:loc::new(0,0,0)
        }
    }

    fn read_end(&mut self) -> Result<Token, Error> {
        return Token{
            kind:Text('123'),
            Loc:loc::new(0,0,0)
        }
    }
}

impl Lexer {
    fn peek_char(&self) -> Result<char, Error> {
        self.code[self.loc.pos..]
            .chars()
            .next()
            .ok_or(Error::NormalEOF)
    }

    fn 
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            line: 1,
            column: 0,
            pos: 0,
        }
    }
}

impl Loc {
    pub fn new(line: usize, column: usize, pos: usize) -> Self {
        Self { line, column, pos }
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Loc(line:{},column:{},pos:{})",
            self.line, self.column, self.pos
        )
    }
}
