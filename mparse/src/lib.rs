pub mod types;
use types::*;

use msgpacker::prelude::*;

pub fn marshal(
    floats: &Vec<f32>,
    img_data: &Vec<Vec<u8>>,
    ern_data: &Vec<String>,
    kvs_data: &Vec<String>,
    frame_data: &Vec<String>,
    map_ref_entt: &Vec<EntityReference>,
    map_ins_entt: &Vec<EntityInstance>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let p = MpPayload {
        version: 0,
        floats: floats.clone(),
        img_data: img_data.clone().into_iter().map(|i| i.into_boxed_slice()).collect(),
        ern_data: ern_data.clone(),
        kvs_data: kvs_data.clone(),
        fn_data: frame_data.clone(),
        map_ref_ents: map_ref_entt.clone(),
        map_ins_ents: map_ins_entt.clone(),
    };

    let mut buf = vec![];

    p.pack(&mut buf);

    Ok(buf)
}

pub fn unmarshal(buf: &Vec<u8>) -> Result<Payload, Box<dyn std::error::Error>> {
    let (_, t) = MpPayload::unpack(&buf).map_err(|_| MparseError("failed to read_str"))?;
    assert_eq!(t.version, 0);
    Ok(
        Payload {
            floats: t.floats,
            img_data: t.img_data.into_iter().map(|i| i.into_vec()).collect(),
            ern_data: t.ern_data,
            kvs_data: t.kvs_data,
            fn_data: t.fn_data,
            map_ref_ents: t.map_ref_ents,
            map_ins_ents: t.map_ins_ents
        }
    )
}
