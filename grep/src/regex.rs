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

        if !&expression.starts_with('^'){
            println!("NEW RegexStep without ^");
            let first_step = RegexStep {
                rep: RegexRep::Any,
                val: RegexVal::Wildcard,
            };

            steps.push(first_step)
        }

        while let Some(c) = char_iterator.next() {
            let step = match c {
                '.' => {
                    //println!("NEW RegexStep {}", c);
                    Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Wildcard,
                    })
                }
                'a'..='z' => {
                    //println!("NEW RegexStep {}", c);
                    Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(c),
                    })
                }
                '*' => {
                    //println!("NEW RegexStep {}", c);
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Any;
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Unexpected '*' found"));
                    }
                    None
                }
                '\\' => {
                    //println!("NEW RegexStep {}", c);
                    match char_iterator.next() {
                        Some(literal) => Some(RegexStep {
                            rep: RegexRep::Exact(1),
                            val: RegexVal::Literal(literal),
                        }),
                        None => {
                            return Err(Error::new(ErrorKind::Other, "Unexpected character found"))
                        }
                    }
                }
                '[' => {
                    let mut allowed_chars: Vec<char> = vec![];
                    let mut not_allowed = false;
                    while let Some(next_char) = char_iterator.next(){
                        match next_char {
                            ']' => break,
                            '^' => not_allowed = true,
                            _ => allowed_chars.push(next_char)
                            
                        }
                    }
        
                    //println!("NEW RegexStep {}", c);
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
            println!("step {:?}", step);
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
                RegexRep::Range(_, _) => todo!(),
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
