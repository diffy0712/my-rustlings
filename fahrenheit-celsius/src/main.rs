
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    ( fahrenheit - 32.0 ) / 1.8
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

#[cfg(test)]
mod fahrenheit_celsius {
    use super::*;

    #[test]
    fn fahrenheit_to_celsius_calculates_correctly() {
        assert_eq!(fahrenheit_to_celsius(77.0), 25.0);
        assert_eq!(fahrenheit_to_celsius(86.0), 30.0);
        assert_eq!(fahrenheit_to_celsius(91.4), 33.0);
        assert_eq!(fahrenheit_to_celsius(95.0), 35.0);
        assert_eq!(fahrenheit_to_celsius(104.0), 40.0);
        assert_eq!(fahrenheit_to_celsius(356.0), 180.0);
    }

    #[test]
    fn celsius_to_fahrenheit_calculates_correctly() {
        assert_eq!(celsius_to_fahrenheit(25.0), 77.0);
        assert_eq!(celsius_to_fahrenheit(30.0), 86.0);
        assert_eq!(celsius_to_fahrenheit(33.0), 91.399994);
        assert_eq!(celsius_to_fahrenheit(35.0), 95.0);
        assert_eq!(celsius_to_fahrenheit(40.0), 104.0);
        assert_eq!(celsius_to_fahrenheit(180.0), 356.0);
    }
}
