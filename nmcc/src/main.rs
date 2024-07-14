use std::vec;

use gltf;
use mparse::{self, types::*};
use serde::Deserialize;
use serde_json;

mod big_buffer;

#[derive(Debug)]
enum Reference {
    Decor(DecorReference),
    Entity(EntityReference),
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
            let mut has_ref = true;
            if kvp.contains(&("_noref".to_string(), "true".to_string())) {
                has_ref = false;
            }

            let ei = match has_ref {
                true => *bb.get_entt_index(e).or_else(|| {
                    eprintln!("W: couldn't get entt index for {:?} {e}", n.name());
                    None
                })?,
                false => bb.add_noref_name(e),
            };

            let mut p = vec![];
            for (k, v) in kvp {
                p.push(bb.add_kv_string(&k));
                p.push(bb.add_kv_string(&v));
            }

            Some(Instance::Entity(EntityInstance {
                index: ei,
                has_ref,
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
        eprintln!("W: {:?} mesh has no primitivesout_uvs", n.name());
        return None;
    }

    let mut out_pos = vec![];
    let mut out_uvs = vec![];
    let mut out_img = vec![];
    let mut out_frame_ids = vec![];

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

        let extras = mesh.extras();

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
                        let mut out_morph = vec![];
                        for k in 0..base_pos.len() {
                            if let Some(vert) = bb.get_vert_at(base_pos[k] as usize) {
                                let f1 = vert[0] + morphs[indices[k] as usize * 3 + 0];
                                let f2 = vert[1] + morphs[indices[k] as usize * 3 + 1];
                                let f3 = vert[2] + morphs[indices[k] as usize * 3 + 2];

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

        let frame_names = get_morph_target_names(extras);
        let fl = frame_names.len();
        let ol = out_pos.len();
        if fl != ol {
            eprintln!("W: frame name length [{ol}] doesn't match frame count [{ol}]");
        }

        for n in &frame_names {
            out_frame_ids.push(bb.add_frame_name(n));
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
        frame_names: out_frame_ids,
        vertices: out_pos,
        uvs: out_uvs,
        texture: bb.add_image(out_img),
    }))
}

fn get_morph_target_names(extras: &Option<Box<serde_json::value::RawValue>>) -> Vec<String> {
    let mut names = vec!["default".to_string()];

    // Check if the primitive has extras field
    if let Some(e) = extras {
        // Parse the extras as JSON\
        if let Ok(json) = serde_json::from_str::<serde_json::value::Value>(e.get()) {
            if let Some(target_names) = json.get("targetNames") {
                if let Some(names_array) = target_names.as_array() {
                    for name_value in names_array {
                        if let Some(name) = name_value.as_str() {
                            names.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    names
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <file.{{gltf,glb}}>", args[0]);
        return Err(Box::new(MparseError("Malformed or missing arguments")));
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

    let f32_data = bb.get_f32_data();
    let img_data = bb.get_img_data();
    let drn_data = bb.get_drn_data();
    let ern_data = bb.get_ern_data();
    let kvs_data = bb.get_kvs_data();
    let fn_data = bb.get_fn_data();

    let buf = mparse::marshal(
        f32_data,
        img_data,
        drn_data,
        ern_data,
        kvs_data,
        fn_data,
        &map_ref_decs,
        &map_ref_entt,
        &map_ins_decs,
        &map_ins_entt,
    )?;

    if cfg!(debug_assertions) {
        let payload = mparse::unmarshal(&buf)?;

        assert_eq!(&payload.floats, f32_data);
        assert_eq!(&payload.img_data, img_data);
        assert_eq!(&payload.drn_data, drn_data);
        assert_eq!(&payload.ern_data, ern_data);
        assert_eq!(&payload.kvs_data, kvs_data);
        assert_eq!(&payload.fn_data, fn_data);
        assert_eq!(&payload.map_ref_decs, &map_ref_decs);
        assert_eq!(&payload.map_ref_ents, &map_ref_entt);
        assert_eq!(&payload.map_ins_decs, &map_ins_decs);
        assert_eq!(&payload.map_ins_ents, &map_ins_entt);
    }

    std::fs::write("map.mp", buf)?;

    Ok(())
}
