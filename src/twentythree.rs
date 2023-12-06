pub fn day_1() {
    let mut buf = String::new();
    let mut numbers = Vec::<u32>::new();

    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        if let Some(index) = buf.find('\n') {
            if index == 0 { break; }
        }

        let sum = buf
            .split("\n")
            .next()
            .map(|line| {
                let chars = line.chars();
                let mut numbers = chars.filter(|char| char.is_numeric());
                let first = numbers.next().unwrap_or('0');
                let last = numbers.last().unwrap_or(first).to_digit(10).unwrap();
                (first.to_digit(10).unwrap() * 10) + last
            }).unwrap();

        numbers.push(sum);
        buf.clear();
    }

    let sum_all = numbers.into_iter().sum::<u32>();

    println!("{}", sum_all);
}
