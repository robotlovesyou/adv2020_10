use std::collections::HashMap;

fn main() {
    let adapters = to_adapter_list(include_str!("../input.txt"));

    let (_, total_ones, total_threes) = adapters.iter().fold(
        (0u64, 0usize, 0usize),
        |(last, ones, threes), adapter| match *adapter - last {
            1 => (*adapter, ones + 1, threes),
            2 => (*adapter, ones, threes),
            3 => (*adapter, ones, threes + 1),
            _ => panic!("oops"),
        },
    );

    let product = total_ones * (total_threes + 1);
    println!("the answer is {}", product);

    let total_options = count_paths(&adapters);

    println!("the number of paths is {}", total_options);
}

fn to_adapter_list(input: &str) -> Vec<u64> {
    let mut adapters: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().expect("bad integer"))
        .collect();

    adapters.sort_unstable();
    adapters
}

fn count_paths(adapters: &[u64]) -> usize {
    let mut map: HashMap<u64, usize> = HashMap::new();
    let target = adapters.last().unwrap() + 3;

    map.insert(0, 1);
    for adapter in adapters.iter() {
        let mut count = 0;
        if *adapter >= 1 && map.contains_key(&(*adapter - 1)) {
            count += *map.get(&(*adapter - 1)).unwrap();
        }
        if *adapter >= 2 && map.contains_key(&(*adapter - 2)) {
            count += *map.get(&(*adapter - 2)).unwrap();
        }
        if *adapter >= 3 && map.contains_key(&(*adapter - 3)) {
            count += *map.get(&(*adapter - 3)).unwrap();
        }
        map.insert(*adapter, count);
    }
    map[&(target - 3)]
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const SMALL_SET: &'static str = indoc! {"\
    16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4"};

    const LARGE_SET: &'static str = indoc!(
        "\
    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"
    );

    #[test]
    fn loads_adapter_set() {
        let adapters = to_adapter_list(SMALL_SET);
        assert_eq!(11, adapters.len());
        assert_eq!(1, *adapters.first().unwrap());
        assert_eq!(19, *adapters.last().unwrap());
    }

    #[test]
    fn it_finds_the_correct_path_count() {
        let adapters = to_adapter_list(SMALL_SET);
        let paths = count_paths(&adapters);
        assert_eq!(8, paths);

        let large_adapters = to_adapter_list(LARGE_SET);
        let large_paths = count_paths(&large_adapters);
        assert_eq!(19208, large_paths);
    }
}
