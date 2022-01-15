pub mod day1 {
    use std::collections::HashSet;

    pub fn part1(input: &str) -> i32 {
        input
            .trim()
            .lines()
            .map(|n| n.parse::<i32>().unwrap())
            .sum()
    }

    pub fn part2(input: &str) -> i32 {
        let mut found = HashSet::new();
        let mut total = 0_i32;

        input
            .lines()
            .map(|n| n.parse::<i32>().unwrap())
            .cycle()
            .find(|&n| {
                total += n;
                !found.insert(total)
            });

        total
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const INPUT: &str = include_str!("../input/day-1.txt");

        #[test]
        fn part1_test() {
            let input = "
+1
-2
+3
+1
";
            assert_eq!(part1(input), 3);

            assert_eq!(part1(INPUT), 599);
        }

        #[test]
        fn part2_test() {
            let input = "+3
+3
+4
-2
-4
";
            assert_eq!(part2(input), 10);
            assert_eq!(part2(INPUT), 81204);
        }
    }
}
