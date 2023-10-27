use image::io::Reader as ImageReader;
use image::GenericImage;
use image::GenericImageView;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Region {
    chunks: Vec<u8>,
}
impl Region {
    fn read_region(path: &Path) -> Region {
        let mut chunks = vec![];
        let mut img = File::open(path).unwrap();
        img.read_to_end(&mut chunks).unwrap();

        Region { chunks }
    }
    fn merge(&mut self, other: Region) {
        let mut self_img = ImageReader::new(Cursor::new(&self.chunks))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let other = ImageReader::new(Cursor::new(&other.chunks))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        for x in 0..32 {
            for y in 0..32 {
                if other.get_pixel(x * 16, y * 16)[3] == 255
                    && self_img.get_pixel(x * 16, y * 16)[3] == 0
                {
                    self_img
                        .copy_from(other.view(x * 16, y * 16, 16, 16).deref(), x * 16, y * 16)
                        .unwrap();
                }
            }
        }
        let mut chunks = Vec::new();
        self_img
            .write_to(&mut Cursor::new(&mut chunks), image::ImageOutputFormat::Png)
            .unwrap();
        self.chunks = chunks;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
enum LayerType {
    Day,
    Night,
    Cave(u32),
}

type Position = (i32, i32);
#[derive(Debug, Deserialize, Serialize)]
struct Layer {
    regions: HashMap<Position, Region>,
}
impl Layer {
    fn read_layer(path: &Path) -> Layer {
        let mut regions = HashMap::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_name = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split(',')
                .map(|f| f.parse::<i32>().unwrap_or_else(|_| panic!("{}", f)))
                .collect::<Vec<i32>>();
            let x = file_name[0];
            let z = file_name[1];
            regions.insert((x, z), Region::read_region(entry.path().as_path()));
        }
        Layer { regions }
    }
    fn merge(&mut self, other: Layer) {
        for (position, region) in other.regions {
            match self.regions.entry(position) {
                std::collections::hash_map::Entry::Occupied(mut e) => e.get_mut().merge(region),
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(region);
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Dimension {
    id: i32,
    layers: HashMap<LayerType, Layer>,
}
impl Dimension {
    fn read_dimension(path: &Path) -> Dimension {
        let id = path.file_name().unwrap().to_str().unwrap()[3..]
            .parse::<i32>()
            .unwrap();
        let mut layers = HashMap::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_str().unwrap().to_string();
            let layer_type = match file_name.as_str() {
                "day" => LayerType::Day,
                "night" => LayerType::Night,
                _ => LayerType::Cave(file_name.parse::<u32>().unwrap()),
            };
            layers.insert(layer_type, Layer::read_layer(entry.path().as_path()));
        }
        Dimension { id, layers }
    }
    fn merge(&mut self, other: Dimension) {
        for (layer_type, layer) in other.layers {
            match self.layers.entry(layer_type) {
                std::collections::hash_map::Entry::Occupied(mut e) => e.get_mut().merge(layer),
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(layer);
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Waypoint {
    id: String,
    name: String,
    icon: String,
    x: i64,
    y: i64,
    z: i64,
    r: u8,
    g: u8,
    b: u8,
    enable: bool,
    #[serde(rename = "type")]
    type_: String,
    origin: String,
    dimensions: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WaypointList {
    waypoints: Vec<Waypoint>,
}
impl WaypointList {
    fn read_waypoints(path: &Path) -> WaypointList {
        let mut waypoints = Vec::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file = File::open(entry.path().as_path()).unwrap();
            let reader = BufReader::new(file);
            let waypoint: Waypoint = serde_json::from_reader(reader).unwrap();
            waypoints.push(waypoint);
        }
        WaypointList { waypoints }
    }
    fn merge(&mut self, other: &WaypointList) {
        //TODO: merge waypoints
        // let mut map = HashMap::new();
        // self.waypoints.iter().for_each(|waypoint| {
        //     map.insert(&waypoint.id, waypoint);
        // });
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JourneyMapData {
    dimensions: HashMap<i32, Dimension>,
    waypoints: WaypointList,
}

impl JourneyMapData {
    pub fn read_save(path: &Path) -> JourneyMapData {
        let mut dimensions = HashMap::new();
        let mut waypoints = WaypointList { waypoints: vec![] };
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_str().unwrap().to_string();
            if file_name.starts_with("DIM") {
                let dimension = Dimension::read_dimension(entry.path().as_path());
                dimensions.insert(dimension.id, dimension);
            } else if file_name.starts_with("waypoints") {
                waypoints = WaypointList::read_waypoints(entry.path().as_path());
            }
        }
        JourneyMapData {
            dimensions,
            waypoints,
        }
    }
    pub fn write_save(&self, path: &Path) {
        for (id, dimension) in &self.dimensions {
            let dimension_path = path.join(format!("DIM{}", id));
            std::fs::create_dir_all(dimension_path.as_path()).unwrap();
            for (layer_type, layer) in &dimension.layers {
                let layer_path = dimension_path.join(match layer_type {
                    LayerType::Day => "day".to_string(),
                    LayerType::Night => "night".to_string(),
                    LayerType::Cave(id) => format!("{}", id),
                });
                std::fs::create_dir_all(layer_path.as_path()).unwrap();
                for (position, region) in &layer.regions {
                    let region_path = layer_path.join(format!("{},{}.png", position.0, position.1));
                    let mut file = File::create(region_path).unwrap();
                    file.write_all(&region.chunks).unwrap();
                }
            }
        }
        let waypoints_dir_path = path.join("waypoints");
        std::fs::create_dir_all(waypoints_dir_path.as_path()).unwrap();
        for waypoint in &self.waypoints.waypoints {
            let mut file = File::create(waypoints_dir_path.join(format!(
                "{}.json",
                waypoint.id.chars().map(|ch| match ch {
                    '/' | '\\' | '|' | '?' | '*' | '<' | '>' | ':' | '"' => '_',
                    _ => ch,
                }).collect::<String>()
            )))
            .unwrap();
            file.write_all(serde_json::to_string_pretty(waypoint).unwrap().as_bytes())
                .unwrap();
        }
    }
    pub fn merge(&mut self, other: JourneyMapData) -> &mut Self {
        self.waypoints.merge(&other.waypoints);
        for (id, dimension) in other.dimensions {
            match self.dimensions.entry(id) {
                std::collections::hash_map::Entry::Occupied(mut e) => e.get_mut().merge(dimension),
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(dimension);
                }
            }
        }
        self
    }
}
