use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    pub code: String,
    pub loc: Loc,
    pub buf: Vec<Token>,
    pub pos: usize,
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
    pub attributes: Option<Vec<Token>>,
    pub loc: Loc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    OpenTag(String),
    CloseTag(String),
    SelfCloseTag(String),
    Attribute(String, String),
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
            buf: vec![],
            pos: 0,
        }
    }

    pub fn tokenize_all(&mut self) -> Result<(), Error> {
        loop {
            match self.tokenize() {
                Ok(tok) => self.buf.push(tok),
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
            c if c.is_whitespace() => {
                self.skip_whitespace()?;
                return self.tokenize();
            }
            '\n' => {
                self.loc.line += 1;
                self.loc.column = 0;
                return Err(Error::END);
            }
            c if c != '<' => self.read_text(),
            _ => self.read_tag(),
        }
    }

    fn read_tag(&mut self) -> Result<Token, Error> {
        assert_eq!(self.take_char()?, '<');

        let close_start = self.peek_char()? == '/';

        if close_start {
            assert_eq!(self.take_char()?, '/');
        }
        let name = self.take_char_while(|c| c.is_alphanumeric())?;

        let close_end = self.peek_char()? == '/';

        if close_end {
            assert_eq!(self.take_char()?, '/');
        }

        let attributes = self.read_attributes()?;

        assert_eq!(self.take_char()?, '>');

        if close_start {
            Ok(Token {
                kind: Kind::CloseTag(name),
                attributes: None,
                loc: self.loc,
            })
        } else if close_end {
            Ok(Token {
                kind: Kind::SelfCloseTag(name),
                attributes: None,
                loc: self.loc,
            })
        } else {
            Ok(Token {
                kind: Kind::OpenTag(name),
                attributes: Some(attributes),
                loc: self.loc,
            })
        }
    }

    fn read_attributes(&mut self) -> Result<Vec<Token>, Error> {
        let mut out = vec![];

        loop {
            let char = self.peek_char()?;
            let next_char = self.peek_chars(1)?;

            if char == '>'|| (char == '/' && next_char == '>'){
                break;
            }
            if char.is_whitespace() {
                self.take_char()?;
            } else {
                let name = self.take_char_while(|c| c != '=')?;

                assert_eq!(self.take_char()?, '=');

                let quote = self.take_char()?;

                let quote_type = if quote == '\"' { '\"' } else { '\'' };

                let value = self.take_char_while(|c| c != quote_type)?;

                self.take_char()?;

                let trpl = Token {
                    kind: Kind::Attribute(name, value),
                    loc: self.loc,
                    attributes: None,
                };

                out.push(trpl)
            }
        }

        Ok(out)
    }

    fn read_text(&mut self) -> Result<Token, Error> {
        let text = self.take_char_while(|c| match c {
            '0'..='9' => true,
            c if c.is_alphanumeric() => true,
            _ => false,
        })?;
        Ok(Token {
            kind: Kind::Text(text),
            attributes: None,
            loc: self.loc,
        })
    }
}

impl Lexer {
    fn peek_char(&self) -> Result<char, Error> {
        self.code[self.loc.pos..].chars().next().ok_or(Error::END)
    }

    fn peek_chars(&self, index: usize) -> Result<char, Error> {
        let chars = self.code[self.loc.pos..].chars().collect::<Vec<char>>();
        let char = chars.get(index);
        match char {
            Some(c) => Ok(*c),
            None => Ok(' '),
        }
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
