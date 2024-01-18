use std::{fmt::{Display, self, Formatter}, vec};

use crate::types::Type;

pub enum NV {
    Zero,
    Succ(Box<NV>),
}

pub enum Term {
    Succ(Box<Term>),
    Zero,
    Pred(Box<Term>),
    IsZero(Box<Term>),
    True,
    False,
    Var(String, Type),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn free_variables(&self) -> Vec<String> {
        match self {
            Term::Var(var, _) => vec![var.clone()],
            Term::Abs(var, _, term) => {
                let mut free_vars = term.free_variables();
                free_vars.retain(|x| x != var);
                free_vars
            }
            Term::App(term1, term2) => {
                let mut free_vars = term1.free_variables();
                free_vars.append(&mut term2.free_variables());
                free_vars
            }
            _ => vec![],
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Term::Var(_, T) => T.clone(),
            Term::Abs(_, T, _) => T.clone(),
            Term::App(term1, _) => match term1.get_type() {
                Type::Arrow(_, T2) => *T2,
                _ => panic!("Expected type arrow"),
            },
            Term::Zero => Type::Nat,
            Term::Succ(term) => match term.get_type() {
                Type::Nat => Type::Nat,
                _ => panic!("Expected type Nat"),
            },
            Term::Pred(term) => match term.get_type() {
                Type::Nat => Type::Nat,
                _ => panic!("Expected type Nat"),
            },
            Term::IsZero(term) => match term.get_type() {
                Type::Nat => Type::Bool,
                _ => panic!("Expected type Nat"),
            },
            Term::True => Type::Bool,
            Term::False => Type::Bool,
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Term::Var(var, _) => write!(f, "{}", var),
            Term::Abs(var, _, term) => write!(f, "(λ{}. {})", var, term),
            Term::App(term1, term2) => write!(f, "({} {})", term1, term2),
            Term::Zero => write!(f, "0"),
            Term::Succ(term) => write!(f, "succ {}", term),
            Term::Pred(term) => write!(f, "pred {}", term),
            Term::IsZero(term) => write!(f, "iszero {}", term),
            Term::True => write!(f, "true"),
            Term::False => write!(f, "false"),
        }
    }
}

impl Clone for Term {
    fn clone(&self) -> Self {
        match self {
            Term::Var(var, T) => Term::Var(var.clone(), T.clone()),
            Term::Abs(var, T, term) => Term::Abs(var.clone(), T.clone(), term.clone()),
            Term::App(term1, term2) => Term::App(term1.clone(), term2.clone()),
            Term::Zero => Term::Zero,
            Term::Succ(term) => Term::Succ(term.clone()),
            Term::Pred(term) => Term::Pred(term.clone()),
            Term::IsZero(term) => Term::IsZero(term.clone()),
            Term::True => Term::True,
            Term::False => Term::False,
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::Var(var1, T1), Term::Var(var2, T2)) => var1 == var2 && T1 == T2,
            (Term::Abs(var1, T1, term1), Term::Abs(var2, T2, term2)) => {
                var1 == var2 && T1 == T2 && term1 == term2
            }
            (Term::App(term1, term2), Term::App(term3, term4)) => term1 == term3 && term2 == term4,
            (Term::Zero, Term::Zero) => true,
            (Term::Succ(term1), Term::Succ(term2)) => term1 == term2,
            (Term::Pred(term1), Term::Pred(term2)) => term1 == term2,
            (Term::IsZero(term1), Term::IsZero(term2)) => term1 == term2,
            (Term::True, Term::True) => true,
            (Term::False, Term::False) => true,
            _ => false,
        }
    }
}