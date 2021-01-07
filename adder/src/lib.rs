pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn not_equal() {
        assert_ne!(2 + 2, 5);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
        assert!(
            result.contains("Joe"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
