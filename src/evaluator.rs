use crate::term::{Term, Term::*};

fn is_value(term: &Term) -> bool {
    match term {
        Abs(..) => true,
        True => true,
        False => true,
        Zero => true,
        Succ(t) => is_value(t),
        _ => false,
    }
}

// call by value strategy
pub fn eval(term: Term) -> Term {
    match term.clone() {
        App(lhs, rhs) => {
            if !is_value(&lhs) {
                println!("lhs: {}", lhs);
                let lhs_eval = eval(*lhs);
                eval(App(Box::new(lhs_eval), rhs))
            } else if !is_value(&rhs) {
                println!("rhs: {}", rhs);
                let rhs_eval = eval(*rhs);
                eval(App(lhs, Box::new(rhs_eval)))
            } else {
                match *lhs {
                    Abs(var, _, body) => eval(substitute(*body, &var, *rhs)),
                    _ => panic!("Expected Abs"),
                }
                
            }
        }
        Succ(t) => Succ(Box::new(eval(*t))),        
        Pred(t) if *t == Zero => Zero,
        Pred(t) => match *t {
            Succ(nv) => *nv,
            _ => Pred(Box::new(eval(*t)))
        }
        IsZero(t) if *t == Zero => True,
        IsZero(t) if matches!(*t, Succ(_)) => False,
        IsZero(t) => {
            let t_eval = eval(*t);
            eval(IsZero(Box::new(t_eval)))
        }
        Zero => Zero,
        True => True,
        False => False,
        _ if is_value(&term) => term,
        _ => panic!("Unrecognized term: {}", term)
    }
}

fn substitute(term: Term, var: &str, new_term: Term) -> Term {
    match term.clone() {
        Var(x, _) if x == var => new_term,
        Var(..) => term.clone(),
        Abs(x, T, e) if x == var => term.clone(),
        Abs(x, T, e) => Abs(x, T, Box::new(substitute(*e, var, new_term.clone()))),
        App(e1, e2) => App(
            Box::new(substitute(*e1, var, new_term.clone())),
            Box::new(substitute(*e2, var, new_term)),
        ),
        Zero => Zero,
        Succ(e) => Succ(Box::new(substitute(*e, var, new_term))),
        Pred(e) => Pred(Box::new(substitute(*e, var, new_term))),
        IsZero(e) => IsZero(Box::new(substitute(*e, var, new_term))),
        True => True,
        False => False,
    }
}