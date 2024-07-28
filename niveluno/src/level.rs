use mparse::types::Payload;

use crate::{
    math,
    nuerror::NUError,
    render::{self, create_texture, PngBin},
};

#[derive(Clone, Debug)]
pub struct LevelPayload {
    pub drn_data: Vec<String>,
    pub ern_data: Vec<String>,
    pub kvs_data: Vec<String>,
    pub fn_data: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Level {
    pub payload: LevelPayload,
    pub img_handles: Vec<usize>,
    pub map_entities: Vec<Entity>,
    pub ref_entities: Vec<LoadedEnttReference>,
    pub map_decor: Vec<Decor>,
    pub ref_decor: Vec<LoadedDecorReference>,
}

impl Level {
    fn spawn_entts() {}
    fn spawn_decor() {}
    fn spawn_entt_from_name() {}
    fn spawn_decor_from_name() {}
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub index: usize,
    pub has_ref: bool,
    pub params: Vec<u32>, // indexes to [k,v,k,v,k,v] etc
    // todo, go back to indices into big float array
    pub location: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Clone, Debug)]
pub struct Decor {
    pub ref_id: usize,
    // todo, go back to indices into big float array
    pub location: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Clone, Debug)]
pub struct LoadedDecorReference {
    pub index: usize,
    pub texture_handle: usize,
    pub frame_handle: usize,
}

#[derive(Clone, Debug)]
pub struct LoadedEnttReference {
    pub index: usize,
    pub texture_handle: usize,
    pub frame_handles: Vec<usize>,
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
            let v0 = v[0].into();
            let v1 = v[1].into();
            let v2 = v[2].into();

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

// todo, move this?
pub fn load_level(payload: Payload) -> Result<Level, NUError> {
    let level_payload = LevelPayload {
        drn_data: payload.drn_data,
        ern_data: payload.ern_data,
        kvs_data: payload.kvs_data,
        fn_data: payload.fn_data,
    };

    // images
    let mut img_handles = vec![];
    for i in &payload.img_data {
        img_handles.push(create_texture(PngBin { data: i.to_vec() })?);
    }

    // decor refs
    let mut ref_decor = vec![];
    for rd in &payload.map_ref_decs {
        let mut verts = vec![];
        for v_index in &rd.vertices {
            verts.push([
                payload.floats[(v_index + 0) as usize],
                payload.floats[(v_index + 1) as usize],
                payload.floats[(v_index + 2) as usize],
            ])
        }
        // eprintln!("verts: {:?}", verts);

        let mut uvs = vec![];
        for u_index in &rd.uvs {
            uvs.push([
                payload.floats[(u_index + 0) as usize],
                payload.floats[(u_index + 1) as usize],
            ])
        }

        // vecs for compatibility with animated models
        let pf = pack_floats(vec![verts], uvs)?;
        if pf.len() != 1 {
            return Err(NUError::MiscError("decor is animated".to_string()));
        }

        ref_decor.push(LoadedDecorReference {
            index: rd.name as usize,
            texture_handle: img_handles[rd.texture as usize],
            frame_handle: pf[0],
        })
    }

    // map instances (decoration)
    let mut map_decor = vec![];
    for di in &payload.map_ins_decs {
        let decor = Decor {
            ref_id: di.index as usize,
            location: [
                payload.floats[(di.location + 0) as usize],
                payload.floats[(di.location + 1) as usize],
                payload.floats[(di.location + 2) as usize],
            ],
            rotation: [
                payload.floats[(di.rotation + 0) as usize],
                payload.floats[(di.rotation + 1) as usize],
                payload.floats[(di.rotation + 2) as usize],
                payload.floats[(di.rotation + 3) as usize],
            ],
            scale: [
                payload.floats[(di.scale + 0) as usize],
                payload.floats[(di.scale + 1) as usize],
                payload.floats[(di.scale + 2) as usize],
            ],
        };
        map_decor.push(decor);
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

        let mut uvs = vec![];
        for u_index in &re.uvs {
            uvs.push([
                payload.floats[(u_index + 0) as usize],
                payload.floats[(u_index + 1) as usize],
            ])
        }

        ref_entts.push(LoadedEnttReference {
            index: re.name as usize,
            texture_handle: img_handles[re.texture as usize],
            frame_handles: pack_floats(verts, uvs)?,
        })
    }

    // map entities
    let mut map_entts = vec![];
    for ei in &payload.map_ins_ents {
        let entity = Entity {
            index: ei.index as usize,
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

    eprintln!("rdl: {}", ref_decor.len());
    eprintln!("mdl: {}", map_decor.len());
    eprintln!("rel: {}", ref_entts.len());
    eprintln!("mel: {}", map_entts.len());

    Ok(Level {
        payload: level_payload,
        // needs to eat copies of payloads _data fields
        // which need to have corresponding lookup functions
        // in this file (level.rs)
        img_handles,
        map_entities: map_entts,
        ref_entities: ref_entts,
        map_decor,
        ref_decor,
    })
}
