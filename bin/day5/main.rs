fn form(vec: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut iter = vec.iter().peekable();

    while let Some(&current) = iter.next() {
        if let Some(&&next) = iter.peek() {
            if (current as i16 - next as i16).abs() == 32 {
                iter.next();
                continue;
            }
        }
        result.push(current);
    }

    if result.len() == vec.len() {
        return result;
    } else {
        return form(result);
    }
}

fn remove_type(vec: Vec<u8>) -> usize {
    (b'a'..=b'z')
        .map(|c| {
            let filtered: Vec<u8> = vec
                .iter()
                .filter(|&&x| x != c && x != c - 32)
                .copied()
                .collect();
            form(filtered).len()
        })
        .min()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let polymer: Vec<u8> = input.bytes().collect();
    form(polymer).len()
}

fn part2(input: &str) -> usize {
    let polymer: Vec<u8> = input.bytes().collect();
    remove_type(polymer)
}

fn main() {
    let input = include_str!("input5.txt");
    println!("{}, {}", part1(input), part2(input));
}