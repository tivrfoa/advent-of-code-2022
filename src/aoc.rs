pub trait AOC {
    fn part1(&self, input: Option<String>, args: Vec<String>) -> String;
    fn part2(&self, input: Option<String>, args: Vec<String>) -> String;
}
