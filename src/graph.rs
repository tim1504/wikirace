use rocksdb::{DB,Options};

pub struct Graph {
    name_by_id: DB,
    id_by_name: DB,
    ingoing_edges_by_id: DB,
    outgoing_edges_by_id: DB,
}

impl Graph {
    pub fn load(path: [&'static str; 4]) -> Graph {
        let mut options = Options::default();
        options.create_if_missing(false); 
        let name_by_id = DB::open_for_read_only(&options, path[0], false).unwrap();
        let id_by_name: DB = DB::open_for_read_only(&options, path[1], false).unwrap();
        let ingoing_edges_by_id = DB::open_for_read_only(&options, path[2], false).unwrap();
        let outgoing_edges_by_id = DB::open_for_read_only(&options, path[3], false).unwrap();
        Graph {
            name_by_id,
            id_by_name,
            ingoing_edges_by_id,
            outgoing_edges_by_id,
        }
    }

    pub fn get_id(&self, name: &str) -> Option<i32> {
        let bytes = self.id_by_name.get(name.as_bytes()).unwrap_or(None);
        if bytes.is_none() {
            return None;
        }
        let bytes = bytes.unwrap();
        if bytes.len() != 4 {
            return None;
        }
        let slice: &[u8] = &bytes;
        let array: &[u8; 4] = slice.try_into().unwrap();
        Some(i32::from_be_bytes(*array))
    }

    pub fn get_name(&self, id: &i32) -> Option<String> {
        let bytes = self
            .name_by_id
            .get(id.to_be_bytes())
            .unwrap_or(None);
        if bytes.is_none() {
            return None;
        }
        Some(String::from_utf8(bytes.unwrap()).unwrap())
    }

    pub fn get_edges(&self, id: &i32, direction: bool) -> Vec<i32> {
        let bytes = match direction {
            true => self
                .ingoing_edges_by_id
                .get(id.to_be_bytes())
                .unwrap_or(None),
            false => self
                .outgoing_edges_by_id
                .get(id.to_be_bytes())
                .unwrap_or(None),
        };
        if bytes.is_none() {
            return Vec::new();
        }
        let bytes = bytes.unwrap();
        if bytes.len() % 4 != 0 {
            return Vec::new();
        }
        let edges: Vec<i32> = bytes
            .chunks_exact(4)
            .map(|chunk| {
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(chunk);
                i32::from_be_bytes(bytes)
            })
            .collect();
        edges
    }
}