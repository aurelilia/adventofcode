const NAMES: [&str; 10] = [
    "---------------",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

pub fn a1(is_2: bool) {
    let mut sum = 0;
    for line in crate::lines(1) {
        let mut matches = Vec::<(usize, usize)>::new();
        if is_2 {
            for (num, name) in NAMES.iter().enumerate() {
                line.match_indices(name)
                    .for_each(|(i, _)| matches.push((i, num)));
            }
        }
        line.chars()
            .enumerate()
            .filter(|(_, d)| char::is_ascii_digit(d))
            .for_each(|(i, d)| matches.push((i, d.to_digit(10).unwrap() as usize)));
        matches.sort_by_key(|(i, _)| *i);

        sum += matches.first().unwrap().1 * 10;
        sum += matches.last().unwrap().1;
    }
    println!("{sum}");
}
