use super::Mul;

pub struct State {
    mul: usize,
    does: usize,
    left: String,
    right: String,
    enable: bool,
}

impl State {
    fn reset(&mut self) {
        self.mul = 0;
        self.does = 0;
        self.left.clear();
        self.right.clear();
        // enable does not change
    }
    fn enable(&mut self) {
        self.reset();
        self.enable = true;
    }
    fn disable(&mut self) {
        self.reset();
        self.enable = false;
    }
    fn current(&self) -> Mul {
        Mul(self.left.parse().unwrap(), self.right.parse().unwrap())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            mul: 0,
            does: 0,
            left: String::new(),
            right: String::new(),
            enable: true, // enabled by default
        }
    }
}

pub trait ValidChars {
    fn valid_chars() -> &'static [char];
}

impl ValidChars for Mul {
    fn valid_chars() -> &'static [char] {
        &['m', 'u', 'l', '(', ',', ')']
    }
}

pub struct Do;
pub struct Dont;

impl ValidChars for Do {
    fn valid_chars() -> &'static [char] {
        &['d', 'o', '(', ')']
    }
}

impl ValidChars for Dont {
    fn valid_chars() -> &'static [char] {
        &['d', 'o', 'n', '\'', 't', '(', ')']
    }
}

pub fn parse(input: &str) -> Vec<Mul> {
    let mut out = vec![];
    let mut state = State::default();
    for char in input.chars() {
        if !char.is_ascii_digit()
            && !Mul::valid_chars().contains(&char)
            && !Do::valid_chars().contains(&char)
            && !Dont::valid_chars().contains(&char)
        {
            state.reset();
            continue;
        }
        match char {
            // do/don't
            'd' => state.does = 1,
            'o' | 'n' | '\'' | 't' | '(' if state.does >= 1 => state.does += 1,
            ')' if state.does == 3 => state.enable(),
            ')' if state.does == 6 => state.disable(),
            // mul
            'm' if state.enable => state.mul = 1,
            'u' | 'l' | '(' if state.mul >= 1 && state.enable => state.mul += 1,
            c if c.is_ascii_digit() && state.mul == 4 && state.enable => state.left.push(c),
            ',' if state.mul == 4 && state.enable => state.mul += 1,
            c if c.is_ascii_digit() && state.mul == 5 && state.enable => state.right.push(c),
            ')' if state.mul == 5 && state.enable => {
                out.push(state.current());
                state.reset();
            }
            _ => continue,
        }
    }
    out
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    use super::parse;
    use crate::Mul;

    #[test]
    fn can_parse() {
        let muls = parse(SAMPLE);
        assert_eq!(muls.len(), 2);
    }

    #[test]
    fn can_multiply() {
        let muls = parse(SAMPLE);
        assert_eq!(muls.iter().map(|Mul(a, b)| a * b).sum::<usize>(), 48);
    }
}
