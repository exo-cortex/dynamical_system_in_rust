// use colored::Colorize;
// use itertools::Itertools;
// use std::collections::HashMap;
use std::fmt;

use crate::var::Var;

#[allow(dead_code)]
pub enum ParameterStatus {
    UniqueAndIsDefault,
    UniqueAndHasBeenUpdated,
}

#[allow(dead_code)]
pub enum ParameterMultiplicity {
    IsUnique,
    CanReoccur,
}

#[allow(dead_code)]
// #[derive(Debug)]
pub struct Parameter<'a> {
    // if parameter can only be set once the outer Vec has only length 1
    pub parameter: Vec<Vec<Var<'a>>>,
    pub set_by: Vec<&'static str>,
    pub explanation: &'static str,
    pub was_set: bool,
    pub is_unique: bool, // can only be set once
}

#[allow(dead_code)]
impl<'a> Parameter<'a> {
    pub fn parameter_length(&self) -> usize {
        self.parameter.len()
    }
}

impl<'a> Parameter<'a> {
    pub fn parse_parameter(&mut self, args: &'a Vec<String>, keywords_index: usize) {
        // let minimum_args = if self.parameter.len() == 1 {
        //     match self.parameter[0] {
        //         Var::Switch(_) => 0,
        //         _ => 1,
        //     }
        // } else {
        //     1
        // };

        let _minimum_args = 1;

        if keywords_index + self.parameter.len() <= args.len() {
            //
            // case 1: is_unique {
            //
            // }
            // case 2:
            // if !self.was_set {
            //     println!("stop doing it.")
            // } else {
            //     if self.is_unique {}
            // }

            // if self.was_set && !self.is_unique {
            //     println!("try find another instance of letter.");
            // }

            // if self.was_set && self.is_unique {
            //     println!("stop this.");
            // }

            // if !self.was_set {
            //
            let mut parameter_set_counter = 1;
            'args_loop: for i in 0..args.len() {
                'set_s_loop: for set_s in &self.set_by {
                    let compare_str = format!("-{}", *set_s);

                    if compare_str == args[i].as_str() {
                        // println!("length of parameter {}: {}", set_s, self.parameter.len());

                        if !self.is_unique && parameter_set_counter > 1 {
                            let parameter_copy = self.parameter.first().unwrap().clone();
                            self.parameter.push(parameter_copy);
                            // println!("length of parameter {}: {}", set_s, self.parameter.len());
                        }
                        parameter_set_counter = parameter_set_counter + 1;
                        // println!("{parameter_set_counter}");

                        // println!("length of parameter {}: {}", set_s, self.parameter.len());
                        //
                        // println!("found `set_by`: {}", compare_str);

                        for (j, part) in &mut self
                            .parameter
                            .last_mut()
                            .expect("here is the problem")
                            // .unwrap()
                            .iter_mut()
                            .enumerate()
                        {
                            if i + j + 1 < args.len() {
                                part.try_parse(args[i + j + 1].as_str());
                            }
                        }
                        self.was_set = true;
                        if self.is_unique {
                            break 'args_loop;
                        }
                        break 'set_s_loop;
                    }
                }
            }
            // } else {
            //     if !self.is_unique {}
            // }
        }
    }
}

#[allow(dead_code)]
impl<'a> fmt::Debug for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = "".to_string();
        for value in &self.parameter {
            string = format!("{}, {:?}", &string, value);
        }
        f.debug_struct("parameter")
            .field("parameter:", &string)
            .finish()
    }
}

#[allow(dead_code)]
impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_unique {
            write!(f, "[ ").unwrap();
            for parameter_element in &self.parameter[0] {
                write!(f, "{} ", &parameter_element).unwrap();
            }
        } else {
            write!(f, "[").unwrap();
            for (i, parameter_set) in self.parameter.iter().enumerate() {
                write!(f, " ({}): [ ", i).unwrap();
                for parameter_element in parameter_set {
                    write!(f, "{} ", parameter_element).unwrap();
                }
                if &parameter_set != &self.parameter.iter().last().unwrap() {
                    write!(f, "],").unwrap();
                } else {
                    write!(f, "]").unwrap();
                }
            }
        }
        // if self.was_set {
        //     write!(f, "] [active]")
        // } else {
        //     write!(f, "]")
        // }
        write!(f, "]")
    }
}
