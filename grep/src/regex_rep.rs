#[derive(Debug, Clone)]
pub enum RegexRep {
    // Repetition
    Any,                                 // *
    Exact(usize),                        // {n}
    Range(Option<usize>, Option<usize>), // {n,m}
    Optional,                            // ?
    Last,                                // $
}
