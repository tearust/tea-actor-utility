use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Item {
    value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AsyncCollector {
    essential_items: HashMap<String, Item>,
    optional_items: HashMap<String, Item>,
    expires_seconds: i32,
}

impl Item {
    pub fn new(value: String) -> Self {
        Item { value: Some(value) }
    }

    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.is_some() && self.value.as_ref().unwrap().is_empty()
    }
}

impl AsyncCollector {
    pub fn is_complete(&self) -> bool {
        self.essential_items
            .iter()
            .all(|(_, item)| !item.is_empty())
    }

    pub fn init(&mut self, keys: Vec<String>, optional: bool) {
        if optional {
            keys.iter().for_each(|key| {
                self.optional_items.insert(key.clone(), Item::default());
            });
        } else {
            keys.iter().for_each(|key| {
                self.essential_items.insert(key.clone(), Item::default());
            });
        }
    }

    pub fn set(&mut self, key: String, item: String, optional: bool) {
        if optional {
            self.optional_items.insert(key, Item::new(item));
        } else {
            self.essential_items.insert(key, Item::new(item));
        }
    }

    pub fn set_expires(&mut self, seconds: i32) {
        self.expires_seconds = seconds;
    }
}

impl Default for Item {
    fn default() -> Self {
        Item { value: None }
    }
}

impl Default for AsyncCollector {
    fn default() -> Self {
        AsyncCollector {
            essential_items: HashMap::new(),
            optional_items: HashMap::new(),
            expires_seconds: 6000,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::async_collector::Item;

    #[test]
    fn string_item_empty_works() {
        let i = Item::new("".to_string());
        assert!(i.is_empty())
    }
}
