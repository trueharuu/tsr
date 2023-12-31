use std::collections::HashMap;

pub type Levels = HashMap<String, Level>;

pub enum Level {
    Allow,
    Warn,
    Deny,
}

pub fn default_levels() -> Levels {
    HashMap::from([("eq_eq_eq", Level::Warn)])
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
}
