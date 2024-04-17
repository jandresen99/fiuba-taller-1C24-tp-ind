use crate::{regex_rep::RegexRep, regex_val::RegexVal};

#[derive(Debug, Clone)]
/// Esta estructura representa un paso de la regex a evaluar.
pub struct RegexStep {
    /// Cada paso debe incluir un tipo de valor contra el cual se va a evaluar otro valor.
    pub val: RegexVal,

    /// Cada paso debe incluir un tipo de repeticion.
    pub rep: RegexRep,
}
