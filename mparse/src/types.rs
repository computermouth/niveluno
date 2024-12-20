#[derive(Debug)]
pub struct MparseError<'a>(pub &'a str);
impl<'a> std::fmt::Display for MparseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MparseError: {:?}", self.0)
    }
}
impl<'a> std::error::Error for MparseError<'a> {}

#[derive(Debug, PartialEq)]
pub struct EntityReference {
    pub name: u32,
    pub is_decor: bool,
    pub frame_names: Vec<u32>,
    pub texture: u32,
    pub vertices: Vec<Vec<u32>>,
    pub uvs: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub struct EntityInstance {
    pub index: u32,
    // if has_ref, follow index into references
    pub has_ref: bool,
    pub params: Vec<u32>, // indexes to [k,v,k,v,k,v] etc
    pub location: u32,    // u32 -> [f32;3]
    pub rotation: u32,    // u32 -> [f32;4]
    pub scale: u32,       // u32 -> [f32;3]
}

#[derive(Debug, PartialEq)]
pub struct DecorInstance {
    pub index: u32,
    pub location: u32, // u32 -> [f32;3]
    pub rotation: u32, // u32 -> [f32;4]
    pub scale: u32,    // u32 -> [f32;3]
}

#[derive(Debug)]
pub struct Payload {
    pub floats: Vec<f32>,
    pub img_data: Vec<Vec<u8>>,
    pub ern_data: Vec<String>,
    pub kvs_data: Vec<String>,
    pub fn_data: Vec<String>,
    pub map_ref_ents: Vec<EntityReference>,
    pub map_ins_ents: Vec<EntityInstance>,
}
