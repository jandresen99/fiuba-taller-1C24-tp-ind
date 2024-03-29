#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char), // Caracteres normales
    Wildcard,
    Allowed(Vec<char>),
    NotAllowed(Vec<char>),
    //Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        println!("matching value '{}' with {:?}", value, self);
        match self {
            Self::Literal(l) => {
                if Some(*l) == value.chars().next() {
                    l.len_utf8()
                } else {
                    0
                }
            }
            Self::Wildcard => {
                if let Some(w) = value.chars().next() {
                    w.len_utf8()
                } else {
                    0
                }
            }
            Self::Allowed(v) => {
                if let Some(a) = value.chars().next() {
                    if v.contains(&a){
                        a.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }

            }
            Self::NotAllowed(v) => {
                if let Some(a) = value.chars().next() {
                    if !v.contains(&a){
                        a.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }

            }
        }
    }
}
