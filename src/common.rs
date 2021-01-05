use std::collections::HashMap;

pub struct StorePropertiesBuilder {
    properties: HashMap<String, String>,
}

impl StorePropertiesBuilder {
    pub fn build() -> Self {
        StorePropertiesBuilder {
            properties: HashMap::new(),
        }
    }

    pub fn insert<'a>(&'a mut self, key: &str, value: &str) -> &'a mut StorePropertiesBuilder {
        self.insert_or(true, key, value)
    }

    pub fn insert_or<'a>(
        &'a mut self,
        condition: bool,
        key: &str,
        value: &str,
    ) -> &'a mut StorePropertiesBuilder {
        if condition {
            self.properties.insert(key.into(), value.into());
        }
        self
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        self.properties.clone()
    }
}
