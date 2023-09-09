use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

pub type Properties = HashMap<String, DynamicPropertyValue>;
pub type DynamicProperties = HashMap<String, Properties>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DynamicPropertyValue {
    Boolean(bool),
    Float(f32), // TODO: Delete this after minecraft 1.20.40
    Double(f64),
    String(String),
    Vector3([f32; 3]),
}
