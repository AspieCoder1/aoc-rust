pub fn as_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    input.lines()
        .map(|line| line.parse::<T>().unwrap())
        .collect()
}