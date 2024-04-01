use crate::evaluated_step::EvaluatedStep;
use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexVal;
use std::collections::VecDeque;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Regex {
    steps: Vec<RegexStep>,
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, std::io::Error> {
        let mut steps: Vec<RegexStep> = vec![];

        // Recorremos la expression char por char con .next()
        let mut char_iterator = expression.chars();

        if !&expression.starts_with('^') {
            let first_step = RegexStep {
                rep: RegexRep::Any,
                val: RegexVal::Wildcard,
            };

            steps.push(first_step)
        } else {
            char_iterator.next();
        }

        while let Some(c) = char_iterator.next() {
            let step = match c {
                '.' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Wildcard,
                }),
                'a'..='z' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Literal(c),
                }),
                ' ' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Literal(c),
                }),
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Any;
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Unexpected '*' found"));
                    }
                    None
                }
                '\\' => match char_iterator.next() {
                    Some(literal) => Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(literal),
                    }),
                    None => return Err(Error::new(ErrorKind::Other, "Unexpected character found")),
                },
                '[' => {
                    let mut allowed_chars: Vec<char> = vec![];
                    let mut not_allowed = false;
                    while let Some(next_char) = char_iterator.next() {
                        match next_char {
                            ']' => break,
                            '^' => not_allowed = true,
                            _ => allowed_chars.push(next_char),
                        }
                    }

                    if not_allowed {
                        Some(RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexVal::NotAllowed(allowed_chars),
                        })
                    } else {
                        Some(RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexVal::Allowed(allowed_chars),
                        })
                    }
                }
                '{' => {
                    let last_val: RegexVal;
                    if let Some(last) = steps.last_mut() {
                        last_val = last.val.clone()
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Unexpected '{' found"));
                    }

                    let mut min = String::new();
                    let mut max = String::new();

                    let mut min_done = false;

                    while let Some(c) = char_iterator.next() {
                        if c.is_digit(10) {
                            if !min_done {
                                min.push(c);
                            } else {
                                max.push(c);
                            }
                        } else if c == ',' {
                            min_done = true;
                            continue;
                        } else if c == '}' {
                            break;
                        } else {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Unexpected character found inside bracket",
                            ));
                        }
                    }

                    let min_usize = min.parse::<usize>();
                    let max_usize = max.parse::<usize>();

                    let final_min: Option<usize>;
                    let final_max: Option<usize>;

                    match min_usize {
                        Ok(min_num) => {
                            final_min = Some(min_num);
                        }
                        Err(_err) => {
                            final_min = None;
                        }
                    }

                    match max_usize {
                        Ok(max_num) => {
                            final_max = Some(max_num);
                        }
                        Err(_err) => {
                            final_max = None;
                        }
                    }

                    Some(RegexStep {
                        rep: RegexRep::Range(final_min, final_max),
                        val: last_val,
                    })
                }
                '+' => {
                    let last_val: RegexVal;
                    if let Some(last) = steps.last_mut() {
                        last_val = last.val.clone()
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Unexpected '*' found"));
                    }

                    Some(RegexStep {
                        rep: RegexRep::Any,
                        val: last_val,
                    })
                }
                '?' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Optional;
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Unexpected '?' found"));
                    }
                    None
                }
                '$' => match char_iterator.next() {
                    None => {
                        if let Some(last) = steps.last_mut() {
                            last.rep = RegexRep::Last;
                        } else {
                            return Err(Error::new(ErrorKind::Other, "Unexpected '$' found"));
                        }
                        None
                    }
                    Some(_) => return Err(Error::new(ErrorKind::Other, "Unexpected '$' found")),
                },

                _ => return Err(Error::new(ErrorKind::Other, "Unexpected character found")),
            };

            if let Some(p) = step {
                steps.push(p);
            }
        }

        Ok(Regex { steps })
    }

    pub fn test(self, value: &str) -> Result<bool, std::io::Error> {
        if !value.is_ascii() {
            return Err(Error::new(ErrorKind::Other, "The input is not ASCII"));
        }

        let mut queue = VecDeque::from(self.steps);
        let mut stack: Vec<EvaluatedStep> = Vec::new();
        let mut index = 0;

        'steps: while let Some(step) = queue.pop_front() {
            //println!("step {:?}", step);
            match step.rep {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;
                    for _ in [1..n] {
                        let size = step.val.matches(&value[index..]);

                        if size == 0 {
                            match backtrack(step, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'steps;
                                }
                                None => return Ok(false),
                            }
                        } else {
                            match_size += size;
                            index += size;
                        }
                    }
                    stack.push(EvaluatedStep {
                        step: step,
                        match_size,
                        backtrackable: false,
                    })
                }
                RegexRep::Last => {
                    let mut match_size = 0;

                    let size = step.val.matches(&value[index..]);

                    if size == 0 {
                        return Ok(false);
                    } else {
                        match_size += size;
                        index += size;

                        match value.chars().nth(index) {
                            Some(_) => return Ok(false),
                            None => stack.push(EvaluatedStep {
                                step: step,
                                match_size,
                                backtrackable: false,
                            }),
                        }
                    }
                }
                RegexRep::Optional => {
                    let mut match_size = 0;

                    let size = step.val.matches(&value[index..]);

                    if size != 0 {
                        match_size += size;
                        index += size;
                    }

                    stack.push(EvaluatedStep {
                        step: step,
                        match_size,
                        backtrackable: false,
                    })
                }
                RegexRep::Any => {
                    let mut keep_matching = true;
                    while keep_matching {
                        let match_size = step.val.matches(&value[index..]);
                        if match_size != 0 {
                            index += match_size;
                            stack.push(EvaluatedStep {
                                step: step.clone(),
                                match_size,
                                backtrackable: true,
                            })
                        } else {
                            keep_matching = false;
                        }
                    }
                }
                RegexRep::Range(min, max) => {
                    let mut keep_matching = true;
                    let mut match_counter = 1; // arranco con 1 porque ya conte el caracter anterior
                    while keep_matching {
                        let match_size = step.val.matches(&value[index..]);
                        if match_size != 0 {
                            match_counter += 1;
                            index += match_size;
                            stack.push(EvaluatedStep {
                                step: step.clone(),
                                match_size,
                                backtrackable: false,
                            })
                        } else {
                            keep_matching = false;
                        }
                    }

                    let matches_range: bool;

                    match (min, max) {
                        (Some(min_val), Some(max_val)) => {
                            matches_range =
                                match_counter >= min_val as i32 && match_counter <= max_val as i32
                        }
                        (Some(min_val), None) => matches_range = match_counter >= min_val as i32,
                        (None, Some(max_val)) => matches_range = match_counter <= max_val as i32,
                        (None, None) => matches_range = false, // No se proporcionan límites, no se puede verificar
                    }

                    if !matches_range {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }
}

fn backtrack(
    current: RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,
) -> Option<usize> {
    let mut back_size = 0;
    next.push_front(current);
    while let Some(e) = evaluated.pop() {
        back_size += e.match_size;

        if e.backtrackable {
            return Some(back_size);
        } else {
            next.push_front(e.step);
        }
    }
    None
}
