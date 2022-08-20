use std::collections::BTreeMap;

use color_eyre::Result;
use mal::{
    atom::{Atom, RunTimeError},
    env::{default_env, Env},
    reader::ParseError,
};

fn main() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    let _ = rl.load_history(".lisphistory.txt");

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("{}", read_eval_print(line));
            }
            Err(_) => break,
        }
    }

    let _ = rl.save_history(".lisphistory.txt");
    Ok(())
}

fn read_eval_print(s: String) -> String {
    let env = default_env();

    let atom = read(s);
    let atom = match atom {
        Ok(atom) => atom,
        Err(e) => {
            return match e {
                ParseError::UnexpectedEndOfFile => String::from("unexpected end of input"),
                ParseError::Unbalenced => String::from("unbalanced"),
                ParseError::UnfinishedEscapeSequence => String::from("unbalanced"),
                ParseError::UnsuportedEscapeSequence => String::from("unbalanced"),
            }
        }
    };
    let result = eval(&atom, &env);
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return String::from(match e {
                RunTimeError::SymbolNotBound => "symbol not bound",
                RunTimeError::NotAFunction => "not a function",
                RunTimeError::WrongNumberArguments => "wrong number of arguments",
                RunTimeError::WrongType => "wrong type of argument",
            })
        }
    };
    print(result)
}

fn read(s: String) -> Result<Atom, ParseError> {
    mal::reader::read_str(s)
}

fn eval(ast: &Atom, env: &Env) -> Result<Atom, RunTimeError> {
    match ast {
        Atom::List(lst) => {
            if lst.is_empty() {
                Ok(ast.clone())
            } else {
                let lst = eval_ast(ast, env)?;
                match lst {
                    Atom::List(lst) => match lst.first().unwrap() {
                        Atom::Builtin(builtin) => {
                            Ok(builtin(vec![lst[1].clone(), lst[2].clone()])?)
                        }
                        _ => Err(RunTimeError::NotAFunction),
                    },
                    a => panic!("Expected a list, but got {} (this should never happen)", a),
                }
            }
        }
        a => eval_ast(a, env),
    }
}

fn eval_ast(ast: &Atom, env: &Env) -> Result<Atom, RunTimeError> {
    match ast {
        Atom::Symbol(sym) => env
            .get(sym)
            .ok_or(RunTimeError::SymbolNotBound)
            .map(|x| x.clone()),
        Atom::List(lst) => Ok(Atom::List(
            lst.iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<Atom>, RunTimeError>>()?,
        )),
        Atom::Vector(lst) => Ok(Atom::Vector(
            lst.iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<Atom>, RunTimeError>>()?,
        )),
        Atom::HashMap(map) => Ok(Atom::HashMap({
            let mut res = BTreeMap::new();
            for (k, v) in map.iter() {
                res.insert(eval(k, env)?, eval(v, env)?);
            }
            res
        })),
        a => Ok(a.clone()),
    }
}

fn print(atom: Atom) -> String {
    dbg!(&atom);
    atom.to_string()
}
