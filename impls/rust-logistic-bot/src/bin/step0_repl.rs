fn main() {
    use std::io::BufRead;
    use std::io::Write;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    loop {
        print!("user> ");
        stdout.lock().flush().unwrap();
        let line = stdin.lock().lines().next().unwrap().unwrap();

        println!("{}", read_eval_print(line));
    }
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
