use std::collections::HashMap;
use std::{fs::OpenOptions, vec};

use gltf::{self, json::image::MimeType};
use rmp_serde;
use serde::{Deserialize, Serialize};
use serde_json;

mod big_buffer;

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
    pub vertices: Vec<u32>,
    pub uvs: Vec<u32>,
    pub texture: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct EntityReference {
    // animation<vertices<u32>>
    pub vertices: Vec<Vec<u32>>,
    pub uvs: Vec<u32>,
    pub texture: u32,
}

#[derive(Debug, Deserialize, Serialize)]
enum Reference {
    Decor(DecorReference),
    Entity(EntityReference),
}

// todo, this seems like a better abstraction, which doesn't
// require a nmcc change when adding new special entities
// downside of maybe doing a few string compares on map load
struct EntityInstance2 {
    // names[index] == player, but also reference[index] == __nomodel
    // names[index] == ogre, but also reference[index] == ${ogre_reference}
    pub index: u32,
    pub params: Vec<(u32,u32)>, // indexes to (k,v)
    pub rotation: u32, // quat
}

#[derive(Debug, Deserialize, Serialize)]
struct DecorInstance {
    pub index: usize,
    pub location: [f32; 3], // TODO -- u32 -> vert
    pub rotation: [f32; 3], // TODO -- u32 -> quat
}

#[derive(Debug, Deserialize, Serialize)]
struct GeneralInstance {
    pub index: usize,
    pub location: [f32; 3], // TODO -- u32 -> vert
    pub rotation: [f32; 3], // TODO -- u32 -> quat
    pub params: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerInstance {
    pub location: [f32; 3], // TODO -- u32 -> vert
    pub rotation: [f32; 3], // TODO -- u32 -> quat
}

#[derive(Debug, Deserialize, Serialize)]
struct LightInstance {
    pub location: [f32; 3], // TODO -- u32 -> vert
    pub color: [u8; 3],
}

#[derive(Debug, Deserialize, Serialize)]
struct TriggerInstance {
    pub location: [f32; 3], // TODO -- u32 -> vert
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
    pub inst: Vec<Instance>,
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
fn trs_from_decomp(i: ([f32; 3], [f32; 4], [f32; 3])) -> (Vec3, Quat4, Vec3) {
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
        },
    )
}

fn get_ref_obj(
    n: gltf::Node,
    b: &Vec<gltf::buffer::Data>,
    bb: &mut big_buffer::BigBuffer,
) -> Option<Reference> {
    let (c_pos, _, scale) = trs_from_decomp(n.transform().decomposed());

    // not completely negative
    if c_pos.x >= 0. && c_pos.y >= 0. && c_pos.z >= 0. {
        return None;
    }

    if scale.x <= 0. || scale.y <= 0. || scale.z <= 0. {
        eprint!("W: scale was negative, skipping {:?}", n.name());
        return None;
    }

    #[derive(Deserialize)]
    struct Extras<'a> {
        _type: Option<&'a str>,
        _entity: Option<&'a str>,
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
        Some(Extras {
            _type: Some("decor"),
            ..
        }) => parse_ref_decor(n, b, bb), // do decor
        Some(Extras {
            _type: Some("entity"),
            _entity: Some("player"),
        }) => print_name(n), // do entity - player
        Some(Extras {
            _type: Some("entity"),
            _entity: Some("light"),
        }) => print_name(n), // do entity - light
        Some(Extras {
            _type: Some("entity"),
            _entity: Some("trigger"),
        }) => print_name(n), // do entity - trigger
        Some(Extras {
            _type: Some("entity"),
            _entity: Some(_),
        }) => print_name(n), // do entity - general
        Some(Extras { _type: Some(_), .. }) => None, // warn, unknown type
        Some(Extras { _type: None, .. }) => None,    // warn, no type
        None => None,                                // warn no extras
    }
}

fn u32s_from_acc(
    u32_acc: &gltf::Accessor,
    b: &Vec<gltf::buffer::Data>,
    n: Option<&str>,
) -> Option<Vec<u32>> {
    let u32_view = u32_acc.view().or_else(|| {
        eprintln!("W: {:?} couldn't get view", n);
        None
    })?;
    let u32_buffer_index = u32_view.buffer().index();
    let u32_data = &b[u32_buffer_index];

    let u32_start: usize = u32_view.offset();
    let u32_end = u32_start + u32_view.length();
    let u32_data_slice = &u32_data[u32_start..u32_end];

    // Determine the component type and read indices accordingly
    match u32_acc.data_type() {
        gltf::accessor::DataType::U16 => Some(
            u32_data_slice
                .chunks(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]) as u32)
                .collect(),
        ),
        gltf::accessor::DataType::U32 => Some(
            u32_data_slice
                .chunks(4)
                .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect(),
        ),
        e => panic!("u32s_from_acc: Unsupported component type: {:?}", e),
    }
}

fn f32s_from_acc(
    f32_acc: &gltf::Accessor,
    b: &Vec<gltf::buffer::Data>,
    n: Option<&str>,
) -> Option<Vec<f32>> {
    let f32_view = f32_acc.view().or_else(|| {
        eprintln!("W: {:?} couldn't get pos view", n);
        None
    })?;
    let f32_buffer_index = f32_view.buffer().index();
    let f32_data = &b[f32_buffer_index];

    let f32_start = f32_view.offset();
    let f32_end = f32_start + f32_view.length();
    let f32_data_slice = &f32_data[f32_start..f32_end];

    // Determine the component type and read indices accordingly
    match f32_acc.data_type() {
        gltf::accessor::DataType::F32 => Some(
            f32_data_slice
                .chunks(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect(),
        ),
        e => panic!("f32s_from_acc: Unsupported component type: {:?}", e),
    }
}

fn image_from_prim(prim: &gltf::Primitive, b: &Vec<gltf::buffer::Data>) -> Option<Vec<u8>> {
    let bct = prim
        .material()
        .pbr_metallic_roughness()
        .base_color_texture()?;

    let source = bct.texture().source().source();

    match source {
        gltf::image::Source::View { view, mime_type } => {
            if mime_type != "image/png" {
                eprintln!("E: texture mimetype not supported {}", mime_type);
                return None;
            }

            let buffer = &b[view.buffer().index()];

            let start = view.offset() as usize;
            let end = start + view.length() as usize;

            let data_slice = &buffer[start..end].to_vec();

            Some(data_slice.to_vec())
        }
        gltf::image::Source::Uri { .. } => None,
    }
}

fn parse_ref_decor(
    n: gltf::Node,
    b: &Vec<gltf::buffer::Data>,
    bb: &mut big_buffer::BigBuffer,
) -> Option<Reference> {

    let mesh = n.mesh().or_else(|| {
        eprintln!("W: {:?} has no mesh", n.name());
        None
    })?;

    let primitives = &mut mesh.primitives();
    // todo, fuck this
    if primitives.len() != 1 {
        eprintln!("W: {:?} mesh has multiple primitives", n.name());
        return None;
    }

    let z_prim = primitives.nth(0).or_else(|| {
        eprintln!("W: {:?} mesh has no zeroth primitive", n.name());
        None
    })?;

    let ind_acc = z_prim.indices().or_else(|| {
        eprintln!("W: {:?} has no index accessor", n.name());
        None
    })?;
    let indices = u32s_from_acc(&ind_acc, b, n.name()).or_else(|| {
        eprintln!("W: {:?} couldn't collect indices", n.name());
        None
    })?;

    let pos_acc = z_prim
        .attributes()
        .find(|a| a.0 == gltf::Semantic::Positions)
        .or_else(|| {
            eprintln!("W: {:?} has no position accessor", n.name());
            None
        })?
        .1;
    let positions = f32s_from_acc(&pos_acc, b, n.name()).or_else(|| {
        eprintln!("W: {:?} couldn't collect positions", n.name());
        None
    })?;

    let uv_acc = z_prim
        .attributes()
        .find(|a| a.0 == gltf::Semantic::TexCoords(0))
        .or_else(|| {
            eprintln!("W: {:?} has no texcoords accessor", n.name());
            None
        })?
        .1;
    let uvs = f32s_from_acc(&uv_acc, b, n.name()).or_else(|| {
        eprintln!("W: {:?} couldn't collect uvs", n.name());
        None
    })?;

    // push positions to floatbuffer,
    // store indicies
    let mut out_pos = Vec::new();
    for i in 0..indices.len() {
        let f1 = positions[indices[i] as usize * 3 + 0];
        let f2 = positions[indices[i] as usize * 3 + 1];
        let f3 = positions[indices[i] as usize * 3 + 2];

        let index = bb.add_sequence(big_buffer::HashItem::Vert([f1, f2, f3]));

        out_pos.push(index);
    }

    // push uvs to floatbuffer,
    // store indicies
    let mut out_uvs = Vec::new();
    for i in 0..indices.len() {
        let f1 = uvs[indices[i] as usize * 2 + 0];
        let f2 = uvs[indices[i] as usize * 2 + 1];

        let index = bb.add_sequence(big_buffer::HashItem::Uv__([f1, f2]));

        out_uvs.push(index);
    }

    let image = image_from_prim(&z_prim, b).or_else(|| {
        eprintln!("W: {:?} z_prim has no image", n.name());
        None
    })?;

    Some(Reference::Decor(DecorReference {
        vertices: out_pos,
        uvs: out_uvs,
        // todo, quat
        texture: bb.add_image(image),
    }))
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

    let (document, buffers, _) = gltf::import(path)?;

    let map = Map {
        refs: vec![],
        inst: vec![],
    };

    let bb = &mut big_buffer::BigBuffer::new();

    // wasteful looping over it twice.
    // pack refs
    let skip_refs: Vec<usize> = vec![];
    for (i, node) in document.nodes().enumerate() {
        if let Some(r) = get_ref_obj(node, &buffers, bb) {}
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
