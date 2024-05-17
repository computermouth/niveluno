
use std::vec;
use std::collections::HashMap;

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

fn get_ref_obj(n: gltf::Node, b: &Vec<gltf::buffer::Data>, im: &mut HashMap<[u32;3], usize>, ib: &mut Vec<[f32;3]>) -> Option<Reference> {

    let (c_pos, _, scale) = trs_from_decomp(n.transform().decomposed());

    // not completely negative
    if c_pos.x >= 0. &&
       c_pos.y >= 0. &&
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
        Some(Extras{_type: Some("decor"), ..}) => parse_ref_decor(n, b, im, ib), // do decor
        Some(Extras{_type: Some("entity"), _entity: Some("player")}) => print_name(n), // do entity - player
        Some(Extras{_type: Some("entity"), _entity: Some("light")}) => print_name(n), // do entity - light
        Some(Extras{_type: Some("entity"), _entity: Some("trigger")}) => print_name(n), // do entity - trigger
        Some(Extras{_type: Some("entity"), _entity: Some(_)}) => print_name(n), // do entity - general
        Some(Extras{_type: Some(_), ..}) => None, // warn, unknown type
        Some(Extras{_type: None, ..}) => None, // warn, no type
        None => None, // warn no extras
    }

}

fn parse_ref_decor(n: gltf::Node, b: &Vec<gltf::buffer::Data>, im: &mut HashMap<[u32;3], usize>, ib: &mut Vec<[f32;3]>) -> Option<Reference> {

    let vertex_count = 0;
    let vertices: Vec<f32> = vec![];
    let u: Vec<f32> = vec![];
    let v: Vec<f32> = vec![];
    let texture: Vec<u8> = vec![];

    let mesh = n.mesh().or_else(|| {eprintln!("W: {:?} has no mesh", n.name()); None})?;

    let primitives = &mut mesh.primitives();
    // todo, fuck this
    if primitives.len() != 1 {
        eprintln!("W: {:?} mesh has multiple primitives", n.name());
        return None;
    }

    let z_prim = primitives.nth(0).or_else(||{ eprintln!("W: {:?} mesh has no zeroth primitive", n.name()); None})?;

    let pos_acc = z_prim.attributes().find(|a| {
        a.0 == gltf::Semantic::Positions
    }).or_else(|| {eprintln!("W: {:?} has no position accessor", n.name()); None})?.1;
    let uv_acc = z_prim.attributes().find(|a| {
        a.0 == gltf::Semantic::TexCoords(0)
    }).or_else(|| {eprintln!("W: {:?} has no texcoords_0 accessor", n.name()); None})?.1;
    let ind_acc = z_prim.indices().or_else(|| {eprintln!("W: {:?} has no index accessor", n.name()); None})?;

    let index_count = ind_acc.count();
    let view = ind_acc.view().or_else(|| {eprintln!("W: {:?} couldn't get view", n.name()); None})?;
    let buffer_index = view.buffer().index();
    let data = &b[buffer_index];

    let start = view.offset();
    let end = start + view.length();
    let data_slice = &data[start..end];

    // Determine the component type and read indices accordingly
    match ind_acc.data_type() {
        gltf::accessor::DataType::U16 => {}
        e => panic!("Unsupported index component type: {:?}", e),
    }
    
    let vert_indices: Vec<u16> = data_slice.chunks(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    
    println!("{:?}[{}] {:?}", n.name(), index_count, vert_indices);

    let pos_count = pos_acc.count();
    let pos_view = pos_acc.view().or_else(|| {eprintln!("W: {:?} couldn't get pos view", n.name()); None})?;
    let pos_buffer_index = pos_view.buffer().index();
    let pos_data = &b[pos_buffer_index];

    let pos_start = pos_view.offset();
    let pos_end = pos_start + pos_view.length();
    let pos_data_slice = &pos_data[pos_start..pos_end];

    // Determine the component type and read indices accordingly
    match pos_acc.data_type() {
        gltf::accessor::DataType::F32 => {}
        e => panic!("Unsupported position component type: {:?}", e),
    }
    
    let vert_positions: Vec<f32> = pos_data_slice.chunks(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    println!("{:?}[{}] {:?}", n.name(), pos_count, vert_positions);

    let mut out_vind = Vec::new();

    for i in 0..index_count {
        let f1 = vert_positions[vert_indices[i] as usize * 3 + 0];
        let f2 = vert_positions[vert_indices[i] as usize * 3 + 1];
        let f3 = vert_positions[vert_indices[i] as usize * 3 + 2];

        let farr = [f1,f2,f3];
        let uarr = u32_arr_from_f32_arr(farr);

        eprintln!("[{}]: {:?} {:?}", vert_indices[i], farr, uarr);

        if let Some(found) = im.get(&uarr) {
            out_vind.push(*found);
        } else {
            ib.push(farr);
            im.insert(uarr, ib.len() - 1);
            out_vind.push(ib.len() - 1);
        }
    }

    println!("out_vind[{}]: {:?}", out_vind.len(), out_vind);
    println!("ib[{}] {:?}", ib.len(), ib);

    None
}

fn u32_arr_from_f32_arr(i: [f32;3]) -> [u32;3] {
    [i[0].to_bits(), i[1].to_bits(), i[2].to_bits()]
}

fn f32_arr_from_u32_arr(i: [u32;3]) -> [f32;3] {
    [f32::from_bits(i[0]),f32::from_bits(i[1]),f32::from_bits(i[2])]
}

fn print_name(n: gltf::Node) -> Option<Reference> {
    eprintln!("{:?}", n.name());
    None
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

    let mut index_map: HashMap<[u32;3], usize> = HashMap::new();
    let mut index_buffer: Vec<[f32;3]> = vec![];

    // wasteful looping over it twice.
    // pack refs
    let skip_refs: Vec<usize> = vec![];
    for (i , node) in document.nodes().enumerate() {
        if let Some(r) = get_ref_obj(node, &buffers, &mut index_map, &mut index_buffer) {}
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

