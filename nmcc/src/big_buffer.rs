use std::collections::HashMap;

#[derive(Debug)]
pub struct BigBuffer {
    f32_data: Vec<f32>,
    f32_hmap: HashMap<u32, u32>,
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
        BigBuffer {
            f32_data: Vec::new(),
            f32_hmap: HashMap::new(),
            img_data: Vec::new(),
            drn_data: Vec::new(),
            drn_hmap: HashMap::new(),
            ern_data: Vec::new(),
            ern_hmap: HashMap::new(),
            kvs_data: Vec::new(),
            kvs_hmap: HashMap::new(),
        }
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

    pub fn add_f32(&mut self, f: f32) -> u32 {

        let hashable = f.to_bits();
        if let Some(&index) = self.f32_hmap.get(&hashable) {
            return index as u32;
        }

        let len = self.f32_data.len() as u32;
        self.f32_hmap.insert(hashable, len);
        self.f32_data.push(f);

        len
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

    pub fn get_f32(&self, i: usize) -> Option<f32> {
        if self.f32_data.len() > i {
            return Some(self.f32_data[i]);
        }
        None
    }

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
