pub mod models;
pub mod save;
use std::{collections::HashMap, io::Write, path::Path};
use thiserror::Error;

use save::JourneyMapSave;
use serde::{Deserialize, Serialize};

use self::models::JourneyMapData;
pub enum JourneyMapType {
    // SinglePlayer,
    MultiPlayer(u128),
}
pub struct JourneyMapInstance<'a> {
    // pub sp_maps: Vec<JourneyMapSave<'a>>,
    pub mp_maps: HashMap<u128, JourneyMapSave<'a>>,
}
impl<'a> JourneyMapInstance<'a> {
    pub fn from_path(root_path: &'a Path) -> JourneyMapInstance<'a> {
        // let mut sp_maps = Vec::new();
        let mut mp_maps = HashMap::new();
        // for entry in root_path.join("sp").read_dir().unwrap() {
        //     let dir = entry.unwrap();
        //     if let Ok(journey_map) = JourneyMapSave::from_path(
        //         root_path,
        //         dir.path().as_path(),
        //         JourneyMapType::SinglePlayer,
        //     ) {
        //         sp_maps.push(journey_map);
        //     } else {
        //         continue;
        //     }
        // }
        for entry in root_path.join("mp").read_dir().unwrap() {
            let dir = entry.unwrap();
            if let Ok(journey_map) = JourneyMapSave::from_path(
                root_path,
                dir.path().as_path(),
                JourneyMapType::MultiPlayer(0),
            ) {
                mp_maps.insert(journey_map.get_id(), journey_map);
            } else {
                continue;
            }
        }
        // JourneyMapInstance { sp_maps, mp_maps }
        JourneyMapInstance { mp_maps }
    }
    pub fn merge_imported(&self, imported: ExportedSave) {
        let save_path = self.mp_maps[&imported.id].get_map_path();
        JourneyMapData::read_save(&save_path)
            .merge(imported.data)
            .write_save(&save_path);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportedSave {
    pub id: u128,
    pub data: models::JourneyMapData,
}
impl ExportedSave {
    pub fn read(path: &Path) -> Result<ExportedSave, ExportError> {
        let file = std::fs::File::open(path).map_err(|_| ExportError::FileNotFound)?;
        let reader = std::io::BufReader::new(file);
        bincode::deserialize_from(reader).map_err(|_| ExportError::InvalidFile)
    }
    pub fn write(&self, path: &Path) -> Result<(), ExportError> {
        let file = std::fs::File::create(path).map_err(|_| ExportError::WriteError)?;
        let mut writer = std::io::BufWriter::new(file);
        writer
            .write_all(&bincode::serialize(self).map_err(|_| ExportError::InvalidFile)?)
            .map_err(|_| ExportError::WriteError)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("File not found")]
    FileNotFound,
    #[error("Invalid file")]
    InvalidFile,
    #[error("Write error")]
    WriteError,
}
