use std::collections::HashMap;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const NROWS: usize = 4;
const NCOLS: usize = 3;

pub fn convert(input: &str) -> Result<String, Error> {
    let nrows = input.lines().count();
    if nrows % NROWS != 0 {
        return Err(Error::InvalidRowCount(nrows));
    }
    let ncols = (input.bytes().count() + 1) / nrows - 1;
    if ncols % NCOLS != 0 {
        return Err(Error::InvalidColumnCount(ncols));
    }
    let hm: HashMap<_, _> = [(" _ ".to_string() +
                              "| |" +
                              "|_|" +
                              "   ", "0".to_string()),
                             ("   ".to_string() +
                              "  |" +
                              "  |" +
                              "   ", "1".to_string()),
                             (" _ ".to_string() +
                              " _|" +
                              "|_ " +
                              "   ", "2".to_string()),
                             (" _ ".to_string() +
                              " _|" +
                              " _|" +
                              "   ", "3".to_string()),
                             ("   ".to_string() +
                              "|_|" +
                              "  |" +
                              "   ", "4".to_string()),
                             (" _ ".to_string() +
                              "|_ " +
                              " _|" +
                              "   ", "5".to_string()),
                             (" _ ".to_string() +
                              "|_ " +
                              "|_|" +
                              "   ", "6".to_string()),
                             (" _ ".to_string() +
                              "  |" +
                              "  |" +
                              "   ", "7".to_string()),
                             (" _ ".to_string() +
                              "|_|" +
                              "|_|" +
                              "   ", "8".to_string()),
                             (" _ ".to_string() +
                              "|_|" +
                              " _|" +
                              "   ", "9".to_string())].into_iter().collect();

    let mut numbers = vec![vec![String::new(); ncols / NCOLS]; nrows / NROWS];
    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            numbers[row / NROWS][col / NCOLS].push(character);
        }
    }

    let mut string = String::new();
    for number in numbers {
        for digit in number {
            string.push_str(match hm.get(&digit) {
                Some(d) => d,
                None => "?",
            });
        }
        string.push(',');
    }
    string.pop();
    Ok(string)
}
