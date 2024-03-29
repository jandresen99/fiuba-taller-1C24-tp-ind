use crate::{regex_rep::RegexRep, regex_val::RegexVal};

#[derive(Debug, Clone)]
pub struct RegexStep {
    pub val: RegexVal,
    pub rep: RegexRep,
}