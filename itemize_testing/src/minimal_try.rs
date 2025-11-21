#![allow(warnings)]

use itemize::*;

// Test 1: No attributes
#[derive(Debug, Clone, PartialEq, TryIntoItems)]
pub struct Simple(i32);

// Test 2: With types attribute
#[derive(Debug, Clone, PartialEq, TryIntoItems)]
#[items_from(types(String), tuples(2), collections(vec, slice, array))]
pub struct WithTypes(i32);

impl TryFrom<String> for WithTypes {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<i32>()
            .map(WithTypes)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal() {
        // Test Simple
        let s = Simple(42);
        let items: Result<Vec<Simple>, std::convert::Infallible> = s.try_into_items().collect();
        assert_eq!(items.unwrap(), vec![Simple(42)]);
        
        // Test WithTypes
        let w = WithTypes(42);
        let items: Result<Vec<WithTypes>, String> = w.try_into_items().collect();
        assert_eq!(items.unwrap(), vec![WithTypes(42)]);
        
        // Test conversion - need to specify error type due to generic implementation
        let s = "123".to_string();
        let items: Result<Vec<WithTypes>, String> = TryIntoItems::<WithTypes, String>::try_into_items(s).collect();
        assert_eq!(items.unwrap(), vec![WithTypes(123)]);
    }
}
