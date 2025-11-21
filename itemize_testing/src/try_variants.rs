#![allow(warnings)]

use itemize::*;
use std::num::ParseIntError;

// Simple type with TryIntoItems using generic error
#[derive(Debug, Clone, PartialEq, TryIntoItems)]
#[items_from(types(String), tuples(2), collections(vec))]
pub struct Number(i32);

impl TryFrom<String> for Number {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<i32>().map(Number)
    }
}

// Type with custom error type
#[derive(Debug, Clone, PartialEq, TryIntoItems)]
#[items_from(types(String, i32), error_type(String))]
pub struct ValidatedValue(i32);

impl TryFrom<String> for ValidatedValue {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .parse::<i32>()
            .map(ValidatedValue)
            .map_err(|e| format!("Parse error: {}", e))
    }
}

impl TryFrom<i32> for ValidatedValue {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(ValidatedValue(value))
        } else {
            Err("Value must be non-negative".to_string())
        }
    }
}

// Type with TryIntoRows
#[derive(Debug, Clone, PartialEq, TryIntoItems, TryIntoRows)]
#[items_from(types(String), tuples(2), collections(vec), error_type(String))]
pub struct Row(String);

impl TryFrom<String> for Row {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string not allowed".to_string())
        } else {
            Ok(Row(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_into_items_basic() {
        // Helper function with explicit error type
        fn try_into_items(
            x: impl TryIntoItems<Number, ParseIntError>,
        ) -> Result<Vec<Number>, ParseIntError> {
            x.try_into_items().collect()
        }

        // Self implementation
        assert_eq!(try_into_items(Number(42)).unwrap(), vec![Number(42)]);

        // From types
        assert_eq!(
            try_into_items("123".to_string()).unwrap(),
            vec![Number(123)]
        );

        // Error cases
        assert!(try_into_items("not a number".to_string()).is_err());
    }

    #[test]
    fn test_try_into_items_tuples() {
        // Use concrete error type to avoid inference issues
        fn try_into_items(
            x: impl TryIntoItems<Number, ParseIntError>,
        ) -> Result<Vec<Number>, ParseIntError> {
            x.try_into_items().collect()
        }

        // Tuple implementations
        let result = try_into_items(("10".to_string(), "20".to_string()));
        assert_eq!(result.unwrap(), vec![Number(10), Number(20)]);

        // Error in tuple
        let result = try_into_items(("42".to_string(), "bad".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_try_into_items_collections() {
        // Use concrete error type
        fn try_into_items(
            x: impl TryIntoItems<Number, ParseIntError>,
        ) -> Result<Vec<Number>, ParseIntError> {
            x.try_into_items().collect()
        }

        // Vec
        let result = try_into_items(vec!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert_eq!(result.unwrap(), vec![Number(1), Number(2), Number(3)]);

        // Error in collection
        let result = try_into_items(vec!["1".to_string(), "2".to_string(), "bad".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_into_items_custom_error() {
        fn try_into_items(
            x: impl TryIntoItems<ValidatedValue, String>,
        ) -> Result<Vec<ValidatedValue>, String> {
            x.try_into_items().collect()
        }

        // Valid values
        assert_eq!(try_into_items(42).unwrap(), vec![ValidatedValue(42)]);
        assert_eq!(
            try_into_items("100".to_string()).unwrap(),
            vec![ValidatedValue(100)]
        );

        // Invalid values
        let err = try_into_items(-5).unwrap_err();
        assert_eq!(err, "Value must be non-negative");

        let err = try_into_items("invalid".to_string()).unwrap_err();
        assert!(err.contains("Parse error"));
    }

    #[test]
    fn test_try_into_rows() {
        fn try_into_rows(x: impl TryIntoRows<Row, String>) -> Result<Vec<Vec<Row>>, String> {
            x.try_into_rows()
                .map(|row_iter| row_iter.collect::<Result<Vec<_>, _>>())
                .collect::<Result<Vec<_>, _>>()
        }

        // Self
        assert_eq!(
            try_into_rows(Row("test".to_string())).unwrap(),
            vec![vec![Row("test".to_string())]]
        );

        // Tuples
        let input = (
            ("a".to_string(), "b".to_string()),
            ("c".to_string(), "d".to_string()),
        );
        assert_eq!(
            try_into_rows(input).unwrap(),
            vec![
                vec![Row("a".to_string()), Row("b".to_string())],
                vec![Row("c".to_string()), Row("d".to_string())],
            ]
        );

        // Collections
        let input = vec![
            ("x".to_string(), "y".to_string()),
            ("z".to_string(), "w".to_string()),
        ];
        assert_eq!(
            try_into_rows(input).unwrap(),
            vec![
                vec![Row("x".to_string()), Row("y".to_string())],
                vec![Row("z".to_string()), Row("w".to_string())],
            ]
        );

        // Error case - empty string
        let input = (
            ("valid".to_string(), "".to_string()),
            ("test".to_string(), "data".to_string()),
        );
        let result = try_into_rows(input);
        assert!(result.is_err());
    }
}
