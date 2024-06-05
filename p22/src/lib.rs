fn celsius2farenheit(celsius: i32) -> i32 {
    // Careful with https://rust-lang.github.io/rust-clippy/master/index.html#/identity_op
    ((celsius * 9) / 5) + 32
}

fn farenheit2celsius(farenheit: i32) -> i32 {
    // Careful with https://rust-lang.github.io/rust-clippy/master/index.html#/identity_op
    (5 * (farenheit - 32)) / 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_from_celsius() {
        let same_celsius_value = -5;
        assert_eq!(
            farenheit2celsius(celsius2farenheit(same_celsius_value)),
            same_celsius_value
        );
    }

    #[test]
    fn loop_from_farenheit() {
        let same_fahrenheit_value = 212;
        assert_eq!(
            celsius2farenheit(farenheit2celsius(same_fahrenheit_value)),
            same_fahrenheit_value
        );
    }

    #[test]
    fn kat() {
        // Source: https://fr.farnell.com/en-FR/convertisseur-temperature
        let equivalences_fahrenheit_and_celsius = [(23, -5), (194, 90), (212, 100), (392, 200)];
        for equivalence in equivalences_fahrenheit_and_celsius {
            let actual_celsius = farenheit2celsius(equivalence.0);
            assert_eq!(
                equivalence.1, actual_celsius,
                "error: fahrenheit {} is not celsius {}. actual: {}",
                equivalence.0, equivalence.1, actual_celsius
            );

            let actual_fahrenheit = celsius2farenheit(equivalence.1);
            assert_eq!(
                equivalence.1, actual_celsius,
                "error: celsius {} is not fahrenheit {}. actual: {}",
                equivalence.1, equivalence.0, actual_fahrenheit
            );

            assert_eq!(
                celsius2farenheit(farenheit2celsius(equivalence.0)),
                equivalence.0
            );
            assert_eq!(
                farenheit2celsius(celsius2farenheit(equivalence.1)),
                equivalence.1
            );
        }
    }
}
