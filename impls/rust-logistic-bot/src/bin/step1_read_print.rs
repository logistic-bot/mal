use color_eyre::Result;
use mal::atom::Atom;

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
    let atom = read(s);
    let atom = match atom {
        Ok(atom) => atom,
        Err(e) => return e.to_string(),
    };
    print(eval(atom))
}

fn read(s: String) -> Result<Atom> {
    mal::reader::read_str(s)
}

fn eval(atom: Atom) -> Atom {
    atom
}

fn print(atom: Atom) -> String {
    dbg!(&atom);
    atom.to_string()
}
