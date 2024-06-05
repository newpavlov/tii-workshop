fn main() {
    println!("Hello, world!");
}

fn celsius2farenheit(celsius: i32) -> i32 {
    return (celsius * (9 / 5)) + 32;
}

fn farenheit2celsius(farenheit: i32) -> i32 {
    return (farenheit - 32) / (9 / 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_from_celsius() {
        assert_eq!(farenheit2celsius(celsius2farenheit(2)), 2);
    }

    #[test]
    fn loop_from_farenheit() {
        assert_eq!(celsius2farenheit(farenheit2celsius(2)), 2);
    }
}