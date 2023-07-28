mod check;
use check::{*};
//------------lexer----------
#[derive(Debug)]
pub enum TokenType {
    KeyWord(KeyWord),
    Num,
    Stringliteral,
    Symbol(SymbolType),
    Var,
    Unknown,
}
#[derive(Debug)]
pub enum KeyWord {
    Let,
    Fn,
}
#[derive(Debug)]
pub enum SymbolType{
    Assign
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
        // pos 0 cuttert_pos 1 char:x
        //pos 1 cuuet 2 char:space 
        let p = self.pos;
        while is_letter(self.char as char) && self.pos < self.input.len() {
            self.read_next();
        }
        println!("{}",self.pos);
        self.input[p..self.pos].to_string()
    }
    fn read_num(&mut self) -> String {
        let p = self.pos;
        while is_num(self.char as char) && self.pos < self.input.len() {
            self.read_next();
        }
        self.input[p..self.pos].to_string()
    }

}
impl Iterator for LexerIter<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut current_char = self.char as char;
        let mut res  = Token { ty: TokenType::Unknown, value: None };
        if self.pos >= self.input.len() {
            return None;
        } else {
            if is_space(current_char){
                self.read_next();
                current_char = self.char as char;
            }
            if is_letter(current_char) {
                res =  Token {
                    ty: TokenType::Var,
                    value: Some(self.read_var()),
                };
                return Some(res);
            } else if is_symbol(current_char){
                if current_char == '='{
                    res = Token{
                        ty:TokenType::Symbol(
                            SymbolType::Assign
                        ),
                        value:None
                    }
                }
            }else if is_num(current_char) {
                res = Token{
                    ty:TokenType::Num,
                    value:Some(self.read_num())
                }
            }
            self.read_next();
            Some(res)
        }
    }
}
// -----------------------utils-----------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let str = "x = 55";
        let lex = Lexer::new(str);
        for i in lex.iter() {
            println!("{:?}", i)
        }
    }
}
