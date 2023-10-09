use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;

pub fn hashmap_to_str<K: Display, V: Display> (hashmap: &HashMap<K, V>) -> String {
    let hashmap_len = hashmap.len();
    hashmap.iter().enumerate().fold(
        String::new(),
        |string_builder, (index, (key, value))| {
            let value_str = value.to_string();
            let ending_comma =
                if index == hashmap_len - 1 { String::from("") } else { String::from(", ") };
            let entry_str = format!(
                "{{key: {}, value: {}}}{}",
                key,
                value_str,
                ending_comma
            );
            string_builder.add(&entry_str)
        },
    )
}

pub fn vec_to_str<V: Display> (vec: &Vec<V>) -> String {
    let vec_len = vec.len();
    let vec_str = vec.iter().enumerate().fold(
        String::new(),
        |string_builder, (index, value)| {
            let value_str = value.to_string();
            let ending_comma =
                if index == vec_len - 1 { String::from("") }
                else { String::from(", ") };
            let entry_str = format!("{}{}", value_str, ending_comma);
            string_builder.add(&entry_str)
        },
    );
    format!("[{}]", vec_str)
}
