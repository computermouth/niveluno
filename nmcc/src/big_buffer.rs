use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum HashItem {
    Uv__([f32; 2]),
    Vert([f32; 3]),
    Quat([f32; 4]),
}

impl PartialEq for HashItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HashItem::Uv__(a), HashItem::Uv__(b)) => a == b,
            (HashItem::Vert(a), HashItem::Vert(b)) => a == b,
            (HashItem::Quat(a), HashItem::Quat(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for HashItem {}

impl Hash for HashItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HashItem::Uv__(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
            HashItem::Vert(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
            HashItem::Quat(arr) => {
                arr.iter().for_each(|&x| {
                    x.to_bits().hash(state);
                });
            }
        }
    }
}

#[derive(Debug)]
pub struct BigBuffer {
    f32_data: Vec<f32>,
    f32_hmap: HashMap<HashItem, usize>,
    img_data: Vec<Vec<u8>>,
    // decor reference name data
    drn_data: Vec<String>,
    drn_hmap: HashMap<String, u32>,
    // entt reference name data
    ern_data: Vec<String>,
    ern_hmap: HashMap<String, u32>,
    // parameter kvs
    kvs_data: Vec<String>,
    kvs_hmap: HashMap<String, u32>,
}

impl BigBuffer {
    pub fn new() -> Self {
        let mut h = BigBuffer {
            f32_data: Vec::new(),
            f32_hmap: HashMap::new(),
            img_data: Vec::new(),
            drn_data: Vec::new(),
            drn_hmap: HashMap::new(),
            ern_data: Vec::new(),
            ern_hmap: HashMap::new(),
            kvs_data: Vec::new(),
            kvs_hmap: HashMap::new(),
        };

        // 3 uvs, 2 vers, 1 quat
        // assuming there's one unit cube in there somewhere,
        // these will be made useful
        let starter = [-0.5, 0.5, 0.5, -0.5];
        h.f32_data.extend_from_slice(&starter);

        h.f32_hmap.insert(HashItem::Quat(starter), 0);
        h.f32_hmap
            .insert(HashItem::Vert([starter[0], starter[1], starter[2]]), 0);
        h.f32_hmap
            .insert(HashItem::Vert([starter[1], starter[2], starter[3]]), 1);
        h.f32_hmap
            .insert(HashItem::Uv__([starter[0], starter[1]]), 0);
        h.f32_hmap
            .insert(HashItem::Uv__([starter[1], starter[2]]), 1);
        h.f32_hmap
            .insert(HashItem::Uv__([starter[2], starter[3]]), 2);

        h
    }

    pub fn add_image(&mut self, image: Vec<u8>) -> u32 {
        if self.img_data.len() == 0 {
            self.img_data.push(image);
            return 0;
        }

        for (i, tmp_img) in self.img_data.iter().enumerate() {
            if tmp_img == &image {
                return i as u32;
            }
        }

        self.img_data.push(image);
        (self.img_data.len() - 1) as u32
    }

    pub fn add_sequence(&mut self, sequence: HashItem) -> u32 {
        if let Some(&index) = self.f32_hmap.get(&sequence) {
            return index as u32;
        }

        let len = self.f32_data.len();
        let mut payload = self.f32_data[(len - 3)..len].to_vec();

        // it's not in there, lets append and return the slice
        let iterations = match sequence {
            HashItem::Uv__(arr) => {
                let plen = payload.len();
                // [-1a, 1b]
                let one_new = [payload[plen - 1], arr[1]];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ => 2,
                };
                self.f32_data.extend_from_slice(&arr[arr.len() - iter..]);
                payload.extend_from_slice(&arr[arr.len() - iter..]);
                iter
            }
            HashItem::Vert(arr) => {
                let plen = payload.len();
                // [-1a, 1b, 2b]
                let two_new = [payload[plen - 1], arr[1], arr[2]];
                // [-2a, -1a, 2b]
                let one_new = [payload[plen - 2], payload[plen - 1], arr[2]];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ if arr == two_new => 2,
                    _ => 3,
                };
                self.f32_data.extend_from_slice(&arr[arr.len() - iter..]);
                payload.extend_from_slice(&arr[arr.len() - iter..]);
                iter
            }
            HashItem::Quat(arr) => {
                let plen = payload.len();
                // [-1a, 1b, 2b, 3b]
                let thr_new = [payload[plen - 1], arr[1], arr[2], arr[3]];
                // [-2a, -1a, 2b, 3b]
                let two_new = [payload[plen - 2], payload[plen - 1], arr[2], arr[3]];
                // [-3a, -2a, -1a, 3b]
                let one_new = [
                    payload[plen - 3],
                    payload[plen - 2],
                    payload[plen - 1],
                    arr[3],
                ];
                let iter = match &arr {
                    _ if arr == one_new => 1,
                    _ if arr == two_new => 2,
                    _ if arr == thr_new => 3,
                    _ => 4,
                };
                self.f32_data.extend_from_slice(&arr[arr.len() - iter..]);
                payload.extend_from_slice(&arr[arr.len() - iter..]);
                iter
            }
        };

        let plen = payload.len();
        let dlen = self.f32_data.len();

        for i in 0..iterations {
            let tmp_quat = HashItem::Quat([
                payload[plen - i - 4],
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.f32_hmap.get(&tmp_quat).is_none() {
                self.f32_hmap.insert(tmp_quat, dlen - i - 4);
            }
            let tmp_vert = HashItem::Vert([
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.f32_hmap.get(&tmp_vert).is_none() {
                self.f32_hmap.insert(tmp_vert, dlen - i - 3);
            }
            let tmp_uv__ = HashItem::Uv__([payload[plen - i - 2], payload[plen - i - 1]]);
            if self.f32_hmap.get(&tmp_uv__).is_none() {
                self.f32_hmap.insert(tmp_uv__, dlen - i - 2);
            }
        }

        if let Some(&index) = self.f32_hmap.get(&sequence) {
            index as u32
        } else {
            panic!("couldn't find last inserted index");
        }
    }

    pub fn add_decor_name(&mut self, s: &str) -> Result<u32, ()> {
        if let Some(_) = self.drn_hmap.get(s) {
            return Err(());
        }

        let len = self.drn_data.len();
        self.drn_hmap.insert(s.to_string(), len as u32);
        self.drn_data.push(s.to_string());

        Ok(len as u32)
    }

    pub fn get_decor_index(&mut self, s: &str) -> Option<&u32> {
        self.drn_hmap.get(s)
    }

    pub fn add_entt_name(&mut self, s: &str) -> Result<u32, ()> {
        if let Some(_) = self.ern_hmap.get(s) {
            return Err(());
        }

        let len = self.ern_data.len();
        self.ern_hmap.insert(s.to_string(), len as u32);
        self.ern_data.push(s.to_string());

        Ok(len as u32)
    }

    pub fn get_entt_index(&mut self, s: &str) -> Option<&u32> {
        self.ern_hmap.get(s)
    }

    pub fn add_kv_string(&mut self, s: &str) -> u32 {
        if let Some(&index) = self.kvs_hmap.get(s) {
            return index as u32;
        }

        let len = self.kvs_data.len();
        self.kvs_hmap.insert(s.to_string(), len as u32);
        self.kvs_data.push(s.to_string());

        len as u32
    }

    pub fn get_vert_at(&self, i: usize) -> Option<[f32; 3]> {
        if self.f32_data.len() < i + 3 {
            return None;
        }
        Some([
            self.f32_data[i + 0],
            self.f32_data[i + 1],
            self.f32_data[i + 2],
        ])
    }

    pub fn print_data(&self) {
        eprintln!("data[{}]: {:?}", self.f32_data.len(), self.f32_data);
    }

    pub fn print_map(&self) {
        eprintln!("map: {:?}", self.f32_hmap);
    }

    // #[derive(Debug)]
    // pub struct BigBuffer {
    //     f32_data: Vec<f32>,
    //     f32_hmap: HashMap<HashItem, usize>,
    //     img_data: Vec<Vec<u8>>,
    //     // decor reference name data
    //     drn_data: Vec<String>,
    //     drn_hmap: HashMap<String, u32>,
    //     // entt reference name data
    //     ern_data: Vec<String>,
    //     ern_hmap: HashMap<String, u32>,
    //     // parameter kvs
    //     kvs_data: Vec<String>,
    //     kvs_hmap: HashMap<String, u32>
    // }

    pub fn get_f32_data(&self) -> &Vec<f32> {
        &self.f32_data
    }

    pub fn get_img_data(&self) -> &Vec<Vec<u8>> {
        &self.img_data
    }

    pub fn get_drn_data(&self) -> &Vec<String> {
        &self.drn_data
    }

    pub fn get_ern_data(&self) -> &Vec<String> {
        &self.ern_data
    }

    pub fn get_kvs_data(&self) -> &Vec<String> {
        &self.kvs_data
    }
}
