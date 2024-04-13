use grep::regex::Regex;

#[test]
fn test_period() {
    let expression = "ab.de";
    let value1 = "abcde";
    let value2 = "abde";
    let value3 = "zyxabcde";
    let value4 = "abcdefghi";
    let value5 = "zyxabcdefghi";
    let value6 = "zyxabdefghi";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
    assert_eq!(Regex::new(expression).unwrap().test(&value3).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value4).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value5).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value6).unwrap(),
        false
    );
}

#[test]
fn test_period_and_repetition() {
    let expression = "ab.*cd";
    let value1 = "abxcd";
    let value2 = "abxxxcd";
    let value3 = "abcd";
    let value4 = "abxxxxcdefghi";
    let value5 = "zyxabxxxxcdefghi";
    let value6 = "zyxabcdefghi";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value3).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value4).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value5).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value6).unwrap(), true);
}

#[test]
fn test_brackets() {
    let expression = "a[bc]d";
    let value1 = "abd";
    let value2 = "acd";
    let value3 = "ad";
    let value4 = "aed";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
}

#[test]
fn test_braces() {
    let expression = "ab{2,4}cd";
    let value1 = "abbcd";
    let value2 = "abbbcd";
    let value3 = "abbbbcd";
    let value4 = "abbbbbcd";
    let value5 = "abcd";
    let value6 = "accd";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value3).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value5).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value6).unwrap(),
        false
    );
}

#[test]
fn test_vertical_var_and_plus() {
    let expression = "abc|de+f";
    let value1 = "abc";
    let value2 = "abcdef";
    let value3 = "abcdeeef";
    let value4 = "abd";
    let value5 = "abdef";
    let value6 = "abdeeef";
    let value7 = "abcdf";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value3).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
    assert_eq!(Regex::new(expression).unwrap().test(&value5).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value6).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value7).unwrap(), true);
}

#[test]
fn test_text_and_brackets() {
    let expression = "la [aeiou] es una vocal";
    let value1 = "la a es una vocal";
    let value2 = "la e es una vocal";
    let value3 = "la i es una vocal";
    let value4 = "la o es una vocal";
    let value5 = "la u es una vocal";
    let value6 = "la b es una vocal";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value3).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value4).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value5).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value6).unwrap(),
        false
    );
}

#[test]
fn test_text_and_brackets_denied() {
    let expression = "la [^aeiou] es una vocal";
    let value1 = "la a es una vocal";
    let value2 = "la e es una vocal";
    let value3 = "la i es una vocal";
    let value4 = "la o es una vocal";
    let value5 = "la u es una vocal";
    let value6 = "la b es una vocal";

    assert_eq!(
        Regex::new(expression).unwrap().test(&value1).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value5).unwrap(),
        false
    );
    assert_eq!(Regex::new(expression).unwrap().test(&value6).unwrap(), true);
}

#[test]
fn test_alpha_and_plus() {
    let expression = "hola [[:alpha:]]+";
    let value1 = "hola a";
    let value2 = "hola aaa";
    let value3 = "hola ";
    let value4 = "hola 1";
    let value5 = "hola 11";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value5).unwrap(),
        false
    );
}

#[test]
fn test_digit_and_text() {
    let expression = "[[:digit:]] es un numero";
    let value1 = "45 es un numero";
    let value2 = "a es un numero";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
}

#[test]
fn test_alnum_and_text() {
    let expression = "el caracter [[:alnum:]] no es un simbolo";
    let value1 = "el caracter a no es un simbolo";
    let value2 = "el caracter 4 no es un simbolo";
    let value3 = "el caracter ? no es un simbolo";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(Regex::new(expression).unwrap().test(&value2).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
}

#[test]
fn test_space_and_text() {
    let expression = "hola[[:space:]]mundo";
    let value1 = "hola mundo";
    let value2 = "holamundo";
    let value3 = "holaamundo";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
}

#[test]
fn test_upper_and_text() {
    let expression = "[[:upper:]]ascal[[:upper:]]ase";
    let value1 = "PascalPase";
    let value2 = "Pascalpase";
    let value3 = "pascalPase";
    let value4 = "pascalpase";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value4).unwrap(),
        false
    );
}

#[test]
fn test_anchoring() {
    let expression = "es el fin$";
    let value1 = "este es el fin";
    let value2 = "es el fin no?";
    let value3 = "es el finde";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
    assert_eq!(
        Regex::new(expression).unwrap().test(&value2).unwrap(),
        false
    );
    assert_eq!(
        Regex::new(expression).unwrap().test(&value3).unwrap(),
        false
    );
}

#[test]
fn test_1() {
    let expression = "ab.?d";
    let value1 = "abd";

    assert_eq!(Regex::new(expression).unwrap().test(&value1).unwrap(), true);
}