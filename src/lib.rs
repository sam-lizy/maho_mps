#[derive(Debug)]
pub enum TokenType {
    KeyWord(KeyWord),
    Num,
    Stringliteral,
    Symbol,
    Var,
    Unknown,
}
#[derive(Debug)]
pub enum KeyWord {
    Let,
    Fn,
}
#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub value: Option<String>,
}

pub struct Lexer {
    input: String,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
    pub fn iter(&self) -> LexerIter {
        LexerIter::new(&self.input)
    }
}
pub struct LexerIter<'a> {
    input: &'a str,
    pos: usize,
    read_pos: usize,
    char: u8,
}
impl LexerIter<'_> {
    fn new(lex: &str) -> LexerIter {
        let mut li = LexerIter {
            input: lex,
            pos: 0,
            read_pos: 0,
            char: 0,
        };
        li.read_next();
        li
    }
    fn read_next(&mut self) {
        let bytes = self.input.as_bytes();
        if self.read_pos < self.input.len(){
            self.char = bytes[self.read_pos]
        }
        self.pos = self.read_pos;
        self.read_pos = self.read_pos + 1;
    }
    fn read_var(&mut self) -> String {

        let p = self.pos;
        let current_char = self.char as char;
        while is_letter(current_char) && self.pos < self.input.len() {
            self.read_next();
        }
        self.input[p..self.pos].to_string()
    }

}
impl Iterator for LexerIter<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let current_char = self.char as char;
        if self.pos >= self.input.len() {
            return None;
        } else {
            if is_letter(current_char) {
                return Some(Token {
                    ty: TokenType::Var,
                    value: Some(self.read_var()),
                });
            } else {
                self.read_next();
                return Some(Token {
                    ty: TokenType::Unknown,
                    value: None,
                });
            }
        }
    }
}
// -----------------------utils-----------------------
fn is_letter(c: char) -> bool {
    ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || c == '_'
}
fn is_space(c: char)->bool{
    c == ' ' || c == '\t' || c == '\n'
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let str = "let";
        let lex = Lexer::new(str);
        for i in lex.iter() {
            println!("{:?}", i)
        }
    }
}
