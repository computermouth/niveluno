
use gltf;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug)]
struct NmccError<'a>(&'a str);
impl<'a> std::fmt::Display for NmccError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file.{{gltf,glb}}>", args[0]);
        return Err(Box::new(NmccError("Malformed or missing arguments")));
    }

    let path = &args[1];

    let (document, buffers, images) = gltf::import(path)?;

    for node in document.nodes() { }


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
    m.serialize(&mut Serializer::new(&mut buf))?;

    std::fs::write("map.mp", buf)?;

    let i = std::fs::read("map.mp")?;

    let mut de = Deserializer::new(std::io::Cursor::new(&i[..]));
    let o: Map = Deserialize::deserialize(&mut de)?;

    eprintln!("{:?}", m);
    eprintln!("{:?}", o);

    Ok(())
}

