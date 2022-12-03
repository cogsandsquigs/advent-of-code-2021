use anyhow::Result;

use utils::files::read_file_string;

fn main() -> Result<()> {
    let input = read_file_string("day-10/input.txt")?;

    println!("Part 1 answer: {}", part_1(&input));

    println!("Part 2 answer: {}", part_2(&input));

    Ok(())
}

fn part_2(input: &String) -> usize {
    let lines = input.lines();

    let incompletes: Vec<&str> = lines.filter(|line| find_illegal(line).is_none()).collect();

    let mut scores: Vec<usize> = incompletes
        .into_iter()
        .map(|line| autocomplete_line(line))
        // Score
        .map(|chars| {
            chars.into_iter().fold(0, |acc, x| {
                acc * 5
                    + match x {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Invalid char"),
                    }
            })
        })
        .collect();

    // Sort the scores
    scores.sort();

    // Get the middle score
    scores[scores.len() / 2]
}

/// Autocompletes a line of text
fn autocomplete_line(line: &str) -> Vec<char> {
    // A stack of opening brackets/parens/etc.
    let mut stack: Vec<char> = Vec::new();
    // The chars to return
    let mut chars: Vec<char> = vec![];

    for char in line.chars() {
        match char {
            // If we see an opening bracket, push it onto the stack
            '(' | '[' | '{' | '<' => stack.push(char),
            // If we see a closing bracket, pop the stack and check if it matches
            ')' | ']' | '}' | '>' => {
                // Pop the stack, to remove the closing bracket.
                stack.pop();
            }

            _ => unreachable!("Unexpected character"),
        }
    }

    // Reverse the stack, so that the first element is the first opening bracket
    stack.reverse();

    for char in stack {
        match char {
            '(' => chars.push(')'),
            '[' => chars.push(']'),
            '{' => chars.push('}'),
            '<' => chars.push('>'),
            _ => unreachable!("Unexpected character"),
        }
    }

    chars
}

fn part_1(input: &String) -> usize {
    let lines = input.lines();

    lines
        .map(|line| find_illegal(line))
        .map(|illegal| illegal.map_or(0, score_illegal_char))
        .sum()
}

/// Scores a character based on the scoring rules
fn score_illegal_char(char: char) -> usize {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!("Invalid character"),
    }
}

/// Finds an illegal token in the line, or returns `None` if the line is valid or incomplete.
fn find_illegal(line: &str) -> Option<char> {
    // A stack of opening brackets/parens/etc.
    let mut stack: Vec<char> = Vec::new();

    for char in line.chars() {
        match char {
            // If we see an opening bracket, push it onto the stack
            '(' | '[' | '{' | '<' => stack.push(char),
            // If we see a closing bracket, pop the stack and check if it matches
            ')' | ']' | '}' | '>' => {
                let Some(opening ) = stack.pop() else {
                    // If the stack is empty, we have an extra closing bracket
                    return Some(char);
                };

                if (opening == '(' && char != ')')
                    || (opening == '[' && char != ']')
                    || (opening == '{' && char != '}')
                    || (opening == '<' && char != '>')
                {
                    return Some(char);
                }
            }

            _ => unreachable!("Unexpected character"),
        }
    }

    None
}
