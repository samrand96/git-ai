use serde_json::Value;

pub fn get_nested_string(value: &Value, path: &[&str]) -> Option<String> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(|val| val.to_string())
}

pub fn first_array_item<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.get(key)?.as_array()?.first()
}
