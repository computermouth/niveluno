
use std::vec;

use gltf;
use serde::{Deserialize, Serialize};
use serde_json;
use rmp_serde;

#[derive(Debug)]
struct NmccError<'a>(&'a str);
impl<'a> std::fmt::Display for NmccError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NmccError: {:?}", self.0)
    }
}
impl<'a> std::error::Error for NmccError<'a> {}

#[derive(Debug, Deserialize, Serialize)]
struct DecorReference {
    pub vertex_count: usize,
    pub vertices: Vec<f32>,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub texture: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EntityReference {
    pub vertex_count: usize,
    pub animation_count: usize,
    pub vertices: Vec<f32>,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub texture: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
enum Reference {
    Decor(DecorReference),
    Entity(EntityReference),
}

#[derive(Debug, Deserialize, Serialize)]
struct DecorInstance {
    pub index: usize,
    pub location: [f32;3],
    pub rotation: [f32;3],
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneralInstance {
    pub index: usize,
    pub location: [f32;3],
    pub rotation: [f32;3],
    pub params: Vec<(String, String)>
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerInstance {
    pub location: [f32;3],
    pub rotation: [f32;3],
}

#[derive(Debug, Deserialize, Serialize)]
struct LightInstance {
    pub location: [f32;3],
    pub color: [u8;3],
}

#[derive(Debug, Deserialize, Serialize)]
struct TriggerInstance {
    pub location: [f32;3],
    pub radius: f32,
}

#[derive(Debug, Deserialize, Serialize)]
enum EntityInstance {
    General(GeneralInstance),
    Player(PlayerInstance),
    Light(LightInstance),
    Trigger(TriggerInstance),
}

#[derive(Debug, Deserialize, Serialize)]
enum Instance {
    Decor(DecorInstance),
    Entity(EntityInstance),
}

#[derive(Debug, Deserialize, Serialize)]
struct Map {
    pub refs: Vec<Reference>,
    pub inst: Vec<Instance>
}

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Quat4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

// wow this fuckin sucks
fn trs_from_decomp (i: ([f32; 3], [f32; 4], [f32; 3])) -> (Vec3, Quat4, Vec3) {
    (
        Vec3 {
            x: i.0[0],
            y: i.0[1],
            z: i.0[2],
        },
        Quat4 {
            x: i.1[0],
            y: i.1[1],
            z: i.1[2],
            w: i.1[3],
        },
        Vec3 {
            x: i.2[0],
            y: i.2[1],
            z: i.2[2],
        }
    )
}

fn get_ref_obj(n: gltf::Node) -> Option<Reference> {

    let (c_pos, _, scale) = trs_from_decomp(n.transform().decomposed());

    if c_pos.x >= 0. ||
       c_pos.y >= 0. ||
       c_pos.z >= 0. {
        return None;
    }

    if scale.x <= 0. ||
       scale.y <= 0. ||
       scale.z <= 0. {
        eprint!("W: scale was negative, skipping {:?}", n.name());
        return None;
    }

    #[derive(Deserialize)]
    struct Extras<'a> {
        _type: Option<&'a str>,
        _entity: Option<&'a str>
    }

    let extras: Option<Extras>;
    let jstr: String;

    if let Some(json_raw) = n.extras() {
        jstr = json_raw.to_string();
        if let Ok(e) = serde_json::from_str(&jstr) {
            extras = e;
        } else {
            return None;
        }
    } else {
        eprint!("W: node in reference zone, with no extras {:?}", n.name());
        return None;
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum EntityInstance {
        General(GeneralInstance),
        Player(PlayerInstance),
        Light(LightInstance),
        Trigger(TriggerInstance),
    }

    match extras {
        Some(Extras{_type: Some("decor"), ..}) => None, // do decor
        Some(Extras{_type: Some("entity"), _entity: Some("player")}) => None, // do entity - player
        Some(Extras{_type: Some("entity"), _entity: Some("light")}) => None, // do entity - light
        Some(Extras{_type: Some("entity"), _entity: Some("trigger")}) => None, // do entity - trigger
        Some(Extras{_type: Some("entity"), _entity: Some(_)}) => None, // do entity - general
        Some(Extras{_type: Some(_), ..}) => None, // warn, unknown type
        Some(Extras{_type: None, ..}) => None, // warn, no type
        None => None, // warn no extras
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file.{{gltf,glb}}>", args[0]);
        return Err(Box::new(NmccError("Malformed or missing arguments")));
    }

    let path = &args[1];

    let (document, buffers, images) = gltf::import(path)?;

    let map = Map{ refs: vec![], inst: vec![] };

    // wasteful looping over it twice.
    // pack refs
    let skip_refs: Vec<usize> = vec![];
    for (i , node) in document.nodes().enumerate() {
        if let Some(r) = get_ref_obj(node) {}
    }

    /*
    let m = Map {
        refs: vec![
            Reference::Decor(DecorReference { vertex_count: 2, vertices: vec![1., 2.], u: vec![3., 4.], v: vec![5., 6.], texture: vec![7, 8] }),
            Reference::Decor(DecorReference { vertex_count: 2, vertices: vec![2., 3.], u: vec![4., 5.], v: vec![6., 7.], texture: vec![8, 9] }),
            Reference::Entity(EntityReference { animation_count: 1, vertex_count: 2, vertices: vec![1., 2.], u: vec![3., 4.], v: vec![5., 6.], texture: vec![7, 8] }),
        ],
        inst: vec![
            Instance::Decor(DecorInstance{ index: 0, location: [1.,2.,3.], rotation: [0., 0., 0.]}),
            Instance::Decor(DecorInstance{ index: 1, location: [1.,2.,3.], rotation: [0., 0., 0.]}),
            Instance::Entity(EntityInstance::General(GeneralInstance{index: 2, location: [1.,2.,3.], rotation: [0., 0., 0.], params: vec![("a".to_string(),"b".to_string())]})),
            Instance::Entity(EntityInstance::Light(LightInstance{location: [1.,2.,3.], color: [10, 20, 30]})),
            Instance::Entity(EntityInstance::Player(PlayerInstance{location: [1.,2.,3.], rotation: [0., 0., 0.]})),
            Instance::Entity(EntityInstance::Trigger(TriggerInstance{location: [1.,2.,3.], radius: 10.})),
        ]
    };

    let mut buf = Vec::new();
    m.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;

    std::fs::write("map.mp", buf)?;

    let i = std::fs::read("map.mp")?;

    let mut de = rmp_serde::Deserializer::new(std::io::Cursor::new(&i[..]));
    let o: Map = Deserialize::deserialize(&mut de)?;
    */

    Ok(())
}

