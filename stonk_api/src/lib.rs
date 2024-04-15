pub mod get_stonks;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_stonks::test_mod();
        assert_eq!(4, 4);
    }
}
