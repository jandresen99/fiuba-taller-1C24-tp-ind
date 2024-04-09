use crate::regex_step::RegexStep;

#[derive(Debug, Clone)]
pub struct EvaluatedStep {
    pub step: RegexStep,
    pub match_size: usize,
    pub backtrackable: bool,
}
