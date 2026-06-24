use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpBin {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpUn {
    Sin,
    Cos,
    Tan,
    Ctg,
    Int,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

fn float_eq(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 0.000000000001;
    (a - b).abs() < EPSILON
}

impl OpBin {
    pub fn apply(self, a: Number, b: Number) -> Option<Number> {
        match self {
            OpBin::Add => Some(a + b),
            OpBin::Sub => Some(a - b),
            OpBin::Mul => Some(a * b),
            OpBin::Div => a / b,
        }
    }
}

impl OpUn {
    pub fn apply(self, a: Number) -> Option<Number> {
        use super::Number::*;

        let a = match a {
            Float(v) => v,
            Int(v) => v as f64,
        };

        let v = match self {
            OpUn::Sin => a.sin(),
            OpUn::Cos => a.cos(),
            OpUn::Tan => a.tan(),
            OpUn::Ctg => {
                let tan =  a.tan();
                if float_eq(tan, 0.0) {
                    return None;
                }
                1.0 / tan
            },
            OpUn::Int => return Some(Number::Int(a as i64)),
        };

        Some(Float(v))
    }
}

impl FromStr for OpUn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(OpUn::Sin),
            "cos" => Ok(OpUn::Cos),
            "tan" => Ok(OpUn::Tan),
            "ctg" => Ok(OpUn::Ctg),
            "int" => Ok(OpUn::Int),
            _ => Err(()),
        }
    }
}

impl FromStr for OpBin {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OpBin::Add),
            "-" => Ok(OpBin::Sub),
            "*" => Ok(OpBin::Mul),
            "/" => Ok(OpBin::Div),
            _ => Err(()),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Float(a), Float(b)) => Float(a + b),
            (Int(a), Int(b)) => Int(a + b),
            (Float(a), Int(b)) => Float(a + (b as f64)),
            (Int(a), Float(b)) => Float((a as f64) + b),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Float(a), Float(b)) => Float(a - b),
            (Int(a), Int(b)) => Int(a - b),
            (Float(a), Int(b)) => Float(a - (b as f64)),
            (Int(a), Float(b)) => Float((a as f64) - b),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
            (Float(a), Float(b)) => Float(a * b),
            (Int(a), Int(b)) => Int(a * b),
            (Float(a), Int(b)) => Float(a * (b as f64)),
            (Int(a), Float(b)) => Float((a as f64) * b),
        }
    }
}

impl Div for Number {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        use Number::*;
        match other {
            Number::Int(0) => return None,
            Number::Float(0.0) => return None,
            _ => (),
        }

        let res = match (self, other) {
            (Float(a), Float(b)) => Float(a / b),
            (Int(a), Int(b)) => Float((a as f64) / (b as f64)),
            (Float(a), Int(b)) => Float(a / (b as f64)),
            (Int(a), Float(b)) => Float((a as f64) / b),
        };

        Some(res)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(a) => write!(f, "{}", a),
            Number::Float(a) => write!(f, "{}f", a),
        }
    }
}
