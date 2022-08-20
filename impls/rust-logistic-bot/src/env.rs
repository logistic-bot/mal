use std::collections::BTreeMap;

use color_eyre::eyre::eyre;

use crate::atom::Atom;

pub type Env = BTreeMap<String, Atom>;

pub fn default_env() -> Env {
    let mut env: Env = BTreeMap::new();

    env.insert(
        String::from("+"),
        Atom::Builtin(|args| {
            if args.len() != 2 {
                Err(eyre!(
                    "Expected exactly 2 arguments, got {}. Args: {}",
                    args.len(),
                    Atom::List(args)
                ))
            } else {
                let num1 = args[0].as_integer()?;
                let num2 = args[1].as_integer()?;
                Ok(Atom::Integer(num1 + num2))
            }
        }),
    );
    env.insert(
        String::from("*"),
        Atom::Builtin(|args| {
            if args.len() != 2 {
                Err(eyre!(
                    "Expected exactly 2 arguments, got {}. Args: {}",
                    args.len(),
                    Atom::List(args)
                ))
            } else {
                let num1 = args[0].as_integer()?;
                let num2 = args[1].as_integer()?;
                Ok(Atom::Integer(num1 * num2))
            }
        }),
    );
    env.insert(
        String::from("-"),
        Atom::Builtin(|args| {
            if args.len() != 2 {
                Err(eyre!(
                    "Expected exactly 2 arguments, got {}. Args: {}",
                    args.len(),
                    Atom::List(args)
                ))
            } else {
                let num1 = args[0].as_integer()?;
                let num2 = args[1].as_integer()?;
                Ok(Atom::Integer(num1 - num2))
            }
        }),
    );
    env.insert(
        String::from("/"),
        Atom::Builtin(|args| {
            if args.len() != 2 {
                Err(eyre!(
                    "Expected exactly 2 arguments, got {}. Args: {}",
                    args.len(),
                    Atom::List(args)
                ))
            } else {
                let num1 = args[0].as_integer()?;
                let num2 = args[1].as_integer()?;
                Ok(Atom::Integer(num1 / num2))
            }
        }),
    );
    env.insert(
        String::from("%"),
        Atom::Builtin(|args| {
            if args.len() != 2 {
                Err(eyre!(
                    "Expected exactly 2 arguments, got {}. Args: {}",
                    args.len(),
                    Atom::List(args)
                ))
            } else {
                let num1 = args[0].as_integer()?;
                let num2 = args[1].as_integer()?;
                Ok(Atom::Integer(num1 % num2))
            }
        }),
    );

    env
}
