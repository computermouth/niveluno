#[derive(Debug)]
pub struct NmccError<'a>(pub &'a str);
impl<'a> std::fmt::Display for NmccError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NmccError: {:?}", self.0)
    }
}
impl<'a> std::error::Error for NmccError<'a> {}

#[derive(Debug, PartialEq)]
pub struct DecorReference {
    pub name: u32,
    pub texture: u32,
    pub vertices: Vec<f32>,
    pub uvs: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct EntityReference {
    pub name: u32,
    pub texture: u32,
    pub vertices: Vec<Vec<f32>>,
    pub uvs: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct EntityInstance {
    // names[index] == player, but also reference[index] == __nomodel
    // names[index] == ogre, but also reference[index] == ${ogre_reference}
    pub index: Option<u32>,
    pub params: Vec<u32>, // indexes to [k,v,k,v,k,v] etc
    pub location: [f32;3],
    pub rotation: [f32;4],
    pub scale: [f32;3],
}

#[derive(Debug, PartialEq)]
pub struct DecorInstance {
    pub index: u32,
    pub location: [f32;3],
    pub rotation: [f32;4],
    pub scale: [f32;3],
}
