use crate::regex_class::RegexClass;

#[derive(Debug, Clone)]
/// Esta estructura representa un valor de la expression de una regex.
pub enum RegexVal {
    /// Se busca que el valor sea un caracter especifico.
    Literal(char),

    /// Se busca que el valor sea cualquier caracter.
    Wildcard,

    /// Se busca que el valor se encuentre dentro de un vector de caracteres.
    Allowed(Vec<char>),

    /// Se busca que el valor NO se encuentre dentro de un vector de caracteres.
    NotAllowed(Vec<char>),

    /// Se busca que el valor cumpla una serie de condiciones especificas.
    Class(RegexClass),
}

impl RegexVal {
    /// Prueba si el valor recibido cumple con el tipo de RegexVal.
    pub fn matches(&self, value: &str) -> usize {
        match self {
            Self::Literal(l) => match_literal(l, value),
            Self::Wildcard => match_wildcard(value),
            Self::Allowed(v) => match_allowed(v, value),
            Self::NotAllowed(v) => match_not_allowed(v, value),
            Self::Class(class_type) => match_class(class_type, value),
        }
    }
}

/// Matchea un caracter con el valor recibido, en caso de matchear devuelve su longitud, sino devuelve cero. Esta funcion se utiliza de manera interna dentro de la funcion matches.
fn match_literal(l: &char, value: &str) -> usize {
    if Some(*l) == value.chars().next() {
        l.len_utf8()
    } else {
        0
    }
}

/// Comprueba que el valor recibido exista, en caso de matchear devuelve su longitud, sino devuelve cero. Esta funcion se utiliza de manera interna dentro de la funcion matches.
fn match_wildcard(value: &str) -> usize {
    if let Some(w) = value.chars().next() {
        w.len_utf8()
    } else {
        0
    }
}

/// Comprueba que el valor recibido pertenezca a al vector de caracteres, en caso de matchear devuelve su longitud, sino devuelve cero. Esta funcion se utiliza de manera interna dentro de la funcion matches.
fn match_allowed(v: &Vec<char>, value: &str) -> usize {
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

/// Comprueba que el valor recibido NO pertenezca a al vector de caracteres, en caso de matchear devuelve su longitud, sino devuelve cero. Esta funcion se utiliza de manera interna dentro de la funcion matches.
fn match_not_allowed(v: &Vec<char>, value: &str) -> usize {
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

/// Comprueba que el valor recibido forme parte de la clase de caracteres, en caso de matchear devuelve su longitud, sino devuelve cero. Esta funcion se utiliza de manera interna dentro de la funcion matches.
fn match_class(class_type: &RegexClass, value: &str) -> usize {
    if let Some(a) = value.chars().next() {
        match class_type {
            RegexClass::Alphanumeric => {
                if a.is_alphanumeric() {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Alphabetic => {
                if a.is_alphabetic() {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Digit => {
                if a.is_digit(10) {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Lowercase => {
                if a.is_lowercase() {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Uppercase => {
                if a.is_uppercase() {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Whitespace => {
                if a.is_whitespace() {
                    a.len_utf8()
                } else {
                    0
                }
            }
            RegexClass::Punctuation => {
                if a.is_ascii_punctuation() {
                    a.len_utf8()
                } else {
                    0
                }
            }
        }
    } else {
        0
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
