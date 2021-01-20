use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn random_u32() {
        assert!(rand::random::<usize>() < (1 << 16u32));
    }
}
