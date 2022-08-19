#[derive(Clone, Debug)]
pub enum Atom {
    List(Vec<Atom>),
    Vector(Vec<Atom>),
    Integer(i64),
    Symbol(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Symbol(sym) => write!(f, "{}", sym),
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
        }
    }
}
