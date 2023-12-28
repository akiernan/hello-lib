pub fn greet(s: Option<&str>) -> String {
    match s {
        Some(p) => format!("Hello {}!", p),
        None => "Hello World!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let greeting = greet(None);
        assert_eq!(greeting, "Hello World!");
    }
}
