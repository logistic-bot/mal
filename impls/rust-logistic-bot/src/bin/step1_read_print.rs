use color_eyre::Result;
use mal::{atom::Atom, reader::ParseError};

fn main() -> Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    let _ = rl.load_history(".lisphistory.txt");

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                println!("{}", read_eval_print(line));
            }
            Err(_) => break,
        }
    }

    Ok(())
}

fn read_eval_print(s: String) -> String {
    let atom = read(s);
    let atom = match atom {
        Ok(atom) => atom,
        Err(e) => {
            return match e {
                ParseError::UnexpectedEndOfFile => String::from("unexpected end of input"),
                ParseError::UnbalencedParenthesis => String::from("unbalanced parenthesis"),
            }
        }
    };
    print(eval(atom))
}

fn read(s: String) -> Result<Atom, ParseError> {
    mal::reader::read_str(s)
}

fn eval(atom: Atom) -> Atom {
    atom
}

fn print(atom: Atom) -> String {
    atom.to_string()
}
