pub fn add(a:u32, b: u32) -> u32 {
    return a + b;
}
pub fn fma(a:u32, b: u32, c: u32) -> u32 {
    return (a * b) + c;
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::fma;

    #[test]
    fn it_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(fma(1, 2, 3), 5);
    }
}
