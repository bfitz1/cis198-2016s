use std::{io, result};
use rand::prelude::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`)
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

pub struct Stack(Vec<Elt>);

pub type Result<T> = result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack(Vec::new())
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        self.0.push(val);
        Ok(())
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        match self.0.pop() {
            None => Err(Error::Underflow),
            Some(elt) => Ok(elt),
        }
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                match (self.pop(), self.pop()) {
                    (_, Err(_)) => Err(Error::Underflow),
                    (Ok(Elt::Int(x)), Ok(Elt::Int(y))) =>
                        self.push(Elt::Int(x + y)),
                    _ => Err(Error::Type),
                }
            },
            Op::Eq => {
                match (self.pop(), self.pop()) {
                    (_, Err(_)) => Err(Error::Underflow),
                    (Ok(Elt::Int(x)), Ok(Elt::Int(y))) =>
                        self.push(Elt::Bool(x == y)),
                    (Ok(Elt::Bool(x)), Ok(Elt::Bool(y))) =>
                        self.push(Elt::Bool(x == y)),
                    _ => Err(Error::Type),
                }
            },
            Op::Neg => {
                match self.pop() {
                    Ok(Elt::Int(x)) =>
                        self.push(Elt::Int(-x)),
                    Ok(Elt::Bool(x)) =>
                        self.push(Elt::Bool(!x)),
                    _ => Err(Error::Underflow),
                }
            },
            Op::Swap => {
                match (self.pop(), self.pop()) {
                    (Ok(x), Ok(y)) => {
                        self.push(x).ok();
                        self.push(y).ok();
                        Ok(())
                    },
                    _ => Err(Error::Underflow),
                }
            },
            Op::Rand => {
                match self.pop() {
                    Ok(Elt::Int(x)) if x > 0 => {
                        let mut rng = thread_rng();
                        let r = rng.gen_range(0, x);
                        self.push(Elt::Int(r))
                    },
                    _ => Err(Error::Type),
                }

            }
            Op::Quit => Err(Error::Quit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}