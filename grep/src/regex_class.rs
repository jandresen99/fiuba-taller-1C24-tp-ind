#[derive(Debug, Clone)]
/// Representa un tipo de repetición de clase a la cual debe pertenecer el caracter a evaluar.
pub enum RegexClass {
    /// Debe ser un caracter alfanumerico (letras + numeros).
    Alphanumeric,

    /// Debe ser un caracter alphabetico (letras).
    Alphabetic,

    /// Debe ser un digito (numeros).
    Digit,

    /// Debe ser una letra en minuscula.
    Lowercase,
    
    /// Debe ser una letra en mayuscula.
    Uppercase,

    /// Debe ser un espacio en blanco.
    Whitespace,

    /// Debe ser un simbolo de puntuacion.
    Punctuation,
}