pub fn a4_1() {
    let mut sum = 0;
    for line in crate::lines(4) {
        let filt: String = line
            .split_once(": ")
            .unwrap()
            .1
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '|' || *c == ' ')
            .collect();
        let (first, second) = filt.split_once(" | ").unwrap();
        let win = first
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let have = second
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap());

        let total = have.filter(|n| win.contains(n)).count();
        if total != 0 {
            sum += 1 << (total - 1);
        }
    }
    println!("{sum}");
}

pub fn a4_2() {
    let mut sum = 0;
    let mut card_total = Vec::new();
    for (i, line) in crate::lines(4).into_iter().rev().enumerate() {
        let mut this = 1;
        let filt: String = line
            .split_once(": ")
            .unwrap()
            .1
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '|' || *c == ' ')
            .collect();

        let (first, second) = filt.split_once(" | ").unwrap();
        let win = first
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let have = second
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap());

        let total = have.filter(|n| win.contains(n)).count();
        for sub in 1..=total {
            this += card_total[i - sub];
        }
        card_total.push(this);
        sum += this;
    }
    println!("{sum}");
}
