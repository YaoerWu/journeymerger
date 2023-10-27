mod journey_map;
use journey_map::models::JourneyMapData;
use journey_map::JourneyMapInstance;
use std::io::Read;
use std::path::Path;

const JOURNEY_MAPS: &str = ".\\.minecraft\\journeymap\\data";
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let root_path = Path::new(&args[0]).parent().unwrap();
    let instance_path = root_path.join(JOURNEY_MAPS);
    let instance = JourneyMapInstance::from_path(&instance_path);
    if args.len() == 1 {
        //export mode
        for (id, map) in &instance.mp_maps {
            let data = JourneyMapData::read_save(&map.get_map_path());
            let exported = journey_map::ExportedSave { id: *id, data };
            if let Err(e) = exported.write(
                root_path
                    .join(Path::new(&format!("{}.bin", map.get_name())))
                    .as_path(),
            ) {
                println!("Error: {}", e);
            }
        }
    } else if args.len() == 2 {
        //import mode
        match journey_map::ExportedSave::read(Path::new(&args[1])) {
            Ok(imported) => {
                instance.merge_imported(imported);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        panic!("Invalid arguments");
    }
    pause();
}
fn pause() {
    let mut buffer = [0u8; 1];
    println!("按任意键继续...");
    std::io::stdin().read_exact(&mut buffer).unwrap();
}
