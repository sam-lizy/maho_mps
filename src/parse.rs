use crate::evaluate::{Calculator};

use crate::{*};
pub enum Expr{
    Variable(Variable)
}
#[derive(Debug)]
pub struct Variable{
    name:String,
    value:String,
}
pub fn parse(tokens:& [Token])->Vec<Variable>{
    let mut res = vec![];
    for index in 0..tokens.len(){
        //token为变量
        if tokens[index].ty == TokenType::Var{
            if tokens[index+1].ty == TokenType::Symbol(SymbolType::Assign){
                let name = tokens[index].value.as_ref().unwrap();
                let mut expr_str:String = "".to_string();

                for i in index+2..tokens.len(){
                    if tokens[i].ty == TokenType::Num || tokens[i].ty ==TokenType::Operator{
                        expr_str = expr_str.to_owned() + tokens[i].value.as_ref().unwrap().as_str();
                    }
                }
                let value= Calculator::final_result(&expr_str);
                res.push(Variable{
                    name:name.to_owned(),
                    value:value.unwrap().to_string()
                })
            }
        }else if  tokens[index].ty == TokenType::Symbol(SymbolType::Assign){
            continue;
        }else if tokens[index].ty == TokenType::Num || tokens[index].ty == TokenType::Stringliteral{
            continue;
        }
    }
    res
}