extern crate fos_fun;
use fos_fun::{term::Term, tokenizer::tokenizer, types::Type};
use rand::Rng;
use std::{io, iter::Zip, process::id};


fn g(var: &str) -> String {
    let mut new_var = var.to_string();
    let mut rng = rand::thread_rng();
    new_var.push_str("_");
    new_var.push_str(rng.gen::<u32>().to_string().as_str());
    new_var
}

fn app(t1: Term, t2: Term) -> Term {
    Term::App(Box::new(t1), Box::new(t2))
}

fn abs(var: String, t: Type, term: Term) -> Term {
    Term::Abs(var.to_string(), t, Box::new(term))
}

fn var(var: String, t: Type) -> Term {
    Term::Var(var.to_string(), t)
}

// fn nr(n: u32) -> Term {
//     let s = g("s");
//     let z = g("z");
//     let mut term = Zero;
//     for _ in 0..n {
//         term = app(succ(), term);
//     }
//     abs(z, abs(s, term))
// }


// fn and() -> Term {
//     let b: String = g("b");
//     let c = g("c");
//     abs(
//         b.clone(),
//         abs(
//             c.clone(),
//             app(
//                 app(var(b.clone()), var(c.clone())),
//                 fls(),
//             ),
//         ),
//     )
// }

// fn is_equal() -> Term {
//     let m = g("m");
//     let n = g("n");

//     abs(
//         m.clone(),
//         abs(
//             n.clone(),
//             app(
//                 app( and(), app(app(m,))
//             ),
//         ),
//     )
// }

// fn add(t1: Term, t2: Term) -> Term {
//     let a = g("a");
//     let b = g("b");
//     let z = g("z");
//     let y = g("y");

//     app(
//         app(
//             abs(
//                 a.clone(),
//                 Type::Nat,
//                 abs(
//                     b.clone(),
//                     Type::Nat,
//                     abs(
//                         z.clone(),
//                         Type::Nat,
//                         app(
//                             app(
//                                 var(a.clone()),
//                                 var(z.clone()),
//                             ),
//                             app(
//                                 app(
//                                     var(b.clone()),
//                                     var(z.clone()),
//                                 ),
//                                 var(z.clone()),
//                             ),
//                         ),
//                     ),
//                 ),
//             ),
//             t1,
//         ),
//         t2,
//     )
// }

fn main() {
    // get use input
    // println!("Enter lambda calculus: ");
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // tokenize
    // let tokens = tokenizer(&input);
    // for token in &tokens {
    //     println!("{}", token);
    // }
    // let ast = fos_fun::parser::parse(&tokens).unwrap();

    // let ast = Add(nr(1), nr(1));
    let ast = app(abs(
        "x".to_string(),
        Type::Nat,
        Term::IsZero(Box::new(Term::Var("x".to_string(), Type::Nat))),
        ),
    Term::Zero);

    // let ast = app(succ(), nr(1));
    println!("AST: {}", ast);
    let res = fos_fun::evaluator::eval(ast);

    println!("res: {}", res);
}
