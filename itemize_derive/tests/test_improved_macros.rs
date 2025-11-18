#[cfg(test)]
mod test_improved_item {
    use itemize_derive::IntoItems;

    // Test basic struct with improved documentation
    #[derive(IntoItems, Debug, Clone)]
    #[into_items(from_types = [&str, String, i32], from_tuples = 3, from_collections(Vec, &[T], [T; N]))]
    struct Item {
        value: String,
    }

    impl From<&str> for Item {
        fn from(s: &str) -> Self {
            Item {
                value: s.to_string(),
            }
        }
    }

    impl From<String> for Item {
        fn from(s: String) -> Self {
            Item { value: s }
        }
    }

    impl From<i32> for Item {
        fn from(i: i32) -> Self {
            Item {
                value: i.to_string(),
            }
        }
    }

    // Additional From implementations for collections
    impl<'a> From<&'a &str> for Item {
        fn from(s: &'a &str) -> Self {
            Item {
                value: s.to_string(),
            }
        }
    }

    // Test error case: union (should fail compilation)
    // Uncomment to test error message
    // #[derive(IntoItems)]
    // union InvalidUnion {
    //     a: i32,
    //     b: f64,
    // }

    #[test]
    fn test_single_values() {
        // Test from_types conversions
        let items1: Vec<Item> = "hello".into_items().collect();
        assert_eq!(items1.len(), 1);
        assert_eq!(items1[0].value, "hello");

        let items2: Vec<Item> = String::from("world").into_items().collect();
        assert_eq!(items2.len(), 1);
        assert_eq!(items2[0].value, "world");

        let items3: Vec<Item> = 42.into_items().collect();
        assert_eq!(items3.len(), 1);
        assert_eq!(items3[0].value, "42");
    }

    #[test]
    fn test_tuples() {
        // Test tuple conversions
        let items: Vec<Item> = ("a", "b").into_items().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].value, "a");
        assert_eq!(items[1].value, "b");

        let items3: Vec<Item> = ("x", "y", "z").into_items().collect();
        assert_eq!(items3.len(), 3);
        assert_eq!(items3[0].value, "x");
        assert_eq!(items3[1].value, "y");
        assert_eq!(items3[2].value, "z");
    }

    #[test]
    fn test_collections() {
        // Test Vec conversion
        let vec = vec!["one", "two", "three"];
        let items: Vec<Item> = vec.into_items().collect();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].value, "one");
        assert_eq!(items[1].value, "two");
        assert_eq!(items[2].value, "three");

        // Test slice conversion
        let slice = &["alpha", "beta"][..];
        let items: Vec<Item> = slice.into_items().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].value, "alpha");
        assert_eq!(items[1].value, "beta");

        // Test array conversion
        let array = ["foo", "bar"];
        let items: Vec<Item> = array.into_items().collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].value, "foo");
        assert_eq!(items[1].value, "bar");
    }
}

#[cfg(test)]
mod test_public_visibility {
    use itemize_derive::IntoItems;

    // Test with visibility matching
    #[derive(IntoItems, Debug)]
    #[into_items(from_types = [i64])]
    pub struct PublicItem(String);

    impl From<i64> for PublicItem {
        fn from(n: i64) -> Self {
            PublicItem(n.to_string())
        }
    }

    #[test]
    fn test_public_item() {
        let item = 42i64.into_items().next().unwrap();
        assert_eq!(item.0, "42");
    }
}
