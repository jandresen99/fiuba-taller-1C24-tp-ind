use grep::regex::Regex;

#[test]
fn period_test() {
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
fn period_and_repetition_test() {
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
fn brackets_test() {
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
fn braces_test() {
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
fn vertical_var_and_plus_test() {
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
