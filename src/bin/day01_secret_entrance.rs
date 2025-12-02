use std::{fs::File, io::Read};

fn convert(input: &str) -> i32 {
    if input.is_empty() {
        return 0;
    }

    let value = input[1..input.len()].parse::<i32>();

    match input.chars().next() {
        Some('L') => -value.unwrap_or_default(),
        Some('R') => value.unwrap_or_default(),
        Some(_) | None => unreachable!(),
    }
}

fn count_zeros(values: &[i32]) -> i32 {
    let mut temp = 50i32;
    let mut counter = 0;

    for value in values {
        let value = if *value < 0 {
            (*value) % -100
        } else {
            (*value) % 100
        };

        temp = match temp + value {
            result if result < 0 => 100 + result,
            result if result > 99 => result % 100,
            result => result,
        };

        if temp == 0 {
            counter += 1;
        }
    }

    counter
}

fn count_all_zeros(values: &[i32]) -> i32 {
    let mut temp = 50i32;
    let mut counter = 0;

    for value in values {
        let value = *value;
        let result = temp + value;

        if value > 0 {
            counter += result / 100;
            temp = result % 100;
        } else {
            counter += (((100 - temp) % 100) + i32::abs(value)) / 100;
            temp = result.rem_euclid(100);
        }
    }

    counter
}

#[test]
fn test_secret_entrance_part1() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
    let values = input.split("\n").map(convert).collect::<Vec<i32>>();
    let password = count_zeros(&values);
    assert_eq!(password, 3i32);
}

#[test]
fn test_secret_entrance_part2() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
    let values = input.split("\n").map(convert).collect::<Vec<i32>>();
    let password = count_all_zeros(&values);
    assert_eq!(password, 6i32);
}

fn main() {
    let mut file = {
        let args: Vec<String> = std::env::args().collect();
        let path = &args[1];
        File::open(path).unwrap()
    };

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let input = input.split("\n").map(convert).collect::<Vec<i32>>();
    println!("{}", count_zeros(&input));
    println!("{}", count_all_zeros(&input));
}
