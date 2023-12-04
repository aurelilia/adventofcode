pub fn a2_1() {
    const COLS: [(&str, usize); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    let mut sum = 0;
    for line in crate::lines(2) {
        let game: usize = line.split(&[' ', ':']).nth(1).unwrap().parse().unwrap();
        let content = line.split(": ").nth(1).unwrap();

        let mut possible = true;
        for part in content.split("; ") {
            for color in part.split(", ") {
                let (count, col) = color.split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();
                let max = COLS.iter().find(|(n, _)| *n == col).unwrap().1;
                possible &= max >= count;
            }
        }

        if possible {
            sum += game;
        }
    }
    println!("{sum}");
}

pub fn a2_2() {
    const COLS: [&str; 3] = ["red", "green", "blue"];

    let mut sum = 0;
    for line in crate::lines(2) {
        let content = line.split(": ").nth(1).unwrap();

        let mut counts = [0, 0, 0];
        for part in content.split("; ") {
            for color in part.split(", ") {
                let (count, col) = color.split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();
                let idx = COLS
                    .into_iter()
                    .enumerate()
                    .find(|(_, n)| *n == col)
                    .unwrap()
                    .0;
                counts[idx] = count.max(counts[idx]);
            }
        }

        sum += counts.iter().product::<usize>();
    }
    println!("{sum}");
}
