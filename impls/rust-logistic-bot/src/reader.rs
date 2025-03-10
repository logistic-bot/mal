use color_eyre::{
    eyre::{eyre, ContextCompat, WrapErr},
    Result,
};
use std::collections::BTreeMap;

use crate::atom::Atom;

/// Stores the tokens and a position
struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    /// Returns the token at the current position and increments the position.
    fn next(&mut self) -> Option<&str> {
        let ret = self.tokens.get(self.position);
        self.position += 1;
        ret.map(|x| &**x)
    }

    /// Returns the token at the current position
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.position).map(|x| &**x)
    }
}

pub fn read_str(s: String) -> Result<Atom> {
    let tokens = tokenize(s);
    let mut reader = Reader {
        tokens,
        position: 0,
    };
    read_form(&mut reader)
}

fn tokenize(haystack: String) -> Vec<String> {
    let re = regex::Regex::new(
        r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#,
    )
    .unwrap();

    let tokens = haystack
        .match_indices(&re)
        // this works around the regex somehow not correctly trimming ',' or spaces sometimes
        .map(|x| x.1.trim().trim_matches(|x| x == ',').trim().to_string())
        // this filters out empty strings that may have been create in the previous step
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    tokens
}

fn read_form(reader: &mut Reader) -> Result<Atom> {
    let token = reader.next().context("unexpected end of file")?;
    match token
        .chars()
        .next()
        .expect("Tokens should always have at least one character")
    {
        '(' => Ok(Atom::List(
            read_list(reader, ")").context("while reading list")?,
        )),
        '[' => Ok(Atom::Vector(read_list(reader, "]")?)),
        '{' => {
            let lst = read_list(reader, "}")?;
            let mut map = BTreeMap::new();
            let mut lst = lst.into_iter();
            while let Some(key) = lst.next() {
                map.insert(
                    key,
                    lst.next()
                        .context("invalid hashmap: got a key without a value")?,
                );
            }
            Ok(Atom::HashMap(map))
        }
        '\"' => {
            let mut chars = token.chars();
            chars.next();
            if chars.next_back() != Some('"') {
                Err(eyre!("unclosed string"))
            } else {
                let res = unescape(chars.as_str())?;
                Ok(Atom::String(res))
            }
        }
        _ => match token {
            "'" => Ok(Atom::List(vec![
                Atom::Symbol(String::from("quote")),
                read_form(reader)?,
            ])),
            "`" => Ok(Atom::List(vec![
                Atom::Symbol(String::from("quasiquote")),
                read_form(reader)?,
            ])),
            "~" => Ok(Atom::List(vec![
                Atom::Symbol(String::from("unquote")),
                read_form(reader)?,
            ])),
            "~@" => Ok(Atom::List(vec![
                Atom::Symbol(String::from("splice-unquote")),
                read_form(reader)?,
            ])),
            "@" => Ok(Atom::List(vec![
                Atom::Symbol(String::from("deref")),
                read_form(reader)?,
            ])),
            _ => Ok(read_atom(token)),
        },
    }
}

/// inpired by <https://docs.rs/snailquote/latest/src/snailquote/lib.rs.html#231-308/>
fn unescape(s: &str) -> Result<String> {
    let mut chars = s.chars();
    let mut res = String::with_capacity(s.len());

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                None => return Err(eyre!("unfinished escape sequence")),
                Some(c2) => {
                    res.push(match c2 {
                        '"' => '"',
                        '\'' => '\'',
                        '\\' => '\\',
                        _ => return Err(eyre!("unsuported escape sequence")),
                    });
                }
            }
            continue;
        }

        res.push(c)
    }

    Ok(res)
}

fn read_list(reader: &mut Reader, end_marker: &str) -> Result<Vec<Atom>> {
    let mut res = Vec::new();
    loop {
        let token = reader.peek();
        if let Some(token) = token {
            if token == end_marker {
                reader.next();
                break;
            } else {
                res.push(read_form(reader).context("while reading form inside of list")?);
            }
        } else {
            return Err(eyre!(
                "unexpected end of file while reading list (missing '{}')",
                end_marker
            ));
        }
    }
    Ok(res)
}

fn read_atom(token: &str) -> Atom {
    match token.parse::<i64>() {
        Ok(num) => Atom::Integer(num),
        Err(_) => {
            if token.starts_with(':') {
                Atom::Keyword(token.chars().skip(1).collect())
            } else {
                Atom::Symbol(token.to_string())
            }
        }
    }
}
