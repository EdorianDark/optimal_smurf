
pub fn optimal_number() ->i64 {
    42
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_optimal_number(){
        assert_eq!(optimal_number(), 42);
    }
}
