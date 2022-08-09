use color_eyre::Result;

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
    print(eval(read(s)))
}

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> String {
    s
}
