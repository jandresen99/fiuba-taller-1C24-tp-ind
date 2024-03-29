#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char), // Caracteres normales
    Wildcard,
    //Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            Self::Literal(l) => {
                println!("value {:?}", value.chars().next());
                println!("self {:?}", Some(*l));
                if Some(*l) == value.chars().next() {
                    l.len_utf8() // cantidad consumida en el input
                } else {
                    0
                }
            }
            RegexVal::Wildcard => {
                if let Some(w) = value.chars().next() {
                    w.len_utf8() // cantidad consumida en el input
                } else {
                    0
                }
            }
        }
    }
}