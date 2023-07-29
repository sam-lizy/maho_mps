mod evaluate;
mod check;
mod parse;
use check::{*};
mod errors;
//------------lexer----------
#[derive(Debug,PartialEq, Eq)]
pub enum TokenType {
    KeyWord(KeyWord),
    Num,
    Stringliteral,
    Symbol(SymbolType),
    Var,
    Operator,
    Unknown,
}
#[derive(Debug,PartialEq, Eq)]
pub enum KeyWord {
    Let,
    Fn,
}
#[derive(Debug,PartialEq, Eq)]
pub enum SymbolType{
    Assign,
    Leftquotation,
    Rightquotation,
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
        self.input[p..self.pos].to_string()
    }
    fn read_num(&mut self) -> String {
        let p = self.pos;
        while is_num(self.char as char) && self.pos < self.input.len() {
            self.read_next();
        }
        self.input[p..self.pos].to_string()
    }
    fn read_str(&mut self)->String{
        self.read_next();
        let p = self.pos;
        while is_letter(self.char as char) || is_num(self.char as char){
            self.read_next();
            if self.char as char == '"' {
                break;
            }
        }
        self.input[p..self.pos].to_string()
    }
    fn skip_space(&mut self){
        while is_space(self.char as char) {
            self.read_next();
        }
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
                self.skip_space();
                current_char = self.char as char;
            }
            println!("{:?}",current_char);
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
                }else if current_char == '"'{
                    res = Token{
                        ty:TokenType::Stringliteral,
                        value:Some(self.read_str())
                    }
                }
            }else if is_num(current_char) {
                return Some(Token{
                    ty:TokenType::Num,
                    value:Some(self.read_num())
                }); 
            }else if is_operator(current_char) {
                res = Token{
                    ty:TokenType::Operator,
                    value:Some(current_char.to_string())
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
        let str = "x = 5+6";
        let mut res = vec![];
        let lex = Lexer::new(str);
        for i in lex.iter() {
            res.push(i);
        }
        println!("{:?}",parse::parse(&res))

        // for i in str.chars(){
        //     println!("{:?}",i)
        // }
    }
}
