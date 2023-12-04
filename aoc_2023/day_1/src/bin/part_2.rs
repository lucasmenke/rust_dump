use std::io::Error;

fn main() {
    let input = include_str!("./input.txt");
    match process(input) {
        Ok(result) => assert_eq!("281", result), 
        Err(error) => println!("{}", error)
    }
}

fn process(_input: &str) -> Result<String, Error> {
    let result = _input.lines().map(|line| {
        let mod_line = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        
        
    });

    Ok(0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        match process(input) {
            Ok(result) => assert_eq!("281", result), 
            Err(error) => println!("{}", error)
        }
    }
}
