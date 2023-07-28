pub fn is_letter(c: char) -> bool {
    ('a'..'z').contains(&c) || ('A'..'Z').contains(&c) || c == '_'
}
pub fn is_space(c: char)->bool{
    c == ' ' || c == '\t' || c == '\n'
}
pub fn is_symbol(c:char) ->bool{
    c == '=' || c == '"' 
}
pub fn is_num(c: char)->bool{
    ('0'..'9').contains(&c)
}