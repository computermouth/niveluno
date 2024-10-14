use mparse::types::Payload;

use crate::{
    math::{self, Vector3},
    nuerror::NUError,
    render::{self, create_texture, PngBin},
};

#[derive(Clone, Debug)]
pub struct MapPayload {
    pub ern_data: Vec<String>,
    pub kvs_data: Vec<String>,
    pub fn_data: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub payload: MapPayload,
    pub map_entities: Vec<Entity>,
    pub ref_entities: Vec<LoadedEnttReference>,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub ref_id: usize,
    pub has_ref: bool,
    pub params: Vec<u32>, // indexes to [k,v,k,v,k,v] etc
    // todo, go back to indices into big float array
    pub location: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Clone, Debug)]
pub struct LoadedEnttReference {
    pub index: usize,
    pub texture_handle: usize,
    pub frame_names: Vec<String>,
    pub frame_handles: Vec<usize>,
    pub mesh: Vec<[Vector3; 3]>,
    pub num_verts: usize,
}

fn pack_floats(verts: Vec<Vec<[f32; 3]>>, uvs: Vec<[f32; 2]>) -> Result<Vec<usize>, NUError> {
    for v in &verts {
        if v.len() != uvs.len() {
            return Err(NUError::MiscError(
                "vert len differs from uv len".to_string(),
            ));
        }
    }

    let mut frame_handles = vec![];

    let frame_count = verts.len();
    for frame in 0..frame_count {
        frame_handles.push(render::get_r_num_verts()?);
        for (v, u) in verts[frame].chunks(3).zip(uvs.chunks(3)) {
            // todo -- are these necessary?
            // looks like we're already flipping x in nmcc
            let v0 = Vector3 {
                x: v[0][0],
                y: v[0][1],
                z: v[0][2],
            };
            let v1 = Vector3 {
                x: v[1][0],
                y: v[1][1],
                z: v[1][2],
            };
            let v2 = Vector3 {
                x: v[2][0],
                y: v[2][1],
                z: v[2][2],
            };

            let u0 = u[0];
            let u1 = u[1];
            let u2 = u[2];

            let n = math::vec3_face_normal(v0, v1, v2);
            render::push_vert(v0, n, u0[0], u0[1])?;
            render::push_vert(v1, n, u1[0], u1[1])?;
            render::push_vert(v2, n, u2[0], u2[1])?;
        }
    }

    if frame_handles.len() == 0 {
        return Err(NUError::MiscError("frame handle length was 0".to_string()));
    }
    Ok(frame_handles)
}

pub fn load(payload: Payload) -> Result<Map, NUError> {
    eprintln!("entts: {:?}", payload.ern_data);
    eprintln!("keyvs: {:?}", payload.kvs_data);
    eprintln!("frams: {:?}", payload.fn_data);

    let level_payload = MapPayload {
        ern_data: payload.ern_data,
        kvs_data: payload.kvs_data,
        fn_data: payload.fn_data,
    };

    // images
    let mut img_handles = vec![];
    for i in &payload.img_data {
        img_handles.push(create_texture(PngBin { data: i.to_vec() })?);
    }

    // entt refs
    let mut ref_entts = vec![];
    for re in &payload.map_ref_ents {
        let mut verts = vec![];
        for frame in &re.vertices {
            let mut frame_verts = vec![];
            for v_index in frame {
                frame_verts.push([
                    payload.floats[(v_index + 0) as usize],
                    payload.floats[(v_index + 1) as usize],
                    payload.floats[(v_index + 2) as usize],
                ])
            }
            verts.push(frame_verts);
        }
        let vlen = verts[0].len();

        let mut uvs = vec![];
        for u_index in &re.uvs {
            uvs.push([
                payload.floats[(u_index + 0) as usize],
                payload.floats[(u_index + 1) as usize],
            ])
        }

        // mesh collisions not allowed for entities,
        // as they may be animated
        let mut mesh_verts = vec![];
        if re.is_decor {
            let n_verts = verts[0].len() / 3;
            for i in 0..n_verts {
                let v1: Vector3 = verts[0][i * 3 + 0].into();
                let v2: Vector3 = verts[0][i * 3 + 1].into();
                let v3: Vector3 = verts[0][i * 3 + 2].into();

                mesh_verts.push([v1, v2, v3]);
            }
        }

        let mut frame_names = vec![];
        for name in &re.frame_names {
            frame_names.push(level_payload.fn_data[*name as usize].clone());
        }

        ref_entts.push(LoadedEnttReference {
            index: re.name as usize,
            texture_handle: img_handles[re.texture as usize],
            frame_names,
            frame_handles: pack_floats(verts, uvs)?,
            num_verts: vlen,
            mesh: mesh_verts,
        })
    }

    // map entities
    let mut map_entts = vec![];
    for ei in &payload.map_ins_ents {
        let entity = Entity {
            ref_id: ei.index as usize,
            has_ref: ei.has_ref,
            params: ei.params.clone(),
            location: [
                payload.floats[(ei.location + 0) as usize],
                payload.floats[(ei.location + 1) as usize],
                payload.floats[(ei.location + 2) as usize],
            ],
            rotation: [
                payload.floats[(ei.rotation + 0) as usize],
                payload.floats[(ei.rotation + 1) as usize],
                payload.floats[(ei.rotation + 2) as usize],
                payload.floats[(ei.rotation + 3) as usize],
            ],
            scale: [
                payload.floats[(ei.scale + 0) as usize],
                payload.floats[(ei.scale + 1) as usize],
                payload.floats[(ei.scale + 2) as usize],
            ],
        };
        map_entts.push(entity);
    }

    Ok(Map {
        payload: level_payload,
        // needs to eat copies of payloads _data fields
        // which need to have corresponding lookup functions
        // in this file (level.rs)
        map_entities: map_entts,
        ref_entities: ref_entts,
    })
}
