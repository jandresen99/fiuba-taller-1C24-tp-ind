#[derive(Debug, Clone)]
pub enum RegexClass {
    Alphanumeric,
    Alphabetic,
    Digit,
    Lowercase,
    Uppercase,
    Whitespace,
    Punctuation,
}
