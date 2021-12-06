fn solution(input: &Vec<usize>, iterations: usize) -> usize {
    let mut freq: [usize; 10] = [0; 10];
    for age in input {
        freq[*age] += 1;
    }

    for _ in 0..iterations {
        let nzeroes = freq[0];
        for i in 0..9 {
            freq[i] = freq[i + 1];
        }
        freq[6] += nzeroes;
        freq[8] = nzeroes;
    }

    freq.iter().sum()
}

fn main() {
    let input: Vec<_> = include_str!("../../inputs/day6.txt")
        .split(',')
        .map(|n| usize::from_str_radix(n, 10).unwrap())
        .collect();

    let nfish1 = solution(&input, 80);
    let nfish2 = solution(&input, 256);
    println!("part1: {}", nfish1);
    println!("part2: {}", nfish2);
}
