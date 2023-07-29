use std::fmt::Debug;

enum Errors {
    Varerror
}
impl Debug for Errors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Varerror =>{
                write!(f,"var error")
            }
        }
    }
}