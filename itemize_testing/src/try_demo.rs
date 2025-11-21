#![allow(warnings)]

use itemize::*;
use std::num::ParseIntError;

// Basic example with TryIntoItems
#[derive(Debug, Clone, PartialEq, TryIntoItems)]
#[items_from(types(String), error_type(ParseIntError))]
pub struct Integer(i32);

impl TryFrom<String> for Integer {
    type Error = ParseIntError;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<i32>().map(Integer)
    }
}

// Example with custom error and collections
#[derive(Debug, Clone, PartialEq, TryIntoItems, TryIntoRows)]
#[items_from(tuples(2), collections(vec), error_type(String))]
pub struct ValidRow(String);

impl TryFrom<String> for ValidRow {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Empty string not allowed".to_string())
        } else {
            Ok(ValidRow(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_try_into_items() {
        // Self implementation
        let i = Integer(42);
        let result: Result<Vec<_>, ParseIntError> = i.try_into_items().collect();
        assert_eq!(result.unwrap(), vec![Integer(42)]);
        
        // String conversion - success
        let s = "123".to_string();
        let result: Result<Vec<_>, ParseIntError> = TryIntoItems::<Integer, ParseIntError>::try_into_items(s).collect();
        assert_eq!(result.unwrap(), vec![Integer(123)]);
        
        // String conversion - failure
        let s = "abc".to_string();
        let result: Result<Vec<_>, ParseIntError> = TryIntoItems::<Integer, ParseIntError>::try_into_items(s).collect();
        assert!(result.is_err());
    }

    #[test]
    fn test_try_into_items_with_tuples() {
        // Tuple success
        let t = ("hello".to_string(), "world".to_string());
        let result: Result<Vec<_>, String> = TryIntoItems::<ValidRow, String>::try_into_items(t).collect();
        assert_eq!(result.unwrap(), vec![ValidRow("hello".to_string()), ValidRow("world".to_string())]);
        
        // Tuple with error
        let t = ("valid".to_string(), "".to_string());
        let result: Result<Vec<_>, String> = TryIntoItems::<ValidRow, String>::try_into_items(t).collect();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_try_into_rows_with_collections() {
        // Vec of tuples
        let input = vec![
            ("a".to_string(), "b".to_string()),
            ("c".to_string(), "d".to_string())
        ];
        let result: Result<Vec<Vec<_>>, String> = TryIntoRows::<ValidRow, String>::try_into_rows(input)
            .map(|row| row.collect::<Result<Vec<_>, _>>())
            .collect();
        
        let expected = vec![
            vec![ValidRow("a".to_string()), ValidRow("b".to_string())],
            vec![ValidRow("c".to_string()), ValidRow("d".to_string())]
        ];
        assert_eq!(result.unwrap(), expected);
        
        // Vec with error
        let input = vec![
            ("valid".to_string(), "also valid".to_string()),
            ("uh oh".to_string(), "".to_string())
        ];
        let result: Result<Vec<Vec<_>>, String> = TryIntoRows::<ValidRow, String>::try_into_rows(input)
            .map(|row| row.collect::<Result<Vec<_>, _>>())
            .collect();
        assert!(result.is_err());
    }
}
