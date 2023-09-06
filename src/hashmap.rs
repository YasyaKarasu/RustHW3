macro_rules! hash_map {
    ( $( $key: expr => $val: expr), * ) => {{
            let mut tmp_map = ::std::collections::HashMap::new();
            $(
                tmp_map.insert($key, $val);
            )*
            tmp_map    
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash_map_macro() {
        let map = hash_map!{
            "a" => 1,
            "b" => 2,
            "c" => 3
        };
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
        assert_eq!(map["c"], 3);
    }
}