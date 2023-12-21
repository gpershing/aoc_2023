pub fn hash_f(s: &str) -> u8 {
    s.chars().filter(|c| *c != '\n').fold(0, |acc, c| ((((acc as u16) + (c as u16)) * 17) % 256) as u8)
}