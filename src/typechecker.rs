use crate::{types::Type, term::Term::{*, self}};


pub fn check(term: Term) -> Result<Type, String> {
    let mut ctx = Context::new();
    let mut typechecker = Typechecker::new(&mut ctx);
    typechecker.check(term)
}

type Context = Vec<(String, Type)>;
 
struct Typechecker<'a> {
    ctx: &'a mut Context,
}

impl Typechecker<'_> {
    fn new(ctx: &mut Context) -> Typechecker {
        Typechecker { ctx }
    }

    fn check(&mut self, term: Term) -> Result<Type, String> {
        match term {
            Var(var, _t) => {
                let var_type = self.ctx.iter().find(|(x, _)| x == &var).map(|(_, T)| T);
                match var_type {
                    Some(_t) => Ok(_t.clone()),
                    None => Err(format!("Variable {} not found in context", var)),
                }
            }
            Abs(var, t, term) => {
                self.ctx.push((var.clone(), t.clone()));
                let term_type = self.check(*term)?;
                self.ctx.pop();
                Ok(Type::Arrow(Box::new(t), Box::new(term_type)))
            }
            App(term1, term2) => {
                let term1_type = self.check(*term1)?;
                let term2_type = self.check(*term2)?;
                match term1_type {
                    Type::Arrow(t1, t2) => {
                        if *t1 == term2_type {
                            Ok(*t2)
                        } else {
                            Err(format!(
                                "Expected type {} but found type {}",
                                t1, term2_type
                            ))
                        }
                    }
                    _ => Err(format!("Expected type arrow but found type {}", term1_type)),
                }
            }
            Zero => Ok(Type::Nat),
            Succ(term) => {
                let term_type = self.check(*term)?;
                match term_type {
                    Type::Nat => Ok(Type::Nat),
                    _ => Err(format!("Expected type Nat but found type {}", term_type)),
                }
            }
            Pred(term) => {
                let term_type = self.check(*term)?;
                match term_type {
                    Type::Nat => Ok(Type::Nat),
                    _ => Err(format!("Expected type Nat but found type {}", term_type)),
                }
            }
            IsZero(term) => {
                let term_type = self.check(*term)?;
                match term_type {
                    Type::Nat => Ok(Type::Bool),
                    _ => Err(format!("Expected type Nat but found type {}", term_type)),
                }
            }
            True => Ok(Type::Bool),
            False => Ok(Type::Bool),
    }
}

}