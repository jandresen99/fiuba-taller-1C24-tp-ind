use crate::regex_step::RegexStep;

#[derive(Debug, Clone)]
/// Representa un paso de la regex que ya ha sido evaluado.
pub struct EvaluatedStep {
    /// Representa un paso de la regex que ya ha sido evaluado.
    pub step: RegexStep,

    /// Representa un paso de la regex que ya ha sido evaluado.
    pub match_size: usize,

    /// Representa un paso de la regex que ya ha sido evaluado.
    pub backtrackable: bool,
}
