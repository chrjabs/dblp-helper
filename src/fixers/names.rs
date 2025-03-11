/// Converts a name of form `<first> <middle> <last>` into `<last>, <first> <middle` while catching
/// some common last names that have more than one word
pub fn fix(name: &mut String) {
    let tokens: Vec<_> = name.split_whitespace().map(String::from).collect();
    name.clear();

    // Case 1: if any but the first token starts with lower case, likely `van ...`, `de la ...`
    for (idx, token) in tokens.iter().enumerate().skip(1) {
        if let Some(first) = token.chars().next() {
            if first.is_lowercase() {
                convert(name, &tokens, idx);
                return;
            }
        }
    }

    // Case 2: if any but the first token is a special word, treat it as the last name start
    for (idx, token) in tokens.iter().enumerate().skip(1) {
        if matches!(token.as_ref(), "Le" | "La" | "Van" | "Von") {
            convert(name, &tokens, idx);
            return;
        }
    }

    convert(name, &tokens, tokens.len() - 1);
}

fn convert(out: &mut String, tokens: &[String], last_name_start: usize) {
    debug_assert!(out.is_empty());
    for (idx, token) in tokens[last_name_start..].iter().enumerate() {
        out.push_str(token);
        if idx + last_name_start + 1 < tokens.len() {
            out.push(' ');
        }
    }
    out.push_str(", ");
    for (idx, token) in tokens[..last_name_start].iter().enumerate() {
        out.push_str(token);
        if idx + 1 < last_name_start {
            out.push(' ');
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        let mut name = String::from("Christoph Jabs");
        super::fix(&mut name);
        assert_eq!(name, "Jabs, Christoph");

        let mut name = String::from("Daniel Le Berre");
        super::fix(&mut name);
        assert_eq!(name, "Le Berre, Daniel");

        let mut name = String::from("Maria Garcia de la Banda");
        super::fix(&mut name);
        assert_eq!(name, "de la Banda, Maria Garcia");
    }
}
