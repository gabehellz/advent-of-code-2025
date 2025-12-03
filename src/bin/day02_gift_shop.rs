use std::{fs::File, io::Read};

fn repeats(value: &str) -> bool {
    let middle = value.len() / 2;
    value[0..middle] == value[middle..value.len()]
}

fn split_every(value: &str, n: usize) -> Vec<&str> {
    let mut temp: Vec<&str> = Vec::new();
    let mut i = 0;
    let mut j = n;
    let len = value.len();

    while len > j {
        temp.push(&value[i..j]);
        i += n;
        j += n;

        if j >= len {
            temp.push(&value[i..len]);
        }
    }

    temp
}

fn repeats_part2(value: &str) -> bool {
    let mut i = 1;
    let len = value.len();

    while len > i {
        let values = split_every(value, i);
        let mut iter = values.iter();
        let mut amount = 0;
        let check = iter.next().unwrap();

        for other in iter {
            if check.eq(other) {
                amount += 1;
            }
        }

        if amount == values.len() - 1 {
            return true;
        }

        i += 1;
    }

    false
}

fn repeated(values: Vec<&str>) -> u64 {
    let mut temp = 0u64;
    let first = values[0];
    let last = values[values.len() - 1];

    let first_num = first.parse::<u64>().unwrap_or_default();
    let last_num = last.parse::<u64>().unwrap_or_default();

    if repeats(first) {
        temp += first_num;
    }

    if repeats(last) {
        temp += last_num;
    }

    for num in (first_num + 1)..last_num {
        let string = num.to_string();

        if repeats(&string) {
            temp += num;
        }
    }

    temp
}

fn repeated_part2(values: Vec<&str>) -> u64 {
    let mut temp = 0u64;
    let first = values[0];
    let last = values[values.len() - 1];

    let first_num = first.parse::<u64>().unwrap_or_default();
    let last_num = last.parse::<u64>().unwrap_or_default();

    if repeats_part2(first) {
        temp += first_num;
    }

    if repeats_part2(last) {
        temp += last_num;
    }

    for num in (first_num + 1)..last_num {
        let string = num.to_string();

        if repeats_part2(&string) {
            temp += num;
        }
    }

    temp
}

fn process_input(input: String) -> u64 {
    input
        .split(",")
        .map(|s| s.split("-").map(|v| v.trim()).collect::<Vec<&str>>())
        .map(|v| repeated(v))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn process_input_part2(input: String) -> u64 {
    input
        .split(",")
        .map(|s| s.split("-").map(|v| v.trim()).collect::<Vec<&str>>())
        .map(|v| repeated_part2(v))
        .reduce(|acc, e| acc + e)
        .unwrap()
}

#[test]
fn test_gift_shop_part2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_owned();

    let result = process_input_part2(input);
    assert_eq!(result, 4174379265u64);
}

#[test]
fn test_gift_shop_part1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_owned();

    let result = process_input(input);
    assert_eq!(result, 1227775554u64);
}

fn main() {
    let mut file = {
        let args: Vec<String> = std::env::args().collect();
        let path = &args[1];
        File::open(path).unwrap()
    };

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let result = process_input(input.clone());
    println!("{}", result);

    let result = process_input_part2(input);
    println!("{}", result);
}
