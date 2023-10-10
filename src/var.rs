use std::f64::consts::TAU;
use std::fmt;

pub trait VarTypes: Sized {
    fn get(var: &Var) -> Option<Self>;
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum Var<'a> {
    Switch(bool),
    Word(&'a str),
    UInt(u32),
    Int(i32),
    UFloat(f64),
    Float(f64),
    Phase(f64),
    Turn(f64),
}

#[allow(dead_code)]
impl<'a> Var<'a> {
    pub fn new(var: Var<'a>) -> Self {
        match var {
            Var::Switch(value) => Var::Switch(value.to_owned()),
            Var::Word(word) => Var::Word(word),
            Var::UInt(value) => Var::UInt(value.to_owned()),
            Var::Int(value) => Var::Int(value.to_owned()),
            Var::Float(value) => Var::Float(value.to_owned()),
            Var::UFloat(value) => {
                if value >= 0.0 {
                    Var::UFloat(value.to_owned())
                } else {
                    Var::UFloat(0.0)
                }
            }
            Var::Phase(value) => Var::Phase((((value % 1.0) + 1.0) % 1.0).to_owned()),
            Var::Turn(value) => Var::Turn((((value % TAU) + TAU) % TAU).to_owned()),
        }
    }

    pub fn try_set(&mut self, string: &'a str) -> Self {
        match self {
            Var::Switch(_) => Var::Switch(string.parse::<bool>().expect("could not parse `bool`")),
            Var::Word(_) => Var::Word(string),
            Var::UInt(_) => Var::UInt(string.parse::<u32>().expect("could not parse `u32`")),
            Var::Int(_) => Var::Int(string.parse::<i32>().expect("could not parse `i32`")),
            Var::Float(_) => Var::Float(string.parse::<f64>().expect("could not parse `f64`")),
            Var::UFloat(_) => Var::UFloat(string.parse::<f64>().expect("could not parse `+f64`")),
            Var::Phase(_) => {
                let mut phase_value: f64 = string.parse::<f64>().expect("could not parse `f64`");
                if phase_value < 0.0 || phase_value >= TAU {
                    println!("phase value outside range 0..2Pi, so it was wrapped around.");
                    phase_value = ((phase_value % TAU) + TAU) % TAU;
                };
                Var::Phase(phase_value)
            }
            Var::Turn(_) => {
                let mut turn_value: f64 = string.parse::<f64>().expect("could not parse `i32`");
                if turn_value < 0.0 || turn_value >= 1.0 {
                    println!("turn value outside range 0..1, so it was wrapped around.");
                    turn_value = ((turn_value % TAU) + TAU) % TAU;
                };
                Var::Phase(turn_value)
            }
        }
    }

    pub fn try_parse(&mut self, string: &'a str) {
        match self {
            Var::Switch(val) => *val = string.parse::<bool>().expect("could not parse `bool`"),
            Var::Word(val) => *val = string,
            Var::UInt(val) => *val = string.parse::<u32>().expect("could not parse `u32`"),
            Var::Int(val) => *val = string.parse::<i32>().expect("could not parse `i32`"),
            Var::Float(val) => *val = string.parse::<f64>().expect("could not parse `f64`"),
            Var::UFloat(val) => {
                let value = string.parse::<f64>().expect("could not parse `+f64`");
                if value <= 0.0 {
                    *val = -value
                } else {
                    *val = value
                };
            }
            Var::Phase(val) => {
                let mut phase_value: f64 = string
                    .parse::<f64>()
                    .expect("could not parse `0.0 <= f64 < 2Pi`");
                if phase_value < 0.0 || phase_value >= TAU {
                    println!("phase value outside range 0..2Pi, so it was wrapped around.");
                    phase_value = ((phase_value % TAU) + TAU) % TAU;
                };
                *val = phase_value;
            }
            Var::Turn(val) => {
                let mut turn_value: f64 = string.parse::<f64>().expect("could not parse `i32`");
                if turn_value < 0.0 || turn_value >= 1.0 {
                    println!("turn value outside range 0..1, so it was wrapped around.");
                    turn_value = ((turn_value % TAU) + TAU) % TAU;
                };
                *val = turn_value;
            }
        }
    }
    pub fn get<T>(&self) -> Option<T>
    where
        T: VarTypes + Default,
    {
        T::get(self)
    }
}

impl VarTypes for bool {
    fn get(var: &Var) -> Option<Self> {
        match *var {
            Var::Switch(v) => Some(v),
            _ => None,
        }
    }
}

impl VarTypes for u32 {
    fn get(var: &Var) -> Option<Self> {
        match *var {
            Var::UInt(v) => Some(v),
            _ => None,
        }
    }
}

impl VarTypes for i32 {
    fn get(var: &Var) -> Option<Self> {
        match *var {
            Var::Int(v) => Some(v),
            _ => None,
        }
    }
}

impl VarTypes for f64 {
    fn get(var: &Var) -> Option<Self> {
        match *var {
            Var::UFloat(v) | Var::Float(v) | Var::Phase(v) | Var::Turn(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> fmt::Debug for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Var::Switch(value) => f
                .debug_struct("Var")
                .field("switch [true/false]", value)
                .finish(),
            Var::Word(value) => f.debug_struct("Var").field("Word", value).finish(),
            Var::UInt(value) => f
                .debug_struct("Var")
                .field("unsigned int [0..]", value)
                .finish(),
            Var::Int(value) => f.debug_struct("Var").field("int", value).finish(),
            Var::UFloat(value) => f
                .debug_struct("Var")
                .field("positive float [0.0..)", value)
                .finish(),
            Var::Float(value) => f.debug_struct("Var").field("float", value).finish(),
            Var::Phase(value) => f
                .debug_struct("Var")
                .field("phase [0..2Pi)", value)
                .finish(),
            Var::Turn(value) => f
                .debug_struct("Var")
                .field("turn [0.0..1.0)", value)
                .finish(),
        }
    }
}

impl<'a> fmt::Display for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Var::Switch(value) => write!(f, "{}", value),
            Var::Word(value) => write!(f, "{}", value),
            Var::UInt(value) => write!(f, "{}", value),
            Var::Int(value) => write!(f, "{}", value),
            Var::Float(value) => write!(f, "{}", value),
            Var::UFloat(value) => write!(f, "{}", value),
            Var::Phase(value) => write!(f, "{}", value),
            Var::Turn(value) => write!(f, "{}", value),
        }
    }
}
