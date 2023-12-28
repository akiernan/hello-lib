pub fn greet() -> String {
    "Hello World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let greeting = greet();
        assert_eq!(greeting, "Hello World!");
    }
}
