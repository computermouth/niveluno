use std::{fs::OpenOptions, vec};

use gltf;
use rmp::{
    self,
    encode::{write_bin, write_bin_len, write_f32, write_str, write_str_len},
};
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

#[derive(Debug)]
struct DecorReference {
    pub name: u32,
    pub texture: u32,
    pub vertices: Vec<u32>,
    pub uvs: Vec<u32>,
}

#[derive(Debug)]
struct EntityReference {
    pub name: u32,
    pub vertices: Vec<Vec<u32>>,
    pub uvs: Vec<u32>,
    pub texture: u32,
}

#[derive(Debug)]
enum Reference {
    Decor(DecorReference),
    Entity(EntityReference),
}

#[derive(Debug)]
struct EntityInstance {
    // names[index] == player, but also reference[index] == __nomodel
    // names[index] == ogre, but also reference[index] == ${ogre_reference}
    pub index: Option<u32>,
    pub params: Vec<u32>, // indexes to [k,v,k,v,k,v] etc
    pub location: u32,    // u32 -> [f32;3]
    pub rotation: u32,    // u32 -> [f32;4]
    pub scale: u32,       // u32 -> [f32;3]
}

#[derive(Debug)]
struct DecorInstance {
    pub index: u32,
    pub location: u32, // u32 -> [f32;3]
    pub rotation: u32, // u32 -> [f32;4]
    pub scale: u32,    // u32 -> [f32;3]
}

#[derive(Debug)]
enum Instance {
    Decor(DecorInstance),
    Entity(EntityInstance),
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

fn json_string_pairs(json_str: &str) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();

    if let Ok(value) = serde_json::from_str(json_str) {
        if let serde_json::Value::Object(map) = value {
            for (key, value) in map {
                if let serde_json::Value::String(value_str) = value {
                    pairs.push((key, value_str));
                }
            }
        }
    } else {
        eprintln!("E: failed to parse json_str '{}'", json_str);
        return pairs;
    }

    pairs
}

fn get_reference(
    n: &gltf::Node,
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
        _noref: Option<&'a str>,
        _entity: Option<&'a str>,
        _decor: Option<&'a str>,
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

    match extras {
        Some(Extras {
            _noref: Some("true"),
            ..
        }) => None,
        Some(Extras {
            _type: Some("decor"),
            _decor: Some(name),
            ..
        }) => parse_ref_decor(n, b, bb, name),
        Some(Extras {
            _type: Some("entity"),
            _entity: Some(name),
            ..
        }) => parse_ref_entt(n, b, bb, name),
        Some(Extras {
            _type: Some("decor"),
            _decor: None,
            ..
        }) => {
            eprintln!("W: decor is missing name {:?}", n.name());
            None
        }
        Some(Extras {
            _type: Some("entity"),
            _entity: None,
            ..
        }) => {
            eprintln!("W: decor is missing name {:?}", n.name());
            None
        }
        Some(Extras { _type: Some(s), .. }) => {
            eprintln!("W: unknown type {s}");
            None
        }
        Some(Extras { _type: None, .. }) => {
            eprintln!("W: no type on reference {:?}", n.name());
            None
        }
        None => {
            eprintln!("W: no extras on reference {:?}", n.name());
            None
        }
    }
}

fn get_instance(n: &gltf::Node, bb: &mut big_buffer::BigBuffer) -> Option<Instance> {
    let (c_pos, rot, scale) = trs_from_decomp(n.transform().decomposed());

    // not really a good test for in positive space
    if c_pos.x < 0. || c_pos.y < 0. || c_pos.z < 0. {
        return None;
    }

    if scale.x <= 0. || scale.y <= 0. || scale.z <= 0. {
        eprint!("W: scale was negative, skipping {:?}", n.name());
        return None;
    }

    #[derive(Deserialize)]
    struct Extras<'a> {
        _type: Option<&'a str>,
        _decor: Option<&'a str>,
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

    let kvp = json_string_pairs(&jstr);

    match extras {
        Some(Extras {
            _type: Some("decor"),
            _decor: Some(d),
            ..
        }) => {
            let di = *bb.get_decor_index(d).or_else(|| {
                eprintln!("W: couldn't get decor index for {:?} {d}", n.name());
                None
            })?;

            Some(Instance::Decor(DecorInstance {
                index: di,
                location: bb.add_sequence(big_buffer::HashItem::Vert([c_pos.x, c_pos.y, c_pos.z])),
                rotation: bb.add_sequence(big_buffer::HashItem::Quat([rot.x, rot.y, rot.z, rot.w])),
                scale: bb.add_sequence(big_buffer::HashItem::Vert([scale.x, scale.y, scale.z])),
            }))
        }
        Some(Extras {
            _type: Some("entity"),
            _entity: Some(e),
            ..
        }) => {
            let mut ei = None;
            if !kvp.contains(&("_noref".to_string(), "true".to_string())) {
                ei = Some(*bb.get_entt_index(e).or_else(|| {
                    eprintln!("W: couldn't get entt index for {:?} {e}", n.name());
                    None
                })?);
            }

            let mut p = vec![];
            for (k, v) in kvp {
                p.push(bb.add_kv_string(&k));
                p.push(bb.add_kv_string(&v));
            }

            Some(Instance::Entity(EntityInstance {
                index: ei,
                params: p,
                location: bb.add_sequence(big_buffer::HashItem::Vert([c_pos.x, c_pos.y, c_pos.z])),
                rotation: bb.add_sequence(big_buffer::HashItem::Quat([rot.x, rot.y, rot.z, rot.w])),
                scale: bb.add_sequence(big_buffer::HashItem::Vert([scale.x, scale.y, scale.z])),
            }))
        }
        Some(Extras { _type: Some(s), .. }) => {
            eprintln!("W: unknown type {s}");
            None
        }
        Some(Extras { _type: None, .. }) => {
            eprintln!("W: no type on instance {:?}", n.name());
            None
        }
        None => {
            eprintln!("W: no extras on reference {:?}", n.name());
            None
        }
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
    n: &gltf::Node,
    b: &Vec<gltf::buffer::Data>,
    bb: &mut big_buffer::BigBuffer,
    name: &str,
) -> Option<Reference> {
    let mesh = n.mesh().or_else(|| {
        eprintln!("W: {:?} has no mesh", n.name());
        None
    })?;

    let primitives = &mut mesh.primitives();
    if primitives.len() == 0 {
        eprintln!("W: {:?} mesh has no primitives", n.name());
        return None;
    }

    let mut out_pos = vec![];
    let mut out_uvs = vec![];
    let mut out_img = vec![];

    for i in 0..primitives.len() {
        let prim = primitives.nth(0).or_else(|| {
            eprintln!("W: {:?} mesh has no zeroth primitive", n.name());
            None
        })?;

        let ind_acc = prim.indices().or_else(|| {
            eprintln!("W: {:?} has no index accessor", n.name());
            None
        })?;
        let indices = u32s_from_acc(&ind_acc, b, n.name()).or_else(|| {
            eprintln!("W: {:?} couldn't collect indices", n.name());
            None
        })?;

        let pos_acc = prim
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

        let uv_acc = prim
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
        for i in 0..indices.len() {
            let f1 = positions[indices[i] as usize * 3 + 0];
            let f2 = positions[indices[i] as usize * 3 + 1];
            let f3 = positions[indices[i] as usize * 3 + 2];

            let index = bb.add_sequence(big_buffer::HashItem::Vert([f1, f2, f3]));

            out_pos.push(index);
        }

        // push uvs to floatbuffer,
        // store indicies
        for i in 0..indices.len() {
            let f1 = uvs[indices[i] as usize * 2 + 0];
            let f2 = uvs[indices[i] as usize * 2 + 1];

            let index = bb.add_sequence(big_buffer::HashItem::Uv__([f1, f2]));

            out_uvs.push(index);
        }

        if i == 0 {
            out_img = image_from_prim(&prim, b).or_else(|| {
                eprintln!("W: {:?} z_prim has no image", n.name());
                None
            })?;
        } else {
            if image_from_prim(&prim, b).is_some_and(|i| i != out_img) {
                eprintln!(
                    "W: {:?}'s prim[{}] has a texture which differs from prim[0]'s",
                    n.name(),
                    i
                );
            }
        }
    }

    let name_id;
    if let Ok(n) = bb.add_decor_name(name) {
        name_id = n;
    } else {
        eprintln!("W: {:?} has duplicate name '{name}'", n.name());
        return None;
    }

    Some(Reference::Decor(DecorReference {
        name: name_id,
        vertices: out_pos,
        uvs: out_uvs,
        texture: bb.add_image(out_img),
    }))
}

fn parse_ref_entt(
    n: &gltf::Node,
    b: &Vec<gltf::buffer::Data>,
    bb: &mut big_buffer::BigBuffer,
    name: &str,
) -> Option<Reference> {
    let mesh = n.mesh().or_else(|| {
        eprintln!("W: {:?} has no mesh", n.name());
        None
    })?;

    let primitives = &mut mesh.primitives();
    if primitives.len() == 0 {
        eprintln!("W: {:?} mesh has no primitives", n.name());
        return None;
    }

    let mut out_pos = vec![];
    let mut out_uvs = vec![];
    let mut out_img = vec![];

    for i in 0..primitives.len() {
        let prim = primitives.nth(0).or_else(|| {
            eprintln!("W: {:?} mesh has no zeroth primitive", n.name());
            None
        })?;

        let ind_acc = prim.indices().or_else(|| {
            eprintln!("W: {:?} has no index accessor", n.name());
            None
        })?;
        let indices = u32s_from_acc(&ind_acc, b, n.name()).or_else(|| {
            eprintln!("W: {:?} couldn't collect indices", n.name());
            None
        })?;

        let pos_acc = prim
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

        let uv_acc = prim
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
        let mut base_pos = vec![];
        for i in 0..indices.len() {
            let f1 = positions[indices[i] as usize * 3 + 0];
            let f2 = positions[indices[i] as usize * 3 + 1];
            let f3 = positions[indices[i] as usize * 3 + 2];

            let index = bb.add_sequence(big_buffer::HashItem::Vert([f1, f2, f3]));

            base_pos.push(index);
        }
        out_pos.push(base_pos.clone());

        // gather animations
        let mut targets = prim.morph_targets();
        for j in 0..targets.len() {
            if let Some(t) = targets.nth(0) {
                if let Some(morph_acc) = t.positions() {
                    if let Some(morphs) = f32s_from_acc(&morph_acc, b, n.name()) {
                        if morphs.len() != base_pos.len() {
                            eprintln!(
                                "W: wrong len morph target on {:?}'s prim[{i}] morph[{j}]",
                                n.name()
                            );
                            continue;
                        }

                        let mut out_morph = vec![];
                        for k in 0..base_pos.len() {
                            if let Some(vert) = bb.get_vert_at(base_pos[k] as usize) {
                                let f1 = vert[0] + morphs[k * 3 + 0];
                                let f2 = vert[1] + morphs[k * 3 + 1];
                                let f3 = vert[2] + morphs[k * 3 + 2];

                                let index =
                                    bb.add_sequence(big_buffer::HashItem::Vert([f1, f2, f3]));
                                out_morph.push(index);
                            } else {
                                eprintln!(
                                    "W: bad base_pos lookup for {:?}'s prim[{i}] morph[{j}][{k}]",
                                    n.name()
                                );
                            }
                        }
                        out_pos.push(out_morph);
                        continue;
                    }
                }
            }
            eprintln!(
                "W: weird morph target on {:?}'s prim[{i}] morph[{j}]",
                n.name()
            )
        }

        // push uvs to floatbuffer,
        // store indicies
        for i in 0..indices.len() {
            let f1 = uvs[indices[i] as usize * 2 + 0];
            let f2 = uvs[indices[i] as usize * 2 + 1];

            let index = bb.add_sequence(big_buffer::HashItem::Uv__([f1, f2]));

            out_uvs.push(index);
        }

        if i == 0 {
            out_img = image_from_prim(&prim, b).or_else(|| {
                eprintln!("W: {:?} z_prim has no image", n.name());
                None
            })?;
        } else {
            if image_from_prim(&prim, b).is_some_and(|i| i != out_img) {
                eprintln!(
                    "W: {:?}'s prim[{}] has a texture which differs from prim[0]'s",
                    n.name(),
                    i
                );
            }
        }
    }

    let name_id;
    if let Ok(n) = bb.add_entt_name(name) {
        name_id = n;
    } else {
        eprintln!("W: {:?} has duplicate name '{name}'", n.name());
        return None;
    }

    Some(Reference::Entity(EntityReference {
        name: name_id,
        vertices: out_pos,
        uvs: out_uvs,
        texture: bb.add_image(out_img),
    }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file.{{gltf,glb}}>", args[0]);
        return Err(Box::new(NmccError("Malformed or missing arguments")));
    }

    let path = &args[1];

    let (document, buffers, _) = gltf::import(path)?;

    let mut map_ref_decs = vec![];
    let mut map_ref_entt = vec![];
    let mut map_ins_decs = vec![];
    let mut map_ins_entt = vec![];

    let bb = &mut big_buffer::BigBuffer::new();

    // first lap we only check for references,
    // then we can enforce later that field entitites must
    // have a matching reference
    let mut reference_index = vec![];
    for (i, node) in document.nodes().enumerate() {
        if let Some(r) = get_reference(&node, &buffers, bb) {
            match r {
                Reference::Decor(d) => map_ref_decs.push(d),
                Reference::Entity(e) => map_ref_entt.push(e),
            }
            reference_index.push(i);
        }
    }

    // now parse for instances
    for (i, node) in document.nodes().enumerate() {
        if reference_index.contains(&i) {
            // was a reference, skip
            continue;
        }
        if let Some(r) = get_instance(&node, bb) {
            match r {
                Instance::Decor(d) => map_ins_decs.push(d),
                Instance::Entity(e) => map_ins_entt.push(e),
            }
        }
    }

    let mut buf = vec![];

    // top-level array
    // rmp::encode::write_array_len(&mut buf, 8)?;
    rmp::encode::write_array_len(&mut buf, 6)?;

    {
        // floats
        let floats = bb.get_f32_data();
        rmp::encode::write_array_len(&mut buf, floats.len() as u32)?;
        for f in floats {
            write_f32(&mut buf, *f)?;
        }
    }

    {
        // img_data
        let img_data = bb.get_img_data();
        rmp::encode::write_array_len(&mut buf, img_data.len() as u32)?;
        for img in img_data {
            write_bin(&mut buf, img)?;
        }
    }

    {
        // drn_data
        let drn_data = bb.get_drn_data();
        rmp::encode::write_array_len(&mut buf, drn_data.len() as u32)?;
        for drn in drn_data {
            write_str_len(&mut buf, (drn.len() + 1) as u32)?;
            write_str(&mut buf, drn)?;
        }
    }

    {
        // ern_data
        let ern_data = bb.get_ern_data();
        rmp::encode::write_array_len(&mut buf, ern_data.len() as u32)?;
        for ern in ern_data {
            write_str_len(&mut buf, (ern.len() + 1) as u32)?;
            write_str(&mut buf, ern)?;
        }
    }

    {
        // kvs_data
        let kvs_data = bb.get_kvs_data();
        rmp::encode::write_array_len(&mut buf, kvs_data.len() as u32)?;
        for kvs in kvs_data {
            write_str_len(&mut buf, (kvs.len() + 1) as u32)?;
            write_str(&mut buf, kvs)?;
        }
    }

    eprintln!("map_ref_decs: {:?}\n", map_ref_decs);
    eprintln!("map_ref_entt: {:?}\n", map_ref_entt);
    eprintln!("map_ins_decs: {:?}\n", map_ins_decs);
    eprintln!("map_ins_entt: {:?}\n", map_ins_entt);

    {
        // map_ref_decs
        rmp::encode::write_array_len(&mut buf, map_ref_decs.len() as u32)?;
        for dec in map_ref_decs {
            rmp::encode::write_array_len(&mut buf, 4)?;
            rmp::encode::write_u32(&mut buf, dec.name)?;
            rmp::encode::write_u32(&mut buf, dec.texture)?;
            {
                // verts
                rmp::encode::write_array_len(&mut buf, dec.vertices.len() as u32)?;
                for i in dec.vertices {
                    rmp::encode::write_u32(&mut buf, i)?;
                }
            }
            {
                // uvs
                rmp::encode::write_array_len(&mut buf, dec.uvs.len() as u32)?;
                for i in dec.uvs {
                    rmp::encode::write_u32(&mut buf, i)?;
                }
            }
        }
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

    */
    std::fs::write("map.mp", buf)?;

    // let i = std::fs::read("map.mp")?;

    // let mut de = rmp_serde::Deserializer::new(std::io::Cursor::new(&i[..]));
    // let o: Map = Deserialize::deserialize(&mut de)?;

    Ok(())
}
