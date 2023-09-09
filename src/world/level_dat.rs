use std::fmt::Debug;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::Deserialize;
use zuri_nbt::{encoding::LittleEndian, serde::deserialize, tag::Compound, NBTTag};

use super::error::WorldError;

#[derive(Deserialize, Debug)]
pub struct LevelDat {
    #[serde(rename = "LevelName")]
    pub level_name: String,
}

#[derive(Debug)]
pub struct LevelData {
    // Format version of the level.dat file
    pub format_version: i32,
    // Raw NBT data
    pub nbt: Compound,
    // Deserialized NBT data
    data: LevelDat,
}

impl LevelData {
    pub fn deserialize(mut buf: Bytes) -> Result<Self, WorldError> {
        let format_version = buf.get_i32_le();
        buf.advance(4); // Skip length
        let nbt = NBTTag::read(&mut buf, &mut LittleEndian).map_err(WorldError::NBTReadError)?;
        let data = deserialize::<LevelDat>(&nbt).map_err(WorldError::NBTDeserializeError)?;

        Ok(Self {
            nbt: nbt.try_into().unwrap(),
            format_version,
            data,
        })
    }

    pub fn serialize(&self) -> Result<Bytes, WorldError> {
        let mut nbt_buf = BytesMut::new();
        self.nbt
            .clone()
            .write(&mut nbt_buf, &mut LittleEndian)
            .map_err(WorldError::NBTWriteError)?;

        let mut buf = BytesMut::new();
        buf.put_i32_le(self.format_version);
        buf.put_i32_le(nbt_buf.len() as i32);
        buf.put(nbt_buf);

        Ok(buf.into())
    }

    pub fn get_data(&self) -> &LevelDat {
        &self.data
    }
}
