use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub code: String,
    pub loc: Loc,
    pub buf: VecDeque<Token>,
    pub states: Vec<usize>,
    pub token_pos: usize,
    pub prev_token_pos: usize,
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
            token_pos: 0,
            prev_token_pos: 0,
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
            c if c != '<' => self.read_text(),
            c if c.is_whitespace() => {
                self.skip_whitespace()?;
                return self.tokenize();
            }
            '\n' => {
                assert_eq!(self.take_char()?, '\n');
                self.loc.line += 1;
                self.loc.column = 0;
                return Err(Error::END);
            }
            _ => self.read_tag(),
        }
    }

    fn read_tag(&mut self) -> Result<Token, Error> {
        assert_eq!(self.take_char()?, '<');
        let name = self.take_char_while(|c| c.is_alphanumeric())?;
        assert_eq!(self.take_char()?, '>');
        Ok(Token {
            kind: Kind::OpenTag(name),
            loc: self.loc,
        })
    }

    fn read_text(&mut self) -> Result<Token, Error> {
        let text = self.take_char_while(|c| match c {
            '0'..='9' => true,
            c if c.is_alphanumeric() => true,
            _ => false,
        })?;
        Ok(Token {
            kind: Kind::Text(text),
            loc: self.loc,
        })
    }
}

impl Lexer {
    fn peek_char(&self) -> Result<char, Error> {
        self.code[self.loc.pos..].chars().next().ok_or(Error::END)
    }

    fn skip_whitespace(&mut self) -> Result<(), Error> {
        return self.take_char_while(|c| c == ' ' || c == '\t').and(Ok(()));
    }

    fn take_char_while<F>(&mut self, mut f: F) -> Result<String, Error>
    where
        F: FnMut(char) -> bool,
    {
        let mut s = "".to_string();
        while !self.eof() && f(self.peek_char()?) {
            s.push(self.take_char()?);
        }
        Ok(s)
    }

    fn eof(&self) -> bool {
        self.loc.pos >= self.code.len()
    }

    fn take_char(&mut self) -> Result<char, Error> {
        let mut iter = self.code[self.loc.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(Error::END)?;
        let (next_pos, _) = iter.next().unwrap_or((cur_char.len_utf8(), ' '));
        self.loc.pos += next_pos;
        self.loc.column += next_pos;
        Ok(cur_char)
    }
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
