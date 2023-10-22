use colored::Colorize;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

// use crate::{parameter::Parameter, var::Var};

#[allow(dead_code)]
pub struct GlobalParameterMap<'a> {
    map_map: HashMap<&'static str, HashMap<&'static str, Parameter<'a>>>,
}

#[allow(dead_code)]
impl<'a> GlobalParameterMap<'a> {
    pub fn new() -> Self {
        GlobalParameterMap {
            map_map: HashMap::<&'static str, HashMap<&'static str, Parameter>>::new(),
        }
    }

    pub fn create_domain(&mut self, domain: &'static str) {
        if !self.map_map.contains_key(domain) {
            self.map_map
                .insert(domain, HashMap::<&'static str, Parameter>::new());
        }
    }

    pub fn insert_into_domain(
        &mut self,
        domain: &'static str,
        variable_name: &'static str,
        default_parameter: Vec<Var<'a>>,
        set_by: Vec<&'static str>,
        explanation: &'static str,
        is_unique: bool,
    ) {
        if self.map_map.contains_key(domain) {
            self.map_map.get_mut(domain).unwrap().insert(
                &variable_name,
                Parameter {
                    parameter: vec![default_parameter],
                    set_by,
                    explanation,
                    was_set: false,
                    is_unique,
                },
            );
        }
        // if let Some(sub_map) = &mut self.map_map.get(domain) {
        // }
        //  else {
        //     &mut self
        //         .map_map
        //         .insert(&domain, HashMap::<&'static str, Parameter>::new());

        // if let Some(sub_map) = &mut self.map_map.get(domain) {
        //     sub_map
        // }
        // }
    }

    pub fn insert(
        &mut self,
        domain: &'static str,
        variable_name: &'static str,
        default_parameter: Vec<Var<'a>>,
        set_by: Vec<&'static str>,
        explanation: &'static str,
        is_unique: bool,
    ) {
        if self.map_map.contains_key(&domain) {
            // insert into sub_map
            // if is_unique {}
            self.map_map.get_mut(&domain).unwrap().insert(
                &variable_name,
                Parameter {
                    parameter: vec![default_parameter],
                    set_by,
                    explanation,
                    was_set: false,
                    is_unique,
                },
            );
        } else {
            // create sub_map
            self.map_map
                .insert(&domain, HashMap::<&'static str, Parameter>::new());
            self.map_map.get_mut(&domain).unwrap().insert(
                &variable_name,
                Parameter {
                    parameter: vec![default_parameter],
                    set_by,
                    explanation,
                    was_set: false,
                    is_unique,
                },
            );
        }
    }

    pub fn set_parameter(parameter: &Parameter) {
        print!("explanation: ");
        println!("{}", parameter.explanation);
        // if let set_strings = &parameter.set_by {
        //     print!("{} possible set strings: [", set_strings.len());
        //     for set_by in set_strings {
        //         print!("-{} ", &set_by);
        //     }
        //     println!("]")
        // }
    }

    pub fn get<'b, 'c>(&'b self, domain_name: &'c str, parameter_name: &'c str) -> &'c Parameter
    where
        'b: 'c,
    {
        if let Some(domain_map) = self.map_map.get(&domain_name) {
            if let Some(parameter) = domain_map.get(&parameter_name) {
                parameter
            } else {
                panic!("parameter `{}` could not be found in global_parameter_map. Did you forget to put it there, or misspell it?", &parameter_name);
                // None
            }
        } else {
            panic!("domain `{}` could not be found in global_parameter_map. Did you forget to put it there, or misspell it?", &domain_name);
            // None
        }
    }

    pub fn set_from_args(&mut self, args: &'a Vec<String>) {
        for arg_index in 0..args.len() {
            let domain_string = format!("{}", &args[arg_index].as_str());
            if let Some(sub_map) = self.map_map.get_mut(domain_string.as_str()) {
                println!("setting up domain `{}`:", &domain_string);
                for (_, parameter) in sub_map.iter_mut() {
                    parameter.parse_parameter(args, arg_index + 1);
                }
            }
        }
        // println!();
    }
}

// argh...
// die key=("domain", "parameter_name")-lösung ist doch nicht so toll glaube ich.
// meine print-methode (folgnd) kann ich nicht so einfach schreiben, dass sie die keys nach domain sortiert

#[allow(dead_code)]
impl<'a> fmt::Display for GlobalParameterMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+++ commandline_options +++")?;
        for domain_key in self.map_map.keys().sorted() {
            // for (domain_key, sub_map) in &self.map_map {
            let domain_string = format!(" {} ", domain_key).bold().reversed().dimmed();
            write!(f, "\n{}", domain_string)?;
            for value_key in self.map_map[domain_key].keys().sorted() {
                let sub_value = &self.map_map[domain_key][value_key];
                write!(f, "\n  - {} = ", value_key)?;

                // if sub_value.parameter.len() > 1 {
                //     let br_open = "[".dimmed();
                //     write!(f, "{}", br_open)?;
                // }

                let value = if sub_value.was_set {
                    format!("{}", sub_value).bold().green()
                } else {
                    format!("{}", sub_value).normal()
                };
                write!(f, "{}", value).unwrap();

                // if sub_value.parameter.len() > 1 {
                //     let br_close = "]".dimmed();
                //     write!(f, "{}", br_close)?;
                // }
                write!(f, ", can be set via [ ")?;

                for i in &sub_value.set_by {
                    let word = format!("-{}", i);
                    write!(f, "\'{}\' ", word)?;
                }
                write!(f, "] ")?;
                let explanation = format!("{}", sub_value.explanation).dimmed();
                write!(f, "\n        {}", explanation)?;
            }
        }
        Ok(())
    }
}
