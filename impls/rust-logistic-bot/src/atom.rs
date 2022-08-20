#[derive(Clone, Debug)]
pub enum Atom {
    List(Vec<Atom>),
    Vector(Vec<Atom>),
    Integer(i64),
    Symbol(String),
    Keyword(String),
    String(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Symbol(sym) => write!(f, "{}", sym),
            Atom::Keyword(sym) => write!(f, ":{}", sym),
            Atom::Integer(num) => write!(f, "{}", num),
            Atom::List(list) => write!(
                f,
                "({})",
                list.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Atom::Vector(list) => write!(
                f,
                "[{}]",
                list.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Atom::String(s) => write!(f, "\"{}\"", escape(s)),
        }
    }
}

/// inspired by: <https://docs.rs/snailquote/latest/src/snailquote/lib.rs.html#231-308/>
fn escape(s: &str) -> String {
    let mut output = String::with_capacity(s.len());
    for c in s.chars() {
        let escape = match c {
            '"' => true,
            '\\' => true,
            _ => false,
        };
        if escape {
            output.push('\\');
        }
        output.push(c);
    }
    output
}
