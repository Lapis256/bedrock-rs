use zuri_nbt::{
    serde::{deserialize, serialize},
    tag::Compound,
};

use crate::dynamic_property::DynamicProperties;

pub struct Entity {
    db_key: Vec<u8>,
    pub nbt: Compound,
}

impl Entity {
    pub fn new(db_key: Vec<u8>, nbt: Compound) -> Self {
        Self { db_key, nbt }
    }

    pub fn get_db_key(&self) -> &[u8] {
        &self.db_key
    }

    pub fn get_dynamic_properties(&self) -> Option<DynamicProperties> {
        deserialize(self.nbt.0.get("DynamicProperties")?).ok()
    }

    pub fn set_dynamic_properties(&mut self, properties: DynamicProperties) {
        self.nbt.0.insert(
            "DynamicProperties".to_string(),
            serialize::<DynamicProperties>(&properties).unwrap(),
        );
    }
}
