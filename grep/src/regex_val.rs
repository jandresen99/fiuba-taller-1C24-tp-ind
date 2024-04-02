use crate::regex_class::RegexClass;

#[derive(Debug, Clone)]
pub enum RegexVal {
    Literal(char), // Caracteres normales
    Wildcard,
    Allowed(Vec<char>),
    NotAllowed(Vec<char>),
    Class(RegexClass),
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        //println!("matching value '{}' with {:?}", value, self);
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
                    if v.contains(&a) {
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
                    if !v.contains(&a) {
                        a.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            Self::Class(class_type) => match class_type {
                RegexClass::Alphanumeric => {
                    if let Some(a) = value.chars().next() {
                        if a.is_alphanumeric() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Alphabetic => {
                    if let Some(a) = value.chars().next() {
                        if a.is_alphabetic() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Digit => {
                    if let Some(a) = value.chars().next() {
                        if a.is_digit(10) {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Lowercase => {
                    if let Some(a) = value.chars().next() {
                        if a.is_lowercase() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Uppercase => {
                    if let Some(a) = value.chars().next() {
                        if a.is_uppercase() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Whitespace => {
                    if let Some(a) = value.chars().next() {
                        if a.is_whitespace() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
                RegexClass::Punctuation => {
                    if let Some(a) = value.chars().next() {
                        if a.is_ascii_punctuation() {
                            a.len_utf8()
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                }
            },
        }
    }
}
