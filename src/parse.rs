use crate::evaluate::{Calculator};

use crate::{*};

#[derive(Debug)]
pub struct Variable{
    name:String,
    value:String,
}
pub fn parse(tokens:& [Token])->Vec<Variable>{
    let mut skip_num = 0;
    let mut res:Vec<Variable> = vec![];
    for index in 0..tokens.len(){
        if skip_num > 0 {
            println!("{}",skip_num);
            skip_num = skip_num - 1;
            continue;
        }
        //token为变量
        if tokens[index].ty == TokenType::Var{
            //变量之后为等号
            println!("{:?}",&tokens[index]);
            if index +1 >= tokens.len(){
                break;
            }
            if tokens[index+1].ty == TokenType::Symbol(SymbolType::Assign){

                let name = tokens[index].value.as_ref().unwrap();
                let mut expr_str:String = "".to_string();

                for i in index+2..tokens.len(){
                    if tokens[i].ty == TokenType::Num || tokens[i].ty ==TokenType::Operator || 
                    tokens[i].ty ==TokenType::Var{
                        if tokens[i].ty ==TokenType::Var{
                            if tokens[i-1].ty ==  TokenType::Num{
                                skip_num = (index..i).len()-1;
                                break;
                            }

                            let val = res.iter().find(|x|{

                                &x.name == tokens[i].value.as_ref().unwrap()
                            });
                            match val {
                                Some(value)=>{
                                    expr_str = expr_str.to_owned() + &value.value;
                                },
                                None => {
                                    panic!("no such var")
                                }
                            }
                        }else {
                            expr_str = expr_str.to_owned() + tokens[i].value.as_ref().unwrap().as_str();
                        }

                    }
                }
                let value= Calculator::final_result(&expr_str);
                res.push(Variable{
                    name:name.to_owned(),
                    value:value.unwrap().to_string()
                })
            }
        }else if  tokens[index].ty == TokenType::Symbol(SymbolType::Assign)|| tokens[index].ty == TokenType::Operator{
            continue;
        }else if tokens[index].ty == TokenType::Num || tokens[index].ty == TokenType::Stringliteral{
            continue;
        }
    }
    res
}