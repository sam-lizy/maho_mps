use std::collections::HashMap;
use std::fmt::{self, Debug};
use crate::evaluate::Calculator;
use crate::{Token,TokenType,SymbolType};
#[derive(Debug,PartialEq, Eq)]
pub enum ParmeterType {
    Int,
}
impl Debug for Variable{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"name:{},value:{:?}",self.name,self.value)
    }
}
pub struct Variable{
    name:String,
    value:Option<String>,
    fn_call:Option<Box<dyn Fn(&[String])>>,
parameter:Option<Vec<String>>,
    parmeter_type:Option<ParmeterType>
}
impl Variable{
    pub fn get_parameter(&self)->&[String]{
        let pars = self.parameter.as_ref().unwrap();
        pars

    }
    pub fn set_fn_call(&mut self,call:Box<dyn Fn(&[String])>){
        self.fn_call = Some(call);
    }
}

pub fn parse(tokens:& [Token])->Vec<Variable>{
    let mut skip_num = 0;
    let mut res:Vec<Variable> = vec![];
    //添加print函数
    {
        let mut print_var =  Variable { 
            name: String::from("print"), 
            value: None, 
            fn_call: None,
            parameter:Some(vec![String::from("x")]),
            parmeter_type:Some(ParmeterType::Int)
        };
        // let var = print_var.get_parameter()[0].clone();
        let pars_vec = print_var.get_parameter();
        print_var.set_fn_call(Box::new(
            |pars_vec: &[String]|
            {
                println!("{}",pars_vec[0])
            }
        ));
        res.push(
            print_var
        );
    }

    for index in 0..tokens.len(){
        if skip_num > 0 {
            skip_num = skip_num - 1;
            continue;
        }
        println!("{}","-----------------");
        println!("{:?}",&res);
        //token为变量
        if tokens[index].ty == TokenType::Var{
            //变量之后为等号
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
                                    expr_str = expr_str.to_owned() + &value.value.as_ref().expect("no such var");
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
                    value:Some(value.unwrap().to_string()),
                    fn_call:None,
                    parameter:None,
                    parmeter_type:None
                })
            }
        }else if  tokens[index].ty == TokenType::Symbol(SymbolType::Assign)|| tokens[index].ty == TokenType::Operator{
            continue;
        }else if tokens[index].ty == TokenType::Num || tokens[index].ty == TokenType::Stringliteral{
            continue;
        }else if tokens[index].ty == TokenType::FnDeclare {
            // token为函数时
            match &tokens[index+1].ty{
                // fn print(){
                //  x = y + 1
                //  print(x)
                //}
                TokenType::Scope(scope)=>{
                    continue;
                },
                _ => panic!("need fn scope")
            }
        }else if tokens[index].ty == TokenType::FnCall {
            //函数调用时
            //寻找函数名称
            if let Some(i) = res.iter().position(|f| f.name == tokens[index].value.clone().unwrap()) {
                println!("{:?}",&res[i]);
                if let Some(parameters) = tokens[index].parameter.clone() {
                    if let Some(first_parameter) = parameters.get(0) {
                        println!("{}",first_parameter);
                        if let Some(par_value) = res.iter().find(|p| p.name == *first_parameter) {
                            // 处理参数值
                            *res[i].parameter.as_mut().unwrap().get_mut(0).unwrap() = par_value.value.as_ref().unwrap().to_string();
                            let fn_call = res[i].fn_call.as_ref().unwrap();
                            fn_call(res[i].get_parameter());
                            continue;
                        }else {
                            panic!("no such parrmeter")
                        }
                    }
                }
            }
        }
    }
    res
}