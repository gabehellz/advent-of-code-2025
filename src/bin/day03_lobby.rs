use std::{fs::File, io::Read};

fn find_double_max(line: &str) -> u64 {
    let len = line.len();
    let mut first_max = '0';
    let mut second_max = '0';
    let mut i = 0;

    while len > i {
        let char = line.chars().nth(i).unwrap();

        if i == len - 1 {
            if char > second_max {
                second_max = char;
            }
        } else if char > first_max {
            first_max = char;
            second_max = '0';
        } else if char > second_max {
            second_max = char;
        }

        i += 1;
    }

    format!("{}{}", first_max, second_max)
        .parse::<u64>()
        .unwrap_or_default()
}

fn find_twelve_max(line: &str) -> u64 {
    let len = line.len();
    let mut i = 0;
    let mut chars = Vec::from(['0'; 12]);
    let mut l = 0;
    let mut j = 0;
    let max = 12;
    let mut s = String::new();

    while len > i {
        let char = line.chars().nth(i).unwrap();

        if char > chars[j] {
            chars[j] = char;
            l = i;
        }

        if chars[11] == '0' {
            // println!("{} {} {} {} {:?}", i, j, len, max, chars);
            if i > len - max + j - 1 {
                i = l;
                j += 1;

                if j == max {
                    i = len;
                }
            }
        }

        i += 1;
    }

    for char in chars {
        s.push(char);
    }

    s.parse::<u64>().unwrap_or_default()
}

fn process_part1(input: String) -> u64 {
    input
        .split("\n")
        .map(|v| find_double_max(v.trim()))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn process_part2(input: String) -> u64 {
    input
        .split("\n")
        .map(|v| find_twelve_max(v.trim()))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[test]
fn test_lobby_part1() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_owned();
    let result = process_part1(input);
    assert_eq!(result, 357u64);
}

#[test]
fn test_lobby_part2() {
    let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111".to_owned();
    let result = process_part2(input);
    assert_eq!(result, 3121910778619);
}

fn main() {
    let mut file = {
        let args: Vec<String> = std::env::args().collect();
        let path = &args[1];
        File::open(path).unwrap()
    };

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let result = process_part1(input.clone());
    println!("{:?}", result);

    let result = process_part2(input);
    println!("{:?}", result);
}
