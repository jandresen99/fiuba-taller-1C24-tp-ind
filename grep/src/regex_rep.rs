#[derive(Debug, Clone)]
/// Representa un tipo de repetición que va a tener un RegexStep.
pub enum RegexRep {
    /// Se puede repetir varias veces
    Any,                                 // *

    /// Se puede repetir una cantidad especifica de veces
    Exact(usize),                        // {n}

    /// Se puede repetir dentro de un rango de veces
    Range(Option<usize>, Option<usize>), // {n,m}

    /// Puede existir o no
    Optional,                            // ?

    /// Debe ser el final del valor
    Last,                                // $
}
