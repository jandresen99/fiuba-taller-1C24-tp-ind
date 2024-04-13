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
                    '.' => handle_dot(),
                    '*' => handle_star(&mut steps)?,
                    '\\' => handle_backslash(&mut char_iterator)?,
                    '[' => handle_brace(&mut char_iterator)?,
                    '{' => handle_bracket(&mut steps, &mut char_iterator)?,
                    '+' => handle_plus(&mut steps)?,
                    '?' => handle_question_mark(&mut steps)?,
                    '$' => handle_end_anchoring(&mut steps, &mut char_iterator)?,
                    _ => handle_default(c),
                };

                if let Some(p) = step {
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
                match step.rep {
                    RegexRep::Exact(n) => {
                        let mut match_size = 0;
                        for _ in [0..n] {
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
                            backtrackable: true,
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
                        let mut matched_range: bool = false;
                        while keep_matching {
                            let match_size = step.val.matches(&value[index..]);
                            if match_size != 0 {
                                match_counter += 1;
                                index += match_size;
                                stack.push(EvaluatedStep {
                                    step: step.clone(),
                                    match_size,
                                    backtrackable: false,
                                });

                                match (min, max) {
                                    (Some(min_val), Some(max_val)) => {
                                        if match_counter >= min_val as i32
                                            && match_counter <= max_val as i32
                                        {
                                            matched_range = true
                                        }
                                        if match_counter == max_val as i32 {
                                            keep_matching = false
                                        }
                                    }
                                    (Some(min_val), None) => {
                                        if match_counter >= min_val as i32 {
                                            matched_range = true
                                        }
                                    }
                                    (None, Some(max_val)) => {
                                        if match_counter <= max_val as i32 {
                                            matched_range = true
                                        }
                                        if match_counter == max_val as i32 {
                                            keep_matching = false
                                        }
                                    }
                                    (None, None) => matched_range = false,
                                }
                            } else {
                                keep_matching = false;
                            }
                        }

                        if !matched_range {
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

fn handle_dot() -> Option<RegexStep> {
    return Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Wildcard,
    });
}

fn handle_star(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, std::io::Error> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Any;
        Ok(None)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unexpected '*' found",
        ))
    }
}

fn handle_backslash(
    char_iterator: &mut std::str::Chars,
) -> Result<Option<RegexStep>, std::io::Error> {
    match char_iterator.next() {
        Some(literal) => Ok(Some(RegexStep {
            rep: RegexRep::Exact(1),
            val: RegexVal::Literal(literal),
        })),
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Unexpected character found \\"),
            ))
        }
    }
}

fn handle_brace(char_iterator: &mut std::str::Chars) -> Result<Option<RegexStep>, std::io::Error> {
    match char_iterator.next() {
        Some(char) => match char {
            '[' => {
                let next_seven_chars: String = char_iterator.by_ref().take(7).collect();

                let class_type = match next_seven_chars.as_str() {
                    ":alnum:" => RegexClass::Alphanumeric,
                    ":alpha:" => RegexClass::Alphabetic,
                    ":digit:" => RegexClass::Digit,
                    ":lower:" => RegexClass::Lowercase,
                    ":upper:" => RegexClass::Uppercase,
                    ":space:" => RegexClass::Whitespace,
                    ":punct:" => RegexClass::Punctuation,
                    _ => return Err(Error::new(ErrorKind::Other, "Unexpected class found")),
                };

                char_iterator.next();
                char_iterator.next();

                Ok(Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Class(class_type),
                }))
            }
            _ => {
                let mut allowed_chars: Vec<char> = vec![];
                let mut not_allowed = false;

                match char {
                    ']' => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Unexpected character found (])"),
                        ))
                    }
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
                    Ok(Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::NotAllowed(allowed_chars),
                    }))
                } else {
                    Ok(Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Allowed(allowed_chars),
                    }))
                }
            }
        },
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("No character found after brace"),
            ))
        }
    }
}

fn handle_bracket(
    steps: &mut Vec<RegexStep>,
    char_iterator: &mut std::str::Chars,
) -> Result<Option<RegexStep>, std::io::Error> {
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
                format!("Unexpected character found ({}) inside bracket", c),
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

    Ok(Some(RegexStep {
        rep: RegexRep::Range(final_min, final_max),
        val: last_val,
    }))
}

fn handle_plus(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, std::io::Error> {
    let last_val: RegexVal;
    if let Some(last) = steps.last_mut() {
        last_val = last.val.clone()
    } else {
        return Err(Error::new(ErrorKind::Other, "Unexpected '+' found"));
    }

    Ok(Some(RegexStep {
        rep: RegexRep::Any,
        val: last_val,
    }))
}

fn handle_question_mark(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, std::io::Error> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Optional;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unexpected '?' found"));
    }
    Ok(None)
}

fn handle_end_anchoring(
    steps: &mut Vec<RegexStep>,
    char_iterator: &mut std::str::Chars,
) -> Result<Option<RegexStep>, std::io::Error> {
    match char_iterator.next() {
        None => {
            if let Some(last) = steps.last_mut() {
                last.rep = RegexRep::Last;
            } else {
                return Err(Error::new(ErrorKind::Other, "Unexpected '$' found"));
            }
            Ok(None)
        }
        Some(_) => return Err(Error::new(ErrorKind::Other, "Unexpected '$' found")),
    }
}

fn handle_default(c: char) -> Option<RegexStep> {
    return Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Literal(c),
    });
}