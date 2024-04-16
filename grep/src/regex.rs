use crate::evaluated_step::EvaluatedStep;
use crate::regex_class::RegexClass;
use crate::regex_rep::RegexRep;
use crate::regex_step::RegexStep;
use crate::regex_val::RegexVal;
use std::collections::VecDeque;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
/// Esta estructura representa una regex
pub struct Regex {
    /// Una regex esta comuesta por varias regex, cada una con uno o más RegexSteps.
    expression_steps: Vec<Vec<RegexStep>>,
}

/// Esta estructura se utiliza dentro de la funcion test para decidir cual loop hay que continuar
enum LoopState {
    MainLoop,
    StepsLoop,
}

impl Regex {
    /// Crea una regex utilizando la expression que recibe por parametro.
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

    /// Prueba la regex contra un string. Devuelve true si el valor cumple con la regex o false si no la cumple.
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
                let loop_state = match step.rep {
                    RegexRep::Exact(n) => {
                        handle_exact(&step, &mut index, &value, &mut stack, &mut queue, n)?
                    }
                    RegexRep::Last => handle_last(&step, &mut index, &value, &mut stack)?,
                    RegexRep::Optional => handle_optional(&step, &mut index, &value, &mut stack)?,
                    RegexRep::Any => handle_any(&step, &mut index, &value, &mut stack)?,
                    RegexRep::Range(min, max) => {
                        handle_range(&step, &mut index, &value, &mut stack, min, max)?
                    }
                };
                match loop_state {
                    Some(l) => match l {
                        LoopState::MainLoop => continue 'main,
                        LoopState::StepsLoop => continue 'steps,
                    },
                    None => (),
                }
            }
            final_result = true;
        }

        Ok(final_result)
    }
}

/// Recibe un RegexStep que se esta ejecutando, una referencia al vector de RegexSteps previamente evaluados y una cola con RegexSteps. El objetivo de la función es retroceder a un RegexStep previo que sea backtrakeable, si lo encuentra devuelve la cantidad de posiciones a retroceder en el valor que se esta testeando, sino devuelve None. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion test.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_dot() -> Option<RegexStep> {
    return Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Wildcard,
    });
}

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_bracket(
    steps: &mut Vec<RegexStep>,
    char_iterator: &mut std::str::Chars,
) -> Result<Option<RegexStep>, std::io::Error> {
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


    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Range(final_min, final_max);
        Ok(None)
    } else {
        return Err(Error::new(ErrorKind::Other, "Unexpected '{' found"));
    }
}

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_question_mark(steps: &mut Vec<RegexStep>) -> Result<Option<RegexStep>, std::io::Error> {
    if let Some(last) = steps.last_mut() {
        last.rep = RegexRep::Optional;
    } else {
        return Err(Error::new(ErrorKind::Other, "Unexpected '?' found"));
    }
    Ok(None)
}

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
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

/// Devuelve un RegexStep que corresponda con el caracter de la regex. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_default(c: char) -> Option<RegexStep> {
    return Some(RegexStep {
        rep: RegexRep::Exact(1),
        val: RegexVal::Literal(c),
    });
}

/// Ejecuta un RegexStep del tipo RegexRep::Exact y devuelve un LoopState para indicar que por cual loop se debe continuar. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_exact(
    step: &RegexStep,
    index: &mut usize,
    value: &str,
    stack: &mut Vec<EvaluatedStep>,
    queue: &mut VecDeque<RegexStep>,
    n: usize,
) -> Result<Option<LoopState>, std::io::Error> {
    let mut match_size = 0;
    for _ in 0..n {
        let size = step.val.matches(&value[*index..]);
        if size == 0 {
            match backtrack(step.clone(), stack, queue) {
                Some(size) => {
                    *index -= size;
                    return Ok(Some(LoopState::StepsLoop));
                }
                None => return Ok(Some(LoopState::MainLoop)),
            }
        } else {
            match_size += size;
            *index += size;
        }
    }
    stack.push(EvaluatedStep {
        step: step.clone(),
        match_size,
        backtrackable: false,
    });
    Ok(None)
}

/// Ejecuta un RegexStep del tipo RegexRep::Last y devuelve un LoopState para indicar que por cual loop se debe continuar. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_last(
    step: &RegexStep,
    index: &mut usize,
    value: &str,
    stack: &mut Vec<EvaluatedStep>,
) -> Result<Option<LoopState>, std::io::Error> {
    let mut match_size = 0;
    let size = step.val.matches(&value[*index..]);

    if size == 0 {
        return Ok(Some(LoopState::MainLoop));
    } else {
        match_size += size;
        *index += size;

        if value.chars().nth(*index).is_some() {
            return Ok(Some(LoopState::MainLoop));
        } else {
            stack.push(EvaluatedStep {
                step: step.clone(),
                match_size,
                backtrackable: false,
            });
            return Ok(None);
        }
    }
}

/// Ejecuta un RegexStep del tipo RegexRep::Optional y devuelve un LoopState para indicar que por cual loop se debe continuar. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_optional(
    step: &RegexStep,
    index: &mut usize,
    value: &str,
    stack: &mut Vec<EvaluatedStep>,
) -> Result<Option<LoopState>, std::io::Error> {
    let mut match_size = 0;
    let size = step.val.matches(&value[*index..]);

    if size != 0 {
        match_size += size;
        *index += size;
    }

    stack.push(EvaluatedStep {
        step: step.clone(),
        match_size,
        backtrackable: true,
    });
    Ok(None)
}

/// Ejecuta un RegexStep del tipo RegexRep::Any y devuelve un LoopState para indicar que por cual loop se debe continuar. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_any(
    step: &RegexStep,
    index: &mut usize,
    value: &str,
    stack: &mut Vec<EvaluatedStep>,
) -> Result<Option<LoopState>, std::io::Error> {
    let mut keep_matching = true;
    while keep_matching {
        let match_size = step.val.matches(&value[*index..]);
        if match_size != 0 {
            *index += match_size;
            stack.push(EvaluatedStep {
                step: step.clone(),
                match_size,
                backtrackable: true,
            });
        } else {
            keep_matching = false;
        }
    }
    Ok(None)
}

/// Ejecuta un RegexStep del tipo RegexRep::Range y devuelve un LoopState para indicar que por cual loop se debe continuar. Esta funcion se utiliza de manera interna dentro de los handlers utilizados en la funcion new.
fn handle_range(
    step: &RegexStep,
    index: &mut usize,
    value: &str,
    stack: &mut Vec<EvaluatedStep>,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<Option<LoopState>, std::io::Error> {
    let mut keep_matching = true;
    let mut match_counter = 0;
    let mut matched_range = false;
    while keep_matching {
        let match_size = step.val.matches(&value[*index..]);
        if match_size != 0 {
            match_counter += 1;
            *index += match_size;
            stack.push(EvaluatedStep {
                step: step.clone(),
                match_size,
                backtrackable: false,
            });
            match (min, max) {
                (Some(min_val), Some(max_val)) => {
                    if match_counter >= min_val && match_counter <= max_val {
                        matched_range = true
                    }
                    if match_counter == max_val {
                        keep_matching = false
                    }
                }
                (Some(min_val), None) => {
                    if match_counter >= min_val {
                        matched_range = true
                    }
                }
                (None, Some(max_val)) => {
                    if match_counter <= max_val {
                        matched_range = true
                    }
                    if match_counter == max_val {
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
        return Ok(Some(LoopState::MainLoop));
    }

    Ok(None)
}
