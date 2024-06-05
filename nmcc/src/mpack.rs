use std::io::Cursor;
use std::io::Read;

use crate::types::*;
use rmp::decode;
use rmp::encode;

pub fn marshal(
    floats: &Vec<f32>,
    img_data: &Vec<Vec<u8>>,
    drn_data: &Vec<String>,
    ern_data: &Vec<String>,
    kvs_data: &Vec<String>,
    map_ref_decs: &Vec<DecorReference>,
    map_ref_entt: &Vec<EntityReference>,
    map_ins_decs: &Vec<DecorInstance>,
    map_ins_entt: &Vec<EntityInstance>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = vec![];

    // top-level array
    encode::write_array_len(&mut buf, 10)?;

    // version
    encode::write_u32(&mut buf, 0)?;

    {
        // floats
        encode::write_array_len(&mut buf, floats.len() as u32)?;
        for f in floats {
            encode::write_f32(&mut buf, *f)?;
        }
    }

    {
        // img_data
        encode::write_array_len(&mut buf, img_data.len() as u32)?;
        for img in img_data {
            encode::write_bin(&mut buf, img)?;
        }
    }

    {
        // drn_data
        encode::write_array_len(&mut buf, drn_data.len() as u32)?;
        for drn in drn_data {
            encode::write_str_len(&mut buf, (drn.len() + 1) as u32)?;
            encode::write_str(&mut buf, drn)?;
        }
    }

    {
        // ern_data
        encode::write_array_len(&mut buf, ern_data.len() as u32)?;
        for ern in ern_data {
            encode::write_str_len(&mut buf, (ern.len() + 1) as u32)?;
            encode::write_str(&mut buf, ern)?;
        }
    }

    {
        // kvs_data
        encode::write_array_len(&mut buf, kvs_data.len() as u32)?;
        for kvs in kvs_data {
            encode::write_str_len(&mut buf, (kvs.len() + 1) as u32)?;
            encode::write_str(&mut buf, kvs)?;
        }
    }

    {
        // map_ref_decs
        encode::write_array_len(&mut buf, map_ref_decs.len() as u32)?;
        for dec in map_ref_decs {
            encode::write_array_len(&mut buf, 4)?;
            encode::write_u32(&mut buf, dec.name)?;
            encode::write_u32(&mut buf, dec.texture)?;
            {
                // verts
                encode::write_array_len(&mut buf, dec.vertices.len() as u32)?;
                for i in &dec.vertices {
                    encode::write_f32(&mut buf, *i)?;
                }
            }
            {
                // uvs
                encode::write_array_len(&mut buf, dec.uvs.len() as u32)?;
                for i in &dec.uvs {
                    encode::write_f32(&mut buf, *i)?;
                }
            }
        }
    }

    {
        // map_ref_entt
        encode::write_array_len(&mut buf, map_ref_entt.len() as u32)?;
        for entt in map_ref_entt {
            encode::write_array_len(&mut buf, 4)?;
            encode::write_u32(&mut buf, entt.name)?;
            encode::write_u32(&mut buf, entt.texture)?;
            {
                // verts
                encode::write_array_len(&mut buf, entt.vertices.len() as u32)?;
                for frame in &entt.vertices {
                    encode::write_array_len(&mut buf, frame.len() as u32)?;
                    for i in frame {
                        encode::write_f32(&mut buf, *i)?;
                    }
                }
            }
            {
                // uvs
                encode::write_array_len(&mut buf, entt.uvs.len() as u32)?;
                for i in &entt.uvs {
                    encode::write_f32(&mut buf, *i)?;
                }
            }
        }
    }

    {
        // map_ins_decs
        encode::write_array_len(&mut buf, map_ins_decs.len() as u32)?;
        for dec in map_ins_decs {
            encode::write_array_len(&mut buf, 11)?;
            encode::write_u32(&mut buf, dec.index)?;
            encode::write_f32(&mut buf, dec.location[0])?;
            encode::write_f32(&mut buf, dec.location[1])?;
            encode::write_f32(&mut buf, dec.location[2])?;
            encode::write_f32(&mut buf, dec.rotation[0])?;
            encode::write_f32(&mut buf, dec.rotation[1])?;
            encode::write_f32(&mut buf, dec.rotation[2])?;
            encode::write_f32(&mut buf, dec.rotation[3])?;
            encode::write_f32(&mut buf, dec.scale[0])?;
            encode::write_f32(&mut buf, dec.scale[1])?;
            encode::write_f32(&mut buf, dec.scale[2])?;
        }
    }

    {
        // map_ins_entt
        encode::write_array_len(&mut buf, map_ins_entt.len() as u32)?;
        for entt in map_ins_entt {
            encode::write_array_len(&mut buf, 12)?;
            match entt.index {
                Some(i) => encode::write_u32(&mut buf, i)?,
                None => encode::write_nil(&mut buf)?,
            }
            encode::write_array_len(&mut buf, entt.params.len() as u32)?;
            for i in &entt.params {
                encode::write_u32(&mut buf, *i)?;
            }
            encode::write_f32(&mut buf, entt.location[0])?;
            encode::write_f32(&mut buf, entt.location[1])?;
            encode::write_f32(&mut buf, entt.location[2])?;
            encode::write_f32(&mut buf, entt.rotation[0])?;
            encode::write_f32(&mut buf, entt.rotation[1])?;
            encode::write_f32(&mut buf, entt.rotation[2])?;
            encode::write_f32(&mut buf, entt.rotation[3])?;
            encode::write_f32(&mut buf, entt.scale[0])?;
            encode::write_f32(&mut buf, entt.scale[1])?;
            encode::write_f32(&mut buf, entt.scale[2])?;
        }
    }

    Ok(buf)
}

fn read_u32_from_marker(cur: &mut Cursor<&Vec<u8>>) -> Result<u32, Box<dyn std::error::Error>> {
    let pos = cur.position();
    let marker = decode::read_marker(cur).map_err(|_| NmccError("failed to read_marker"))?;
    cur.set_position(pos);

    match marker {
        rmp::Marker::FixPos(val) => Ok(val as u32),
        rmp::Marker::U8 => Ok(decode::read_u8(cur)? as u32),
        rmp::Marker::U16 => Ok(decode::read_u16(cur)? as u32),
        rmp::Marker::U32 => Ok(decode::read_u32(cur)?),
        marker => {
            eprintln!("Unexpected marker: {:?}", marker);
            Err(Box::new(NmccError("unexpected marker for u32 value")))
        }
    }
}

pub fn unmarshal(
    buf: &Vec<u8>,
) -> Result<
    (
        Vec<f32>,             // floats
        Vec<Vec<u8>>,         // img_data
        Vec<String>,          // drn_data
        Vec<String>,          // ern_data
        Vec<String>,          // kvs_data
        Vec<DecorReference>,  // map_ref_decs
        Vec<EntityReference>, // map_ref_entts
        Vec<DecorInstance>,   // map_ins_decs
        Vec<EntityInstance>,  // map_ins_entts
    ),
    Box<dyn std::error::Error>,
> {
    let mut floats = vec![];
    let mut img_data = vec![];
    let mut drn_data = vec![];
    let mut ern_data = vec![];
    let mut kvs_data = vec![];
    let mut map_ref_decs = vec![];
    let mut map_ref_entts = vec![];
    let mut map_ins_decs = vec![];
    let mut map_ins_entts = vec![];

    let mut cur = Cursor::new(buf);

    // top-level array
    let len = decode::read_array_len(&mut cur)?;
    assert_eq!(10, len);

    // version
    let version = read_u32_from_marker(&mut cur)?;
    assert_eq!(0, version);

    {
        // floats
        let flen = decode::read_array_len(&mut cur)?;
        for _ in 0..flen {
            floats.push(decode::read_f32(&mut cur)?);
        }
    }

    {
        // img_data
        let ilen = decode::read_array_len(&mut cur)?;
        for _ in 0..ilen {
            let blen = decode::read_bin_len(&mut cur)?;
            let mut img = vec![0u8; blen as usize];
            cur.read_exact(&mut img)?;
            img_data.push(img);
        }
    }

    {
        // drn_data
        let drn_len = decode::read_array_len(&mut cur)?;
        for _ in 0..drn_len {
            let slen = decode::read_str_len(&mut cur)?;
            // remove the null terminator
            let mut drn_str = vec![0u8; (slen - 1) as usize];
            decode::read_str(&mut cur, &mut drn_str)
                .map_err(|_| NmccError("failed to read_str"))?;
            drn_data.push(std::str::from_utf8(&drn_str)?.to_string());
        }
    }

    {
        // ern_data
        let ern_len = decode::read_array_len(&mut cur)?;
        for _ in 0..ern_len {
            let slen = decode::read_str_len(&mut cur)?;
            // remove the null terminator
            let mut ern_str = vec![0u8; (slen - 1) as usize];
            decode::read_str(&mut cur, &mut ern_str)
                .map_err(|_| NmccError("failed to read_str"))?;
            ern_data.push(std::str::from_utf8(&ern_str)?.to_string());
        }
    }

    {
        // kvs_data
        let kvs_len = decode::read_array_len(&mut cur)?;
        for _ in 0..kvs_len {
            let slen = decode::read_str_len(&mut cur)?;
            // remove the null terminator
            let mut kvs_str = vec![0u8; (slen - 1) as usize];
            decode::read_str(&mut cur, &mut kvs_str)
                .map_err(|_| NmccError("failed to read_str"))?;
            kvs_data.push(std::str::from_utf8(&kvs_str)?.to_string());
        }
    }

    {
        // map_ref_decs
        let mrd_len = decode::read_array_len(&mut cur)?;
        for _ in 0..mrd_len {
            assert_eq!(4, decode::read_array_len(&mut cur)?);
            let name = read_u32_from_marker(&mut cur)?;
            let txtr = read_u32_from_marker(&mut cur)?;
            let mut verts = vec![];
            let mut uvs = vec![];
            {
                // verts
                let vert_len = decode::read_array_len(&mut cur)?;
                for _ in 0..vert_len {
                    verts.push(decode::read_f32(&mut cur)?);
                }
            }
            {
                // uvs
                let uv_len = decode::read_array_len(&mut cur)?;
                for _ in 0..uv_len {
                    uvs.push(decode::read_f32(&mut cur)?);
                }
            }
            map_ref_decs.push(DecorReference {
                name,
                texture: txtr,
                vertices: verts,
                uvs,
            })
        }
    }

    {
        // map_ref_entt
        let mre_len = decode::read_array_len(&mut cur)?;
        for _ in 0..mre_len {
            assert_eq!(4, decode::read_array_len(&mut cur)?);
            let name = read_u32_from_marker(&mut cur)?;
            let texture = read_u32_from_marker(&mut cur)?;
            let mut vertices = vec![];
            let mut uvs = vec![];
            {
                // verts
                let framecount = decode::read_array_len(&mut cur)?;
                for _ in 0..framecount {
                    let vertcount = decode::read_array_len(&mut cur)?;
                    let mut v = vec![];
                    for _ in 0..vertcount {
                        v.push(decode::read_f32(&mut cur)?);
                    }
                    vertices.push(v);
                }
            }
            {
                // uvs
                let uv_len = decode::read_array_len(&mut cur)?;
                for _ in 0..uv_len {
                    uvs.push(decode::read_f32(&mut cur)?);
                }
            }
            map_ref_entts.push(EntityReference {
                name,
                texture,
                vertices,
                uvs,
            })
        }
    }

    {
        // map_ins_decs
        let dec_len = decode::read_array_len(&mut cur)?;
        for _ in 0..dec_len {
            assert_eq!(11, decode::read_array_len(&mut cur)?);

            map_ins_decs.push(DecorInstance {
                index: read_u32_from_marker(&mut cur)?,
                location: [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?],
                rotation: [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?],
                scale: [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?],
            });
        }
    }

    {
        // map_ins_entt
        let entt_len = decode::read_array_len(&mut cur)?;
        for _ in 0..entt_len {
            assert_eq!(12, decode::read_array_len(&mut cur)?);
            let pos = cur.position();
            let index = match decode::read_marker(&mut cur)
                .map_err(|_| NmccError("failed to read index marker"))?
            {
                rmp::Marker::Null => {
                    cur.set_position(pos);
                    decode::read_nil(&mut cur)?;
                    None
                }
                _ => {
                    cur.set_position(pos);
                    Some(read_u32_from_marker(&mut cur)?)
                }
            };
            let plen = decode::read_array_len(&mut cur)?;
            let mut params = vec![];
            for _ in 0..plen {
                params.push(read_u32_from_marker(&mut cur)?);
            }
            let location = [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?];
            let rotation = [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?];
            let scale = [decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?,decode::read_f32(&mut cur)?];
            map_ins_entts.push(EntityInstance {
                index,
                params,
                location,
                rotation,
                scale,
            });
        }
    }

    Ok((
        floats,
        img_data,
        drn_data,
        ern_data,
        kvs_data,
        map_ref_decs,
        map_ref_entts,
        map_ins_decs,
        map_ins_entts,
    ))
}
