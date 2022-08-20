use std::collections::BTreeMap;

use color_eyre::{
    eyre::{eyre, ContextCompat},
    Result,
};
use mal::{
    atom::Atom,
    env::{default_env, Env},
};

fn main() -> Result<()> {
    color_eyre::install()?;
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
        Err(e) => return e.to_string(),
    };
    let result = eval(&atom, &env);
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return e.to_string();
        }
    };
    print(result)
}

fn read(s: String) -> Result<Atom> {
    mal::reader::read_str(s)
}

fn eval(ast: &Atom, env: &Env) -> Result<Atom> {
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
                        a => Err(eyre!("expected a function or builtin as first element of list for list evaluation, but got {}, which is invalid", a)),
                    },
                    a => panic!("Expected a list, but got {} (this should never happen)", a),
                }
            }
        }
        a => eval_ast(a, env),
    }
}

fn eval_ast(ast: &Atom, env: &Env) -> Result<Atom> {
    match ast {
        Atom::Symbol(sym) => env
            .get(sym)
            .context(format!("symbol {} is not bound to any value", sym))
            .map(|x| x.clone()),
        Atom::List(lst) => Ok(Atom::List(
            lst.iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<Atom>>>()?,
        )),
        Atom::Vector(lst) => Ok(Atom::Vector(
            lst.iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<Atom>>>()?,
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
    atom.to_string()
}
