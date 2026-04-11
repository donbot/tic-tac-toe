use std::io::BufRead;

pub fn get_move<R: BufRead>(reader: &mut R) -> Option<usize> {
    let mut input = String::new();
    reader.read_line(&mut input).ok()?;

    input
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|&n| n >= 1 && n <= 9)
        .map(|n| n - 1)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_get_move_returns_some_number() {
        let mut input = Cursor::new("5\n");
        let result = get_move(&mut input);

        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_get_move_returns_none() {
        let scenarios = [("Out of Bounds", "9000\n"), ("Invalid Input", "potato\n")];

        for (name, input_text) in scenarios {
            let mut input = Cursor::new(input_text);
            let result = get_move(&mut input);
            assert_eq!(result, None, "Failed in Scenario {}", name);
        }
    }
}
