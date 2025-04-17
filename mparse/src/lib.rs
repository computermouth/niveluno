
mod types;
use types::external;
use types::internal;

pub mod exports {
    use crate::types;
    pub use types::external::*;
}

use msgpacker::prelude::*;

pub fn marshal(
    version: u32,
    floats: &Vec<f32>,
    img_data: &Vec<Vec<u8>>,
    ern_data: &Vec<String>,
    kvs_data: &Vec<String>,
    frame_data: &Vec<String>,
    map_ref_entt: &Vec<external::EntityReference>,
    map_ins_entt: &Vec<external::EntityInstance>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let p = internal::Payload {
        version,
        floats: floats.clone(),
        img_data: img_data
            .clone()
            .into_iter()
            .map(|i| i.into_boxed_slice())
            .collect(),
        ern_data: ern_data.clone(),
        kvs_data: kvs_data.clone(),
        fn_data: frame_data.clone(),
        map_ref_ents: map_ref_entt
            .clone()
            .into_iter()
            .map(|e| internal::EntityReference {
                name: e.name,
                is_decor: e.is_decor,
                frame_names: e.frame_names,
                texture: e.texture,
                vertices: e.vertices,
                uvs: e.uvs,
            })
            .collect(),
        map_ins_ents: map_ins_entt
            .clone()
            .into_iter()
            .map(|e| internal::EntityInstance {
                index: e.index,
                has_ref: e.has_ref,
                params: e.params,
                location: e.location,
                rotation: e.rotation,
                scale: e.scale,
            })
            .collect(),
    };

    let mut buf = vec![];

    p.pack(&mut buf);

    Ok(buf)
}

pub fn unmarshal(buf: &Vec<u8>) -> Result<external::Payload, Box<dyn std::error::Error>> {
    let (_, t) =
        internal::Payload::unpack(&buf).map_err(|_| external::MparseError("failed to read_str"))?;
    let p = external::Payload {
        version: t.version,
        floats: t.floats,
        img_data: t.img_data,
        ern_data: t.ern_data,
        kvs_data: t.kvs_data,
        fn_data: t.fn_data,
        map_ref_ents: t
            .map_ref_ents
            .into_iter()
            .map(|e| external::EntityReference {
                name: e.name,
                is_decor: e.is_decor,
                frame_names: e.frame_names,
                texture: e.texture,
                vertices: e.vertices,
                uvs: e.uvs,
            })
            .collect(),
        map_ins_ents: t
            .map_ins_ents
            .clone()
            .into_iter()
            .map(|e| external::EntityInstance {
                index: e.index,
                has_ref: e.has_ref,
                params: e.params,
                location: e.location,
                rotation: e.rotation,
                scale: e.scale,
            })
            .collect(),
    };
    Ok(p)
}
