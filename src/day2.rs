use std::{collections::HashMap, ops::Add};

struct Pair(u32, u32);
impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Pair {
    fn product(self) -> u32 {
        self.0 * self.1
    }
}

pub fn part1(input: &str) -> u32 {
    let pair = input
        .lines()
        .map(count_repeats)
        .fold(Pair(0, 0), |acc, repeats| acc + repeats);

    pair.product()
}

fn count_repeats(id: &str) -> Pair {
    let mut count: HashMap<char, u32> = HashMap::new();

    id.chars().for_each(|c| match count.get_mut(&c) {
        Some(occurences) => {
            *occurences += 1;
        }
        None => {
            count.insert(c, 1);
        }
    });

    count
        .values()
        .fold(Pair(0, 0), |accumulator, count| match count {
            2 => Pair(1, accumulator.1),
            3 => Pair(accumulator.0, 1),
            _ => accumulator,
        })
}

//*************************************************************//
// part 2
//*************************************************************//

pub fn part2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let (x, y) = find_pair(&lines, |(x, y)| match_with_tolerance(x, y, 1)).unwrap();

    get_common_chars(x, y)
}

fn get_common_chars(x: &str, y: &str) -> String {
    let x = x.chars();
    let y = y.chars();
    x.zip(y).filter(|(l, r)| l == r).map(|(l, _)| l).collect()
}

fn find_pair<'a, T>(
    lines: &[&'a T],
    mut predicate: impl FnMut((&'a T, &'a T)) -> bool,
) -> Option<(&'a T, &'a T)>
where
    T: ?Sized,
{
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let x = lines[i];
            let y = lines[j];

            if predicate((x, y)) {
                return Some((x, y));
            }
        }
    }

    None
}

fn match_with_tolerance<'a>(a: &'a str, b: &'a str, tolerance: usize) -> bool {
    let a = a.chars();
    let b = b.chars();
    let mismatches: usize = a.zip(b).map(|(l, r)| if l != r { 1 } else { 0 }).sum();
    mismatches <= tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/day-2.txt");

    #[test]
    fn test_part1() {
        let input = "
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

        assert_eq!(part1(input), 12);
        assert_eq!(part1(INPUT), 6200);
    }

    #[test]
    fn test_part2() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

        assert_eq!(part2(input), "fgij");
        assert_eq!(part2(INPUT), "xpysnnkqrbuhefmcajodplyzw");
    }

    #[test]
    fn test_remove_non_matching() {
        assert_eq!(get_common_chars("abcde", "_b_de"), "bde".to_string());
        assert_eq!(get_common_chars("abcde", "___de"), "de".to_string());
        assert_eq!(get_common_chars("abcde", "abcde"), "abcde".to_string());
        assert_eq!(get_common_chars("badec", "abcde"), "".to_string());
    }

    #[test]
    fn test_match_with_tolerance() {
        assert!(match_with_tolerance("abc", "abc", 1));
        assert!(match_with_tolerance("abc", "aac", 1));
        assert!(match_with_tolerance("hello", "heloo", 1));
        assert!(match_with_tolerance("hello", "hlelo", 2));
        assert!(!match_with_tolerance("llll", "lssss", 1));
    }
}
