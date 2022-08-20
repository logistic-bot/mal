use std::collections::BTreeMap;

pub enum RunTimeError {
    SymbolNotBound,
    NotAFunction,
    WrongNumberArguments,
    WrongType,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Atom {
    List(Vec<Atom>),
    Vector(Vec<Atom>),
    Integer(i64),
    Symbol(String),
    Keyword(String),
    String(String),
    HashMap(BTreeMap<Atom, Atom>),
    Builtin(fn(Vec<Atom>) -> Result<Atom, RunTimeError>),
}

impl Atom {
    pub fn as_integer(&self) -> Result<i64, RunTimeError> {
        match self {
            Atom::Integer(num) => Ok(*num),
            _ => Err(RunTimeError::WrongType),
        }
    }
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
            Atom::String(s) => write!(f, r#""{}""#, escape(s)),
            Atom::HashMap(map) => {
                write!(
                    f,
                    "{{{}}}",
                    map.iter()
                        .map(|(k, v)| format!("{} {}", k, v))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Atom::Builtin(b) => write!(f, "#<BUILTIN {:?}>", b),
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
