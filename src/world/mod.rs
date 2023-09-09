pub mod error;
pub mod level_dat;

use std::{cell::OnceCell, collections::HashMap};

use bytes::Bytes;
use rusty_leveldb::DB;
use zuri_nbt::{
    encoding::LittleEndian,
    serde::{deserialize, serialize},
    NBTTag,
};

use self::{error::WorldError, level_dat::LevelData};
use crate::{
    dynamic_property::DynamicProperties, entity::Entity, level_db::create_bedrock_options,
    level_db::Iter,
};

pub struct World {
    db: OnceCell<DB>,
    path: String,
}

impl World {
    pub fn new(path: String) -> Self {
        Self {
            path,
            db: OnceCell::new(),
        }
    }

    pub fn open(&mut self) {
        let mut option = create_bedrock_options(10);
        option.create_if_missing = false;
        if let Ok(db) = DB::open(format!("{}/db", self.path), option) {
            let _ = self.db.set(db);
        };
    }

    pub fn close(&mut self) -> Result<(), WorldError> {
        let db = self.get_db_mut()?;
        let _ = db.close();
        Ok(())
    }

    pub fn read_data(&self) -> Result<LevelData, WorldError> {
        let buf = match std::fs::read(format!("{}/level.dat", self.path)) {
            Ok(buf) => buf,
            Err(e) => return Err(WorldError::IOError(e)),
        };
        let buf = Bytes::from(buf);
        LevelData::deserialize(buf)
    }

    pub fn get_db(&self) -> Result<&DB, WorldError> {
        match self.db.get() {
            Some(db) => Ok(db),
            None => Err(WorldError::DBClosed),
        }
    }

    pub fn get_db_mut(&mut self) -> Result<&mut DB, WorldError> {
        match self.db.get_mut() {
            Some(db) => Ok(db),
            None => Err(WorldError::DBClosed),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_nbt_from_db(&mut self, key: &[u8]) -> Result<NBTTag, WorldError> {
        let db = self.get_db_mut()?;

        let data = match db.get(key) {
            None => return Err(WorldError::DBValueNotFound(key.to_vec())),
            Some(data) => data,
        };
        NBTTag::read(&mut data.as_slice(), &mut LittleEndian).map_err(WorldError::NBTReadError)
    }

    fn get_nbt_vec_from_db(
        &mut self,
        key_prefix: &[u8],
    ) -> Result<HashMap<Vec<u8>, NBTTag>, WorldError> {
        let db = self.get_db_mut()?;
        let mut tags: HashMap<Vec<u8>, NBTTag> = HashMap::new();
        for (key, value) in Iter::from_db(db) {
            if key.starts_with(key_prefix) {
                let tag = NBTTag::read(&mut value.as_slice(), &mut LittleEndian)
                    .map_err(WorldError::NBTReadError)?;
                tags.insert(key, tag);
            }
        }
        Ok(tags)
    }

    pub fn get_dynamic_properties(&mut self) -> Result<DynamicProperties, WorldError> {
        deserialize(&self.get_nbt_from_db(b"DynamicProperties")?)
            .map_err(WorldError::NBTDeserializeError)
    }

    pub fn update_dynamic_properties(
        &mut self,
        properties: &DynamicProperties,
    ) -> Result<(), WorldError> {
        let db = self.get_db_mut()?;
        let mut buf = Vec::new();
        let nbt = serialize(properties).map_err(WorldError::NBTSerializeError)?;
        nbt.write(&mut buf, &mut LittleEndian)
            .map_err(WorldError::NBTWriteError)?;
        let _ = db.put(b"DynamicProperties", &buf);
        let _ = db.flush();
        Ok(())
    }

    pub fn get_local_player(&mut self) -> Result<Entity, WorldError> {
        let key = b"~local_player";
        self.get_nbt_from_db(key)
            .map(|nbt| Entity::new(key.to_vec(), nbt.try_into().unwrap()))
    }

    pub fn update_local_player(&mut self, entity: &Entity) -> Result<(), WorldError> {
        self.update_entity(entity)
    }

    pub fn get_entities(&mut self) -> Result<Vec<Entity>, WorldError> {
        self.get_nbt_vec_from_db(b"actorprefix").map(|tags| {
            Ok(tags
                .into_iter()
                .map(|(key, tag)| Entity::new(key, tag.try_into().unwrap()))
                .collect())
        })?
    }

    pub fn update_entity(&mut self, entity: &Entity) -> Result<(), WorldError> {
        let db = self.get_db_mut()?;
        let mut buf = Vec::new();
        entity
            .nbt
            .clone()
            .write(&mut buf, &mut LittleEndian)
            .map_err(WorldError::NBTWriteError)?;
        let _ = db.put(entity.get_db_key(), &buf);
        let _ = db.flush();
        Ok(())
    }
}
