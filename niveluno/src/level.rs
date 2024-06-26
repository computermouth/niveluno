use mparse::types::Payload;

use crate::{
    math,
    nuerror::NUError,
    render::{self, create_texture, PngBin},
};

pub struct Level {
    pub img_handles: Vec<usize>,
    pub map_entities: u32,
    pub ref_entities: u32,
    pub map_decor: u32,
    pub ref_decor: Vec<LoadedDecorReference>,
}

struct Entity {}

struct Decor {
    // index to decor reference??
    index: usize,
    // todo, go back to indices into big float array
    pub location: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

enum MapInstance {
    Entity(Entity),
    Decor(Decor),
}

pub struct LoadedDecorReference {
    // index to decor reference??
    pub index: usize,
    pub texture_handle: usize,
    pub frame_handle: usize,
}

enum LoadedReference {
    Decor(LoadedDecorReference),
}

struct FloatPackInput {
    verts: Vec<Vec<[f32; 3]>>,
    uvs: Vec<Vec<[f32; 2]>>,
}

fn pack_floats(verts: Vec<Vec<[f32; 3]>>, uvs: Vec<Vec<[f32; 2]>>) -> Result<Vec<usize>, NUError> {
    if verts.len() != uvs.len() {
        return Err(NUError::MiscError(
            "vert len differs from uv len".to_string(),
        ));
    }

    let mut frame_handles = vec![];

    let frame_count = verts.len();
    for frame in 0..frame_count {
        frame_handles.push(render::get_r_num_verts()?);
        for (v, u) in verts[frame].chunks(3).zip(uvs[frame].chunks(3)) {
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
pub fn load_level(payload: &Payload) -> Result<Level, NUError> {
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

        let mut uvs = vec![];
        for u_index in &rd.uvs {
            uvs.push([
                payload.floats[(u_index + 0) as usize],
                payload.floats[(u_index + 1) as usize],
            ])
        }

        // vecs for compatibility with animated models
        let pf = pack_floats(vec![verts], vec![uvs])?;
        if pf.len() != 1 {
            return Err(NUError::MiscError("decor is animated".to_string()));
        }

        ref_decor.push(LoadedDecorReference {
            index: rd.name as usize,
            texture_handle: img_handles[rd.texture as usize],
            frame_handle: pf[0],
        })
    }

    // // map instances (decoration)
    // let mut map_decor = vec![];
    // for di in &payload.map_ins_decs {
    //     let decor = Decor {
    //         name: di.index,
    //         location: [
    //             payload.floats[(di.location + 0) as usize],
    //             payload.floats[(di.location + 1) as usize],
    //             payload.floats[(di.location + 2) as usize],
    //         ],
    //         rotation: [
    //             payload.floats[(di.rotation + 0) as usize],
    //             payload.floats[(di.rotation + 1) as usize],
    //             payload.floats[(di.rotation + 2) as usize],
    //             payload.floats[(di.rotation + 3) as usize],
    //         ],
    //         scale: [
    //             payload.floats[(di.scale + 0) as usize],
    //             payload.floats[(di.scale + 1) as usize],
    //             payload.floats[(di.scale + 2) as usize],
    //         ],
    //     };
    // }

    eprintln!("rdl: {}", ref_decor.len());

    Ok(Level {
        // needs to eat copies of payloads _data fields
        // which need to have corresponding lookup functions
        // in this file (level.rs)
        img_handles,
        map_entities: 0,
        ref_entities: 0,
        map_decor: 0,
        ref_decor,
    })
}
