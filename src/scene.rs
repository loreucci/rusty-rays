use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::camera::{Camera, CameraDescription};
use crate::material::{create_material, MaterialDescription, MaterialPtr};
use crate::objects::{create_object, ObjectDescription, World};

pub struct Scene {
    pub world: World,
    pub camera: Camera,
}

#[derive(Serialize, Deserialize)]
struct ObjectWithMaterialDescription {
    material: String,
    #[serde(flatten)]
    desc: ObjectDescription,
}

#[derive(Serialize, Deserialize)]
struct SceneDescription {
    materials: HashMap<String, MaterialDescription>,
    world: Vec<ObjectWithMaterialDescription>,
    camera: CameraDescription,
}

pub fn parse_scene(filepath: &str) -> Result<Scene, String> {
    // read file contents
    let contents = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };

    // parse entire scene description
    let s: SceneDescription = match serde_json::from_str(&contents.to_owned()) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    // materials
    let mut materials: HashMap<String, MaterialPtr> = HashMap::new();
    for (key, value) in &s.materials {
        materials.insert(key.clone(), create_material(value));
    }

    // world
    let mut world = World::new();
    for obj in &s.world {
        let matptr = &materials.get(&obj.material);
        let m = match matptr {
            Some(v) => v,
            None => return Err(format!("material '{}' not defined", &obj.material)),
        };
        world.add(&create_object(&obj.desc, m));
    }

    // camera
    let c = &s.camera;
    let camera = Camera::from(c);

    Ok(Scene { world, camera })
}
