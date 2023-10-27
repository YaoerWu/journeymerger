use std::path::{Path, PathBuf};

use super::JourneyMapType;
use anyhow::Result;

pub struct JourneyMapSave<'a> {
    root_path: &'a Path,
    name: String,
    map_type: JourneyMapType,
}
impl<'a> JourneyMapSave<'a> {
    pub fn from_path(
        root_path: &'a Path,
        path: &'_ Path,
        map_type: JourneyMapType,
    ) -> Result<JourneyMapSave<'a>> {
        match map_type {
            // JourneyMapType::SinglePlayer => Ok(JourneyMapSave {
            //     root_path,
            //     name: path
            //         .file_name()
            //         .unwrap()
            //         .to_str()
            //         .unwrap()
            //         .chars()
            //         .map(|c| if c == '~' { ' ' } else { c })
            //         .collect(),
            //     map_type: JourneyMapType::SinglePlayer,
            // }),
            JourneyMapType::MultiPlayer(_) => {
                let file_name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split('_')
                    .collect::<Vec<&str>>();
                if file_name.len() != 2 {
                    return Err(anyhow::anyhow!("Invalid file name"));
                }
                let name = file_name[0]
                    .chars()
                    .map(|c| if c == '~' { ' ' } else { c })
                    .collect::<String>();
                let uuid = u128::from_str_radix(
                    &file_name[1]
                        .chars()
                        .filter(|c| *c != '~')
                        .collect::<String>(),
                    16,
                )
                .unwrap();
                Ok(JourneyMapSave {
                    root_path,
                    name,
                    map_type: JourneyMapType::MultiPlayer(uuid),
                })
            }
        }
    }
    pub fn get_map_path(&self) -> PathBuf {
        let path = self.root_path.join(match self.map_type {
            // JourneyMapType::SinglePlayer => "sp",
            JourneyMapType::MultiPlayer(_) => "mp",
        });
        let name = self
            .name
            .chars()
            .map(|c| if c == ' ' { '~' } else { c })
            .collect::<String>();
        let map_type = match self.map_type {
            // JourneyMapType::SinglePlayer => String::from(""),
            JourneyMapType::MultiPlayer(uuid) => {
                let uuid = format!("{:x}", uuid);
                format!(
                    "_{}~{}~{}~{}~{}",
                    &uuid[0..8],
                    &uuid[8..12],
                    &uuid[12..16],
                    &uuid[16..20],
                    &uuid[20..]
                )
            }
        };
        path.join(name + &map_type)
    }
    pub fn get_id(&self) -> u128 {
        match self.map_type {
            // JourneyMapType::SinglePlayer => 0,
            JourneyMapType::MultiPlayer(id) => id,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
