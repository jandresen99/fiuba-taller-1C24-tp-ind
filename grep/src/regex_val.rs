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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_literal() {
        let regex_val = RegexVal::Literal('a');
        assert_eq!(regex_val.matches("apple"), 1);
        assert_eq!(regex_val.matches("banana"), 0);
    }

    #[test]
    fn test_matches_wildcard() {
        let regex_val = RegexVal::Wildcard;
        assert_eq!(regex_val.matches("apple"), 1);
        assert_eq!(regex_val.matches(""), 0);
    }

    #[test]
    fn test_matches_allowed() {
        let regex_val = RegexVal::Allowed(vec!['a', 'e', 'i', 'o', 'u']);
        assert_eq!(regex_val.matches("apple"), 1);
        assert_eq!(regex_val.matches("banana"), 0);
    }

    #[test]
    fn test_matches_not_allowed() {
        let regex_val = RegexVal::NotAllowed(vec!['a', 'e', 'i', 'o', 'u']);
        assert_eq!(regex_val.matches("apple"), 0);
        assert_eq!(regex_val.matches("banana"), 1);
    }

    #[test]
    fn test_matches_class() {
        let regex_val_alphanumeric = RegexVal::Class(RegexClass::Alphanumeric);
        assert_eq!(regex_val_alphanumeric.matches("1"), 1);
        assert_eq!(regex_val_alphanumeric.matches("!"), 0);

        let regex_val_alphabetic = RegexVal::Class(RegexClass::Alphabetic);
        assert_eq!(regex_val_alphabetic.matches("a"), 1);
        assert_eq!(regex_val_alphabetic.matches("1"), 0);

        let regex_val_digit = RegexVal::Class(RegexClass::Digit);
        assert_eq!(regex_val_digit.matches("1"), 1);
        assert_eq!(regex_val_digit.matches("a"), 0);

        let regex_val_lowercase = RegexVal::Class(RegexClass::Lowercase);
        assert_eq!(regex_val_lowercase.matches("a"), 1);
        assert_eq!(regex_val_lowercase.matches("A"), 0);

        let regex_val_uppercase = RegexVal::Class(RegexClass::Uppercase);
        assert_eq!(regex_val_uppercase.matches("A"), 1);
        assert_eq!(regex_val_uppercase.matches("a"), 0);

        let regex_val_whitespace = RegexVal::Class(RegexClass::Whitespace);
        assert_eq!(regex_val_whitespace.matches(" "), 1);
        assert_eq!(regex_val_whitespace.matches("a"), 0);

        let regex_val_punctuation = RegexVal::Class(RegexClass::Punctuation);
        assert_eq!(regex_val_punctuation.matches("!"), 1);
        assert_eq!(regex_val_punctuation.matches("a"), 0);
    }
}
