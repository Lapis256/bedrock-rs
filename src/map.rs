use image::{png::PngEncoder, ColorType};
use serde::{Deserialize, Serialize};
use zuri_nbt::{
    err::ErrorPath,
    serde::{deserialize, serialize, SerializeError},
    NBTTag,
};

use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct Map {
    pub dimension: u8,
    #[serde(rename = "fullyExplored")]
    pub fully_explored: u8,
    #[serde(rename = "mapLocked")]
    pub map_locked: u8,
    pub scale: u8,
    #[serde(rename = "unlimitedTracking")]
    pub unlimited_tracking: u8,
    pub height: i16,
    width: i16,
    #[serde(rename = "xCenter")]
    pub x_center: i32,
    #[serde(rename = "zCenter")]
    pub z_center: i32,
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "parentMapId")]
    pub parent_map_id: i64,
    pub colors: Vec<u8>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            dimension: 0,
            fully_explored: 0,
            map_locked: 0,
            scale: 0,
            unlimited_tracking: 0,
            height: 128,
            width: 128,
            x_center: 0,
            z_center: 0,
            map_id: -1,
            parent_map_id: -1,
            colors: vec![0; 128 * 128 * 4],
        }
    }
}

impl From<NBTTag> for Map {
    fn from(nbt: NBTTag) -> Self {
        deserialize::<Map>(&nbt).unwrap()
    }
}

impl Map {
    pub fn encode_png(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let png = PngEncoder::new(&mut buf);
        let _ = png.encode(
            &self.colors,
            self.width as u32,
            self.height as u32,
            ColorType::Rgba8,
        );
        buf
    }

    pub fn to_nbt(&self) -> Result<NBTTag, ErrorPath<SerializeError>> {
        serialize(&self)
    }

    pub fn is_empty(&self) -> bool {
        !self.colors.iter().any(|&x| x != 0)
    }

    pub fn get_db_key(&self) -> Vec<u8> {
        format!("map_{}", self.map_id).into_bytes()
    }
}
