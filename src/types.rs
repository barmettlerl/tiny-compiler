use std::fmt::{Display, Formatter, self};

pub enum Type {
    Nat,
    Bool,
    Arrow(Box<Type>, Box<Type>),
}

impl Clone for Type {
    fn clone(&self) -> Self {
        match self {
            Type::Nat => Type::Nat,
            Type::Bool => Type::Bool,
            Type::Arrow(T1, T2) => Type::Arrow(T1.clone(), T2.clone()),
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Nat, Type::Nat) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Arrow(T1, T2), Type::Arrow(T3, T4)) => T1 == T3 && T2 == T4,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Nat => write!(f, "Nat"),
            Type::Bool => write!(f, "Bool"),
            Type::Arrow(T1, T2) => write!(f, "({} -> {})", T1, T2),
        }
    }
}