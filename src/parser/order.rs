#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityOrder {
    Lowest,
    Equals,      // '==' or '!='
    LessGreater, // '>' or '<'
    Sum,         // '+' or '-'
    Product,     // '*' or '/'
    Prefix,      // '!' or '-'
    Call,        // add(...)
    Postfix,     // 'hoge++' or 'hoge--'
}
