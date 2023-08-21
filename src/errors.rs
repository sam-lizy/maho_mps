use std::fmt::Debug;

pub enum Errors {
    Varerror,
    NeedFnName
}
impl Debug for Errors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Varerror =>{
                write!(f,"var error")
            },
            Self::NeedFnName =>{
                write!(f,"need fn name")
            }
        }
    }
}