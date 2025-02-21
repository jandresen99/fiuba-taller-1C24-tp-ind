use crate::evaluated_step::EvaluatedStep;
use crate::regex_class::RegexClass;
use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexVal;
use std::collections::VecDeque;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Regex {
    expression_steps: Vec<Vec<RegexStep>>,
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, std::io::Error> {
        let mut expression_steps: Vec<Vec<RegexStep>> = vec![];

        let aux_expressions: Vec<&str> = expression.split('|').collect();

        for aux_expression in aux_expressions {
            let mut steps: Vec<RegexStep> = vec![];
            // Recorremos la expression char por char con .next()
            let mut char_iterator = aux_expression.chars();

            if !&aux_expression.starts_with('^') {
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
                    'a'..='z' | ' ' => Some(RegexStep {
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
                        None => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Unexpected character found (1)",
                            ))
                        }
                    },
                    '[' => match char_iterator.next() {
                        Some(char) => match char {
                            '[' => {
                                let next_five_chars: String =
                                    char_iterator.by_ref().take(7).collect();

                                let class_type = match next_five_chars.as_str() {
                                    ":alnum:" => RegexClass::Alphanumeric,
                                    ":alpha:" => RegexClass::Alphabetic,
                                    ":digit:" => RegexClass::Digit,
                                    ":lower:" => RegexClass::Lowercase,
                                    ":upper:" => RegexClass::Uppercase,
                                    ":space:" => RegexClass::Whitespace,
                                    ":punct:" => RegexClass::Punctuation,
                                    _ => {
                                        return Err(Error::new(
                                            ErrorKind::Other,
                                            "Unexpected class found",
                                        ))
                                    }
                                };

                                char_iterator.next();
                                char_iterator.next();

                                Some(RegexStep {
                                    rep: RegexRep::Exact(1),
                                    val: RegexVal::Class(class_type),
                                })
                            }
                            _ => {
                                let mut allowed_chars: Vec<char> = vec![];
                                let mut not_allowed = false;

                                match char {
                                    ']' => break,
                                    '^' => not_allowed = true,
                                    _ => allowed_chars.push(char),
                                }

                                while let Some(next_char) = char_iterator.next() {
                                    match next_char {
                                        ']' => break,
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
                        },
                        None => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                "Unexpected character found (2)",
                            ))
                        }
                    },
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
                        Some(_) => {
                            return Err(Error::new(ErrorKind::Other, "Unexpected '$' found"))
                        }
                    },

                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Unexpected character found (3)",
                        ))
                    }
                };

                if let Some(p) = step {
                    //println!("added step {:?}", p);
                    steps.push(p);
                }
            }
            expression_steps.push(steps);
        }
        Ok(Regex { expression_steps })
    }

    pub fn test(self, value: &str) -> Result<bool, std::io::Error> {
        if !value.is_ascii() {
            return Err(Error::new(ErrorKind::Other, "The input is not ASCII"));
        }

        let mut final_result = false;

        'main: for steps in self.expression_steps {
            let mut queue = VecDeque::from(steps);
            let mut stack: Vec<EvaluatedStep> = Vec::new();
            let mut index = 0;

            'steps: while let Some(step) = queue.pop_front() {
                //println!("running step {:?}", step);
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
                                    None => continue 'main,
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
                            continue 'main;
                        } else {
                            match_size += size;
                            index += size;

                            match value.chars().nth(index) {
                                Some(_) => continue 'main,
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
                                matches_range = match_counter >= min_val as i32
                                    && match_counter <= max_val as i32
                            }
                            (Some(min_val), None) => {
                                matches_range = match_counter >= min_val as i32
                            }
                            (None, Some(max_val)) => {
                                matches_range = match_counter <= max_val as i32
                            }
                            (None, None) => matches_range = false,
                        }

                        if !matches_range {
                            continue 'main;
                        }
                    }
                }
            }
            final_result = true;
        }

        Ok(final_result)
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
