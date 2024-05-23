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

// Your existing implementation
#[derive(Debug)]
pub struct BigBuffer {
    data: Vec<f32>,
    img_data: Vec<Vec<u8>>,
    map: HashMap<HashItem, usize>,
}

impl BigBuffer {
    pub fn new() -> Self {
        let mut h = BigBuffer {
            data: Vec::new(),
            img_data: Vec::new(),
            map: HashMap::new(),
        };

        // 3 uvs, 2 vers, 1 quat
        // assuming there's one unit cube in there somewhere,
        // these will be made useful
        let starter = [-0.5, 0.5, 0.5, -0.5];
        h.data.extend_from_slice(&starter);

        h.map.insert(HashItem::Quat(starter), 0);
        h.map
            .insert(HashItem::Vert([starter[0], starter[1], starter[2]]), 0);
        h.map
            .insert(HashItem::Vert([starter[1], starter[2], starter[3]]), 1);
        h.map.insert(HashItem::Uv__([starter[0], starter[1]]), 0);
        h.map.insert(HashItem::Uv__([starter[1], starter[2]]), 1);
        h.map.insert(HashItem::Uv__([starter[2], starter[3]]), 2);

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
        if let Some(&index) = self.map.get(&sequence) {
            return index as u32;
        }

        let len = self.data.len();
        let mut payload = self.data[(len - 3)..len].to_vec();

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
                self.data.extend_from_slice(&arr[arr.len() - iter..]);
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
                self.data.extend_from_slice(&arr[arr.len() - iter..]);
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
                self.data.extend_from_slice(&arr[arr.len() - iter..]);
                payload.extend_from_slice(&arr[arr.len() - iter..]);
                iter
            }
        };

        let plen = payload.len();
        let dlen = self.data.len();

        for i in 0..iterations {
            let tmp_quat = HashItem::Quat([
                payload[plen - i - 4],
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.map.get(&tmp_quat).is_none() {
                self.map.insert(tmp_quat, dlen - i - 4);
            }
            let tmp_vert = HashItem::Vert([
                payload[plen - i - 3],
                payload[plen - i - 2],
                payload[plen - i - 1],
            ]);
            if self.map.get(&tmp_vert).is_none() {
                self.map.insert(tmp_vert, dlen - i - 3);
            }
            let tmp_uv__ = HashItem::Uv__([payload[plen - i - 2], payload[plen - i - 1]]);
            if self.map.get(&tmp_uv__).is_none() {
                self.map.insert(tmp_uv__, dlen - i - 2);
            }
        }

        if let Some(&index) = self.map.get(&sequence) {
            index as u32
        } else {
            panic!("couldn't find last inserted index");
        }
    }

    pub fn print_data(&self) {
        eprintln!("data[{}]: {:?}", self.data.len(), self.data);
    }

    pub fn print_map(&self) {
        eprintln!("map: {:?}", self.map);
    }
}
