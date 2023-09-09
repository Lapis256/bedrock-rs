use std::{fmt::Debug, io::Error as IOError};

use thiserror::Error as ThisError;
use zuri_nbt::{
    err::{ErrorPath, ReadError, WriteError},
    serde::{DeserializeError, SerializeError},
};

#[derive(Debug, ThisError)]
pub enum WorldError {
    #[error("IO error: {0}")]
    IOError(IOError),
    #[error("NBT read error: {0}")]
    NBTReadError(ErrorPath<ReadError>),
    #[error("NBT write error: {0}")]
    NBTWriteError(ErrorPath<WriteError>),
    #[error("NBT deserialize error: {0}")]
    NBTDeserializeError(ErrorPath<DeserializeError>),
    #[error("NBT serialize error: {0}")]
    NBTSerializeError(ErrorPath<SerializeError>),
    #[error("DB was closed")]
    DBClosed,
    #[error("DB value not found: {0:?}")]
    DBValueNotFound(Vec<u8>),
}
